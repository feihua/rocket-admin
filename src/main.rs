#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate rocket;

use std::net::Ipv4Addr;

use rbatis::rbatis::RBatis;
use rocket::{Config, Request};
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
use handler::system::{menu_handler, role_handler, user_handler};
use crate::utils::auth::Token;

pub mod handler;
pub mod model;
pub mod vo;
pub mod utils;

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

    RB.init(rbdc_mysql::driver::MysqlDriver {}, "mysql://root:ad879037-c7a4-4063-9236-6bfc35d54b7d@139.159.180.129:3306/rustdb").unwrap();

    let config = Config {
        address: Ipv4Addr::new(0, 0, 0, 0).into(),
        port: 8099,
        ..Config::debug_default()
    };

    let _rocket = rocket::build()
        .configure(config)
        .mount("/", routes![ping])
        .mount("/api", routes![user_handler::login,
            user_handler::query_user_role,
            user_handler::update_user_role,
            user_handler::query_user_menu,
            user_handler::user_list,
            user_handler::user_save,
            user_handler::user_delete,
            user_handler::user_update,
            user_handler::update_user_password,
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
        .register("/", catchers![not_found,resp,not_permissions])
        .launch()
        .await?;

    Ok(())
}