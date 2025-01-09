#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate rocket;

use std::net::Ipv4Addr;

use crate::handler::system::{
    sys_dept_handler, sys_dict_data_handler, sys_dict_type_handler, sys_login_log_handler,
    sys_menu_handler, sys_notice_handler, sys_operate_log_handler, sys_post_handler,
    sys_role_handler, sys_user_handler,
};
use middleware::auth::Token;
use rbatis::rbatis::RBatis;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
use rocket::{Config, Request};

pub mod common;
pub mod handler;
pub mod middleware;
pub mod model;
pub mod route;
pub mod utils;
pub mod vo;

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
        "mysql://root:123456@127.0.0.1:3306/rustdb",
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
        .mount(
            "/api",
            routes![
                sys_user_handler::add_sys_user,
                sys_user_handler::delete_sys_user,
                sys_user_handler::update_sys_user,
                sys_user_handler::update_sys_user_status,
                sys_user_handler::update_sys_user_password,
                sys_user_handler::reset_sys_user_password,
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
                sys_role_handler::query_allocated_list,
                sys_role_handler::query_unallocated_list,
                sys_role_handler::cancel_auth_user,
                sys_role_handler::batch_cancel_auth_user,
                sys_role_handler::batch_auth_user,
                sys_menu_handler::add_sys_menu,
                sys_menu_handler::delete_sys_menu,
                sys_menu_handler::update_sys_menu,
                sys_menu_handler::update_sys_menu_status,
                sys_menu_handler::query_sys_menu_detail,
                sys_menu_handler::query_sys_menu_list,
                sys_menu_handler::query_sys_menu_list_simple,
                sys_post_handler::add_sys_post,
                sys_post_handler::delete_sys_post,
                sys_post_handler::update_sys_post,
                sys_post_handler::update_sys_post_status,
                sys_post_handler::query_sys_post_detail,
                sys_post_handler::query_sys_post_list,
                sys_operate_log_handler::delete_sys_operate_log,
                sys_operate_log_handler::query_sys_operate_log_detail,
                sys_operate_log_handler::query_sys_operate_log_list,
                sys_notice_handler::add_sys_notice,
                sys_notice_handler::delete_sys_notice,
                sys_notice_handler::update_sys_notice,
                sys_notice_handler::update_sys_notice_status,
                sys_notice_handler::query_sys_notice_detail,
                sys_notice_handler::query_sys_notice_list,
                sys_login_log_handler::delete_sys_login_log,
                sys_login_log_handler::query_sys_login_log_detail,
                sys_login_log_handler::query_sys_login_log_list,
                sys_dict_type_handler::add_sys_dict_type,
                sys_dict_type_handler::delete_sys_dict_type,
                sys_dict_type_handler::update_sys_dict_type,
                sys_dict_type_handler::update_sys_dict_type_status,
                sys_dict_type_handler::query_sys_dict_type_detail,
                sys_dict_type_handler::query_sys_dict_type_list,
                sys_dict_data_handler::add_sys_dict_data,
                sys_dict_data_handler::delete_sys_dict_data,
                sys_dict_data_handler::update_sys_dict_data,
                sys_dict_data_handler::update_sys_dict_data_status,
                sys_dict_data_handler::query_sys_dict_data_detail,
                sys_dict_data_handler::query_sys_dict_data_list,
                sys_dept_handler::add_sys_dept,
                sys_dept_handler::delete_sys_dept,
                sys_dept_handler::update_sys_dept,
                sys_dept_handler::update_sys_dept_status,
                sys_dept_handler::query_sys_dept_detail,
                sys_dept_handler::query_sys_dept_list,
            ],
        )
        .register("/", catchers![not_found, resp, not_permissions])
        .launch()
        .await?;

    Ok(())
}
