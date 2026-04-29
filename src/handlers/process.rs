use crate::common::AppState;
use crate::models::{
    Host, ProcessActionRequest, ProcessActionResponse, ProcessConfigRequest, ProcessConfigResponse,
    ProcessRequest, ProcessResponse, UserWithPermissions,
};
use crate::schema;
use crate::supervisor::{ProcessInfo, Server};
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use diesel::QueryDsl;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use tokio::task::JoinSet;

/**

/process        -> GET ? group_id=1 & host_id=1 & process_name=foo
/process/config -> GET ? host_id=1 & process_name=foo
/process/stdout -> GET ? host_id=1 & process_name=foo offset=1 length=213
/process/stderr -> GET ? host_id=1 & process_name=foo offset=1 length=213
/process/start  -> POST  [ {host_id=1 process_name=foo } ]
/process/stop   -> POST  [ {host_id=1 process_name=foo } ]

*/

enum Action {
    View,
    Act,
}

fn user_can_do(
    user: &UserWithPermissions,
    action: Action,
    group_id: i32,
    host_id: i32,
    service_name: &str,
) -> bool {
    if user.is_admin {
        return true;
    }

    let mut allow = false;
    for perm in user.permissions.iter() {
        if perm.group_id != group_id {
            continue;
        }
        if let Some(perm_host_id) = perm.host_id {
            if perm_host_id != host_id {
                continue;
            }
        }
        if !Regex::new(&perm.service_name)
            .unwrap()
            .is_match(&service_name)
        {
            continue;
        }
        match action {
            Action::View => allow = perm.can_view,
            Action::Act => allow = perm.can_act,
        }
    }
    allow
}

impl Server {
    fn from_host(host: &Host) -> Self {
        Self::new(
            &host.hostname,
            host.port as u16,
            host.username.as_deref(),
            host.password.as_deref(),
        )
    }
}

#[axum::debug_handler]
pub async fn list(
    State(state): State<AppState>,
    Query(req): Query<ProcessRequest>,
    Extension(user): Extension<UserWithPermissions>,
) -> (StatusCode, Json<Vec<ProcessResponse>>) {
    let mut db_conn = state.db_pool.get().await.unwrap();

    let mut db_query = schema::host::table.into_boxed();
    if let Some(host_id) = req.host_id {
        db_query = db_query.filter(schema::host::id.eq(host_id));
    }
    if let Some(group_id) = req.group_id {
        db_query = db_query.filter(schema::host::group_id.eq(group_id));
    }
    let hosts: Vec<Host> = db_query.load(&mut *db_conn).await.unwrap();

    let mut host_with_processes: Vec<(Host, Vec<ProcessInfo>)> = Vec::with_capacity(hosts.len());
    let mut join_set = JoinSet::new();
    for host in hosts {
        let server = Server::from_host(&host);
        join_set.spawn(async move {
            println!("Getting process for {}:{}", host.hostname, host.port);
            (host, server.get_all_process_info().await.unwrap())
        });
    }

    while let Some(res) = join_set.join_next().await {
        host_with_processes.push(res.unwrap());
    }

    let mut result = Vec::with_capacity(host_with_processes.len());
    for (host, processes) in host_with_processes {
        for process in processes {
            if let Some(process_name) = &req.process_name {
                if process.name != *process_name {
                    continue;
                }
            }
            if user_can_do(&user, Action::View, host.group_id, host.id, &process.name) {
                result.push(ProcessResponse {
                    group_id: host.group_id,
                    host_id: host.id,
                    process,
                });
            }
        }
    }

    (StatusCode::OK, Json(result))
}

#[axum::debug_handler]
pub async fn get_config(
    State(state): State<AppState>,
    Query(query): Query<ProcessConfigRequest>,
    Extension(user): Extension<UserWithPermissions>,
) -> (StatusCode, Json<Vec<ProcessConfigResponse>>) {
    let mut db_conn = state.db_pool.get().await.unwrap();

    let host: Host = schema::host::table
        .filter(schema::host::id.eq(query.host_id))
        .first(&mut db_conn)
        .await
        .unwrap();

    let server = Server::from_host(&host);

    let configs: Vec<ProcessConfigResponse> = server
        .get_all_config_info()
        .await
        .unwrap()
        .into_iter()
        .filter_map(|config| {
            if let Some(process_name) = &query.process_name
                && config.name != *process_name
            {
                return None;
            }
            if !user_can_do(&user, Action::View, host.group_id, host.id, &config.name) {
                return None;
            }
            Some(ProcessConfigResponse {
                host_id: host.id,
                config,
            })
        })
        .collect();

    (StatusCode::OK, Json(configs))
}

