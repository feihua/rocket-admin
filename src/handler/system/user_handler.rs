use std::collections::{HashMap, HashSet};

use rbatis::rbdc::datetime::DateTime;
use rbatis::plugin::page::PageRequest;
use rbs::to_value;
use rocket::serde::json::{Json, Value};
use rocket::serde::json::serde_json::json;

use crate::model::system::menu::SysMenu;
use crate::model::system::role::SysRole;
use crate::model::system::user::SysUser;
use crate::model::system::user_role::SysUserRole;
use crate::RB;
use crate::utils::auth::Token;
use crate::utils::error::WhoUnfollowedError;
use crate::utils::jwt_util::JWTToken;
use crate::vo::{err_result_msg, err_result_page, handle_result, ok_result_data, ok_result_msg, ok_result_page};
use crate::vo::system::user_vo::*;

// 后台用户登录
#[post("/login", data = "<item>")]
pub async fn login(item: Json<UserLoginReq>) -> Value {
    log::info!("user login params: {:?}", &item);

    let user_result = SysUser::select_by_mobile(&mut RB.clone().clone(), &item.mobile).await;
    log::info!("select_by_mobile: {:?}",user_result);

    match user_result {
        Ok(u) => {
            match u {
                None => {
                    return json!(err_result_msg("用户不存在".to_string()));
                }
                Some(user) => {
                    let id = user.id.unwrap();
                    let username = user.user_name;
                    let password = user.password;

                    if password.ne(&item.password) {
                        return json!(err_result_msg("密码不正确".to_string()));
                    }

                    let btn_menu = query_btn_menu(&id).await;

                    if btn_menu.len() == 0 {
                        return json!(err_result_msg("用户没有分配角色或者菜单,不能登录".to_string()));
                    }

                    match JWTToken::new(id, &username, btn_menu).create_token("123") {
                        Ok(token) => {
                            json!(ok_result_data(token))
                        }
                        Err(err) => {
                            let er = match err {
                                WhoUnfollowedError::JwtTokenError(s) => { s }
                                _ => "no math error".to_string()
                            };

                            json!(err_result_msg(er))
                        }
                    }
                }
            }
        }

        Err(err) => {
            log::info!("select_by_column: {:?}",err);
            return json!(err_result_msg("查询用户异常".to_string()));
        }
    }
}

async fn query_btn_menu(id: &i32) -> Vec<String> {
    let user_role = SysUserRole::is_admin(&mut RB.clone(), id).await;
    let mut btn_menu: Vec<String> = Vec::new();
    if user_role.unwrap().len() == 1 {
        let data = SysMenu::select_all(&mut RB.clone()).await;

        for x in data.unwrap() {
            btn_menu.push(x.api_url.unwrap_or_default());
        }
        log::info!("admin login: {:?}",id);
        btn_menu
    } else {
        let btn_menu_map: Vec<HashMap<String, String>> = RB.query_decode("select distinct u.api_url from sys_user_role t left join sys_role usr on t.role_id = usr.id left join sys_role_menu srm on usr.id = srm.role_id left join sys_menu u on srm.menu_id = u.id where t.user_id = ?", vec![to_value!(id)]).await.unwrap();
        for x in btn_menu_map {
            btn_menu.push(x.get("api_url").unwrap().to_string());
        }
        log::info!("ordinary login: {:?}",id);
        btn_menu
    }
}

#[post("/query_user_role", data = "<item>")]
pub async fn query_user_role(item: Json<QueryUserRoleReq>, _auth: Token) -> Value {
    log::info!("query_user_role params: {:?}", item);


    let user_role = SysUserRole::select_by_column(&mut RB.clone(), "user_id", item.user_id).await;
    let mut user_role_ids: Vec<i32> = Vec::new();

    for x in user_role.unwrap() {
        user_role_ids.push(x.role_id);
    }

    let sys_role = SysRole::select_all(&mut RB.clone()).await;

    let mut sys_role_list: Vec<UserRoleList> = Vec::new();

    for x in sys_role.unwrap() {
        sys_role_list.push(UserRoleList {
            id: x.id.unwrap(),
            status_id: x.status_id,
            sort: x.sort,
            role_name: x.role_name,
            remark: x.remark.unwrap_or_default(),
            create_time: x.create_time.unwrap().0.to_string(),
            update_time: x.update_time.unwrap().0.to_string(),
        });
    }

    json!(ok_result_data(QueryUserRoleData {
        sys_role_list,
        user_role_ids,
    }))
}

