#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate rocket;

use std::net::Ipv4Addr;

use middleware::auth::Token;
use rbatis::rbatis::RBatis;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
use rocket::{Config, Request};
use crate::handler::system::{sys_menu_handler, sys_role_handler, sys_user_handler};

pub mod common;
pub mod handler;
pub mod middleware;
pub mod model;
pub mod utils;
pub mod vo;
pub mod route;

#[get("/ping")]
fn ping(_auth: Token) -> &'static str {
    "pong"
}

#[catch(404)]
fn not_found(req: &Request) -> Value {
    json!({"code": 1,"msg": format!("Sorry, '{}' is not a valid path", req.uri())})
}

#[catch(403)]
fn not_permissions(req: &Request) -> Value {
    json!({"code": 1,"msg": format!("you has no permissions request path: '{}'", req.uri())})
}

#[catch(401)]
fn resp() -> Value {
    json!({"code": 401,"msg": "Unauthorized","description": "The request requires user authentication"})
}

lazy_static! {
    static ref RB: RBatis = RBatis::new();
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    log4rs::init_file("src/config/log4rs.yaml", Default::default()).unwrap();

    RB.init(
        rbdc_mysql::driver::MysqlDriver {},
        "mysql://root:oMbPi5munxCsBSsiLoPV123@110.41.179.89:3306/axum",
    )
    .unwrap();

    let config = Config {
        address: Ipv4Addr::new(0, 0, 0, 0).into(),
        port: 8099,
        ..Config::debug_default()
    };

    let _rocket = rocket::build()
        .configure(config)
        .mount("/", routes![ping])
        .mount("/api", routes![
            sys_user_handler::add_sys_user,
            sys_user_handler::delete_sys_user,
            sys_user_handler::update_sys_user,
            sys_user_handler::update_sys_user_status,
            sys_user_handler::update_user_password,
            sys_user_handler::query_sys_user_detail,
            sys_user_handler::query_sys_user_list,
            sys_user_handler::login,
            sys_user_handler::query_user_role,
            sys_user_handler::update_user_role,
            sys_user_handler::query_user_menu,
            sys_role_handler::add_sys_role,
            sys_role_handler::delete_sys_role,
            sys_role_handler::update_sys_role,
            sys_role_handler::update_sys_role_status,
            sys_role_handler::query_sys_role_detail,
            sys_role_handler::query_sys_role_list,
            sys_role_handler::query_role_menu,
            sys_role_handler::update_role_menu,
            sys_menu_handler::add_sys_menu,
            sys_menu_handler::delete_sys_menu,
            sys_menu_handler::update_sys_menu,
            sys_menu_handler::update_sys_menu_status,
            sys_menu_handler::query_sys_menu_detail,
            sys_menu_handler::query_sys_menu_list,
        ])
        .register("/", catchers![not_found,resp,not_permissions])
        .launch()
        .await?;

    Ok(())
}
