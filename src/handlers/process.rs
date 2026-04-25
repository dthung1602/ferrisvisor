use crate::common::AppState;
use crate::models::{DisplayProcess, Host, ProcessQuery, UserWithPermissions};
use crate::schema;
use crate::supervisor::{ProcessInfo, Server};
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use diesel::QueryDsl;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use regex::Regex;
use tokio::task::JoinSet;

/**

/process        -> GET ? group_id=1 & host_id=1 & process_name=foo
/process/config -> GET ? host_id=1 & process_name=foo
/process/stdout -> GET ? host_id=1 & process_name=foo offset=1 length=213
/process/stderr -> GET ? host_id=1 & process_name=foo offset=1 length=213
/process/start  -> POST  [ {host_id=1 name=foo } ]
/process/stop   -> POST  [ {host_id=1 name=foo } ]

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

#[axum::debug_handler]
pub async fn list(
    State(state): State<AppState>,
    Query(query): Query<ProcessQuery>,
    Extension(user): Extension<UserWithPermissions>,
) -> (StatusCode, Json<Vec<DisplayProcess>>) {
    let mut db_conn = state.db_conn.lock().await;

    let mut db_query = schema::host::table.into_boxed();
    if let Some(host_id) = query.host_id {
        db_query = db_query.filter(schema::host::id.eq(host_id));
    }
    if let Some(group_id) = query.group_id {
        db_query = db_query.filter(schema::host::group_id.eq(group_id));
    }
    let hosts: Vec<Host> = db_query.load(&mut *db_conn).await.unwrap();

    let mut host_with_processes: Vec<(Host, Vec<ProcessInfo>)> = Vec::with_capacity(hosts.len());
    let mut join_set = JoinSet::new();
    for host in hosts {
        let server = Server::new(
            &host.hostname,
            host.port as u16,
            host.username.as_deref(),
            host.password.as_deref(),
        );
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
            if let Some(process_name) = &query.process_name {
                if process.name != *process_name {
                    continue;
                }
            }
            if user_can_do(&user, Action::View, host.group_id, host.id, &process.name) {
                result.push(DisplayProcess {
                    group_id: host.group_id,
                    host_id: host.id,
                    process,
                });
            }
        }
    }

    (StatusCode::OK, Json(result))
}
