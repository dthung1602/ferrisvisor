mod database;
mod models;
mod schema;
mod supervisor;

use diesel::QueryDsl;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use models::{Host, User};
use crate::models::HasPassword;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = database::establish_connection().await?;
    let mut conn = db.read_pool.get().await?;

    // let new_user = models::NewUser::new("admin@example.com", "123", true);

    // diesel::insert_into(schema::user::table)
    //     .values(&new_user)
    //     .execute(&mut conn)
    //     .await?;

    // let users: Vec<User> = schema::user::table
    //     .select(User::as_select())
    //     .load(&mut conn)
    //     .await?;
    //
    // let res = users[0].verify_password("123");
    // println!("{:?}", res);
    // let res = users[0].verify_password("123sss");
    // println!("{:?}", res);
    //
    // println!("{:?}", users);

    // let host_obj = models::NewHost {
    //     name: "dev_host1".to_string(),
    //     port: 9001,
    //     username: Some("user".to_string()),
    //     password: Some("123".to_string()),
    // };
    //
    // diesel::insert_into(schema::host::table)
    //     .values(&host_obj)
    //     .execute(&mut conn)
    //     .await?;
    //
    // let hosts: Vec<Host> = schema::host::table
    //     .select(Host::as_select())
    //     .load(&mut conn)
    //     .await?;
    //
    // println!("{:?}", hosts);

    Ok(())
}

async fn test_rpc() -> anyhow::Result<()> {
    println!("Starting Supervisor RPC Client");
    let server = supervisor::Server::new("localhost", 9001, Some("user"), Some("123"));
    let res = server.get_api_version().await?;
    println!("{:?}", res);
    // let res = server.get_supervisor_version().await?;
    // println!("{:?}", res);
    // let res = server.get_state().await?;
    // println!("{:?}", res);
    // let res = server.get_pid().await?;
    // println!("{:?}", res);
    // let res = server.clear_log().await?;
    // println!("{:?}", res);
    // let res = server.read_log(100, 100).await?;
    // println!("{:?}", res);
    // let res = server.shutdown().await?;
    // println!("{:?}", res);
    // let res = server.restart().await?;
    // println!("{:?}", res);
    // let res = server.get_pid().await?;
    // println!("{:?}", res);
    // let res = server.get_all_process_info().await?;
    // println!("{:?}", res);
    // let res = server.get_process_info("group_normal:normal1").await?;
    // println!("{:?}", res);
    // let res = server.get_all_config_info().await?;
    // println!("{:?}", res);
    // let res = server.read_process_stdout_log("group_normal:normal1", 100, 100).await?;
    // println!("{:?}", res);
    // let res = server.tail_process_stdout_log("group_normal:normal1", 500, 100).await?;
    // println!("{:?}", res);
    // let res = server.start_process("group_normal:normal1", true).await?;
    // println!("{:?}", res);
    // let res = server.start_all_processes(true).await?;
    // println!("{:?}", res);
    // let res = server.stop_process("group_normal:normal1", true).await?;
    // println!("{:?}", res);
    // let res = server.stop_all_processes(true).await?;
    // println!("{:?}", res);
    // let res = server.stop_process_group("group_normal", true).await?;
    // println!("{:?}", res);
    // let res = server.signal_process("group_normal:normal1", "15").await?;
    // println!("{:?}", res);
    // let res = server.signal_process_group("group_normal", "15").await?;
    // println!("{:?}", res);
    // let res = server.signal_all_processes( "15").await?;
    // println!("{:?}", res);
    // let res = server.send_process_stdin("group_normal:normal1", "TEST TEST HELLOO").await?;
    // println!("{:?}", res);
    // let res = server.reload_config().await?;
    // println!("{:?}", res);

    Ok(())
}