#[post("/update_user_role", data = "<item>")]
pub async fn update_user_role(item: Json<UpdateUserRoleReq>, _auth: Token) -> Value {
    log::info!("update_user_role params: {:?}", item);


    let user_role = item.0;
    let user_id = user_role.user_id;
    let role_ids = &user_role.role_ids;
    let len = user_role.role_ids.len();

    if user_id == 1 {
        return json!(err_result_msg("不能修改超级管理员的角色".to_string()));
    }

    let sys_result = SysUserRole::delete_by_column(&mut RB.clone(), "user_id", user_id).await;

    if sys_result.is_err() {
        return json!(err_result_msg("更新用户角色异常".to_string()));
    }

    let mut sys_role_user_list: Vec<SysUserRole> = Vec::new();
    for role_id in role_ids {
        let r_id = role_id.clone();
        sys_role_user_list.push(SysUserRole {
            id: None,
            create_time: Some(DateTime::now()),
            update_time: Some(DateTime::now()),
            status_id: 1,
            sort: 1,
            role_id: r_id,
            user_id: user_id.clone(),
        })
    }

    let result = SysUserRole::insert_batch(&mut RB.clone(), &sys_role_user_list, len as u64).await;

    json!(&handle_result(result))
}

#[get("/query_user_menu")]
pub async fn query_user_menu(auth: Token) -> Value {
    log::info!("query_user_menu params: {:?}", auth);


    //根据id查询用户
    let result = SysUser::select_by_id(&mut RB.clone(), auth.id).await;

    match result {
        Ok(sys_user) => {
            match sys_user {
                // 用户不存在的情况
                None => {
                    json!(err_result_msg("用户不存在".to_string()))
                }
                Some(user) => {
                    //role_id为1是超级管理员--判断是不是超级管理员
                    let sql_str = "select count(id) from sys_user_role where role_id = 1 and user_id = ?";
                    let count = RB.query_decode::<i32>(sql_str, vec![to_value!(user.id)]).await.unwrap();

                    let sys_menu_list: Vec<SysMenu>;

                    if count > 0 {
                        sys_menu_list = SysMenu::select_all(&mut RB.clone()).await.unwrap_or_default();
                    } else {
                        let sql_str = "select u.* from sys_user_role t left join sys_role usr on t.role_id = usr.id left join sys_role_menu srm on usr.id = srm.role_id left join sys_menu u on srm.menu_id = u.id where t.user_id = ?";
                        sys_menu_list = RB.query_decode(sql_str, vec![to_value!(user.id)]).await.unwrap();
                    }

                    let mut sys_menu: Vec<MenuUserList> = Vec::new();
                    let mut btn_menu: Vec<String> = Vec::new();
                    let mut sys_menu_ids: HashSet<i32> = HashSet::new();

                    for x in sys_menu_list {
                        if x.menu_type != 3 {
                            sys_menu_ids.insert(x.id.unwrap_or_default().clone());
                            sys_menu_ids.insert(x.parent_id.clone());
                        }

                        if x.api_url.clone().unwrap_or_default().len() > 0 {
                            btn_menu.push(x.api_url.unwrap_or_default());
                        }
                    }

                    let mut menu_ids = Vec::new();
                    for id in sys_menu_ids {
                        menu_ids.push(id)
                    }
                    let menu_result = SysMenu::select_by_ids(&mut RB.clone(), &menu_ids).await.unwrap();
                    for menu in menu_result {
                        sys_menu.push(MenuUserList {
                            id: menu.id.unwrap(),
                            parent_id: menu.parent_id,
                            name: menu.menu_name,
                            icon: menu.menu_icon.unwrap_or_default(),
                            api_url: menu.api_url.as_ref().unwrap().to_string(),
                            menu_type: menu.menu_type,
                            path: menu.menu_url.unwrap_or_default(),
                        });
                    }

                    json!(ok_result_data(QueryUserMenuData {
                            sys_menu,
                            btn_menu,
                            avatar: "https://gw.alipayobjects.com/zos/antfincdn/XAosXuNZyF/BiazfanxmamNRoxxVxka.png".to_string(),
                            name: user.user_name,
                    }))
                }
            }
        }
        // 查询用户数据库异常
        Err(err) => {
            json!(err_result_msg(err.to_string()))
        }
    }
}