async fn perform_action<F, Fut>(
    state: &AppState,
    user: &UserWithPermissions,
    requests: Vec<ProcessActionRequest>,
    action: F,
) -> Vec<ProcessActionResponse>
where
    F: Fn(Server, String) -> Fut,
    Fut: Future<Output = crate::supervisor::Result<bool>>,
{
    let mut results = Vec::with_capacity(requests.len());

    let host_ids: Vec<i32> = requests.iter().map(|r| r.host_id).collect();
    let mut db_conn = state.db_pool.get().await.unwrap();
    let hosts_res: Result<Vec<Host>, _> = schema::host::table
        .filter(schema::host::id.eq_any(host_ids))
        .load(&mut *db_conn)
        .await;
    drop(db_conn);

    let host_map: HashMap<i32, Host> = match hosts_res {
        Ok(hosts) => hosts.into_iter().map(|h| (h.id, h)).collect(),
        Err(e) => {
            let error = Some(format!("Failed to load hosts: {}", e));
            return requests
                .into_iter()
                .map(|req| ProcessActionResponse {
                    host_id: req.host_id,
                    process_name: req.process_name,
                    success: false,
                    error: error.clone(),
                })
                .collect();
        }
    };

    // TODO req async
    for req in requests {
        let host = match host_map.get(&req.host_id) {
            Some(h) => h,
            None => {
                results.push(ProcessActionResponse {
                    host_id: req.host_id,
                    process_name: req.process_name,
                    success: false,
                    error: Some("Host not found".to_string()),
                });
                continue;
            }
        };

        if !user_can_do(user, Action::Act, host.group_id, host.id, &req.process_name) {
            results.push(ProcessActionResponse {
                host_id: req.host_id,
                process_name: req.process_name,
                success: false,
                error: Some("Permission denied".to_string()),
            });
            continue;
        }

        let server = Server::from_host(host);
        match action(server, req.process_name.clone()).await {
            Ok(success) => {
                results.push(ProcessActionResponse {
                    host_id: req.host_id,
                    process_name: req.process_name,
                    success,
                    error: None,
                });
            }
            Err(e) => {
                results.push(ProcessActionResponse {
                    host_id: req.host_id,
                    process_name: req.process_name,
                    success: false,
                    error: Some(e.to_string()),
                });
            }
        }
    }

    results
}

#[axum::debug_handler]
pub async fn start(
    State(state): State<AppState>,
    Extension(user): Extension<UserWithPermissions>,
    Json(requests): Json<Vec<ProcessActionRequest>>,
) -> (StatusCode, Json<Vec<ProcessActionResponse>>) {
    let results = perform_action(&state, &user, requests, |server, name| async move {
        server.start_process(&name, false).await
    })
    .await;

    (StatusCode::OK, Json(results))
}

#[axum::debug_handler]
pub async fn stop(
    State(state): State<AppState>,
    Extension(user): Extension<UserWithPermissions>,
    Json(requests): Json<Vec<ProcessActionRequest>>,
) -> (StatusCode, Json<Vec<ProcessActionResponse>>) {
    let results = perform_action(&state, &user, requests, |server, name| async move {
        server.stop_process(&name, false).await
    })
    .await;

    (StatusCode::OK, Json(results))
}

#[axum::debug_handler]
pub async fn restart(
    State(state): State<AppState>,
    Extension(user): Extension<UserWithPermissions>,
    Json(requests): Json<Vec<ProcessActionRequest>>,
) -> (StatusCode, Json<Vec<ProcessActionResponse>>) {
    let stop_result = perform_action(&state, &user, requests.clone(), |server, name| async move {
        server.stop_process(&name, true).await
    })
    .await;

    let mut to_restart = vec![];
    let mut success = HashSet::new();
    for i in 0..stop_result.len() {
        if stop_result[i].success {
            to_restart.push(requests[i].clone());
            success.insert(i);
        }
    }
    
    let start_results = perform_action(&state, &user, to_restart, |server, name| async move {
        server.start_process(&name, true).await
    })
    .await;
    
    let mut result = Vec::with_capacity(requests.len());
    for (i, res) in stop_result.into_iter().enumerate() {
        if success.contains(&i) {
            result.push(start_results[i].clone());
        } else {
            result.push(res);
        }
    }

    (StatusCode::OK, Json(result))
}
