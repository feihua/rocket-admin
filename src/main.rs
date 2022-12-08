#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate lazy_static;

pub mod handler;
pub mod model;
pub mod vo;
pub mod utils;

use std::net::Ipv4Addr;
use std::sync::Arc;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
use crate::handler::{menu_handler, role_handler, user_handler};
use rbatis::rbatis::Rbatis;
use rocket::Config;

#[get("/ping")]
fn ping() -> &'static str {
    "pong"
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

lazy_static! {
    static ref RB: Rbatis = Rbatis::new();
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    log4rs::init_file("src/config/log4rs.yaml", Default::default()).unwrap();

    RB.init(rbdc_mysql::driver::MysqlDriver {}, "mysql://root:r-wz9wop62956dh5k9ed@rm-wz9a2yv489d123yqkdo.mysql.rds.aliyuncs.com:3306/zero-react").unwrap();
    let rb = Arc::new(&RB);

    let config = Config {
        address: Ipv4Addr::new(0, 0, 0, 0).into(),
        port: 8099,
        ..Config::debug_default()
    };

    let _rocket = rocket::build()
        .configure(config)
        .mount("/", routes![ping])
        .mount("/api", routes![user_handler::login,
            user_handler::query_user_menu,
            user_handler::user_list,
            user_handler::user_save,
            user_handler::user_delete,
            user_handler::user_update,
            role_handler::query_role_menu,
            role_handler::update_role_menu,
            role_handler::role_list,
            role_handler::role_save,
            role_handler::role_delete,
            role_handler::role_update,
            menu_handler::menu_list,
            menu_handler::menu_save,
            menu_handler::menu_delete,
            menu_handler::menu_update,])
        .register("/", catchers![not_found])
        .manage(rb)
        .launch()
        .await?;

    Ok(())
}