// 查询用户列表
#[post("/user_list", data = "<item>")]
pub async fn user_list(item: Json<UserListReq>, _auth: Token) -> Value {
    log::info!("query user_list params: {:?}", &item);


    let mobile = item.mobile.as_deref().unwrap_or_default();
    let status_id = item.status_id.as_deref().unwrap_or_default();

    let page_req = &PageRequest::new(item.page_no.clone(), item.page_size.clone());
    let result = SysUser::select_page_by_name(&mut RB.clone(), page_req, mobile, status_id).await;

    match result {
        Ok(page) => {
            let total = page.total;

            let mut list_data: Vec<UserListData> = Vec::new();

            for user in page.records {
                list_data.push(UserListData {
                    id: user.id.unwrap(),
                    sort: user.sort,
                    status_id: user.status_id,
                    mobile: user.mobile,
                    user_name: user.user_name,
                    remark: user.remark.unwrap_or_default(),
                    create_time: user.create_time.unwrap().0.to_string(),
                    update_time: user.update_time.unwrap().0.to_string(),
                })
            }

            json!(ok_result_page(list_data, total))
        }
        Err(err) => {
            json!(err_result_page(err.to_string()))
        }
    }
}

// 添加用户信息
#[post("/user_save", data = "<item>")]
pub async fn user_save(item: Json<UserSaveReq>, _auth: Token) -> Value {
    log::info!("user_save params: {:?}", &item);

    let user = item.0;


    let sys_user = SysUser {
        id: None,
        create_time: Some(DateTime::now()),
        update_time: Some(DateTime::now()),
        status_id: user.status_id,
        sort: user.sort,
        mobile: user.mobile,
        user_name: user.user_name,
        remark: user.remark,
        password: "123456".to_string(),//默认密码为123456,暂时不加密
    };

    let result = SysUser::insert(&mut RB.clone(), &sys_user).await;

    json!(&handle_result(result))
}

// 更新用户信息
#[post("/user_update", data = "<item>")]
pub async fn user_update(item: Json<UserUpdateReq>, _auth: Token) -> Value {
    log::info!("user_update params: {:?}", &item);

    let user = item.0;


    let result = SysUser::select_by_id(&mut RB.clone(), user.id.clone()).await.unwrap();

    match result {
        None => {
            json!(err_result_msg("用户不存在".to_string()))
        }
        Some(s_user) => {
            let sys_user = SysUser {
                id: Some(user.id),
                create_time: s_user.create_time,
                update_time: Some(DateTime::now()),
                status_id: user.status_id,
                sort: user.sort,
                mobile: user.mobile,
                user_name: user.user_name,
                remark: user.remark,
                password: s_user.password,
            };

            let result = SysUser::update_by_column(&mut RB.clone(), &sys_user, "id").await;

            json!(&handle_result(result))
        }
    }
}

// 删除用户信息
#[post("/user_delete", data = "<item>")]
pub async fn user_delete(item: Json<UserDeleteReq>, _auth: Token) -> Value {
    log::info!("user_delete params: {:?}", &item);


    let ids = item.ids.clone();
    for id in ids {
        if id != 1 {//id为1的用户为系统预留用户,不能删除
            let _ = SysUser::delete_by_column(&mut RB.clone(), "id", &id).await;
        }
    }

    json!(ok_result_msg("删除用户信息成功".to_string()))
}

// 更新用户密码
#[post("/update_user_password", data = "<item>")]
pub async fn update_user_password(item: Json<UpdateUserPwdReq>, _auth: Token) -> Value {
    log::info!("update_user_pwd params: {:?}", &item);

    let user_pwd = item.0;


    let sys_user_result = SysUser::select_by_id(&mut RB.clone(), user_pwd.id).await;

    match sys_user_result {
        Ok(user_result) => {
            match user_result {
                None => {
                    json!(err_result_msg("用户不存在".to_string()))
                }
                Some(mut user) => {
                    if user.password == user_pwd.pwd {
                        user.password = user_pwd.re_pwd;
                        let result = SysUser::update_by_column(&mut RB.clone(), &user, "id").await;

                        json!(&handle_result(result))
                    } else {
                        json!(err_result_msg("旧密码不正确".to_string()))
                    }
                }
            }
        }
        Err(err) => {
            json!(err_result_msg(err.to_string()))
        }
    }
}
