use std::collections::HashMap;
use rocket::serde::json::{Json, Value};
use rocket::serde::json::serde_json::json;
use rbatis::rbdc::datetime::DateTime;
use rbatis::sql::{PageRequest};
use rbs::to_value;
use crate::model::user::{SysUser};
use crate::model::menu::{SysMenu};
use crate::model::role::{SysRole};
use crate::model::user_role::{SysUserRole};
use crate::utils::error::WhoUnfollowedError;
use crate::utils::jwt_util::JWTToken;
use crate::vo::user_vo::*;
use crate::vo::{BaseResponse, handle_result};
use crate::RB;
use crate::utils::auth::Token;

// 后台用户登录
#[post("/login", data = "<item>")]
pub async fn login(item: Json<UserLoginReq>) -> Value {
    log::info!("user login params: {:?}", &item);
    let mut rb = RB.to_owned();

    let user_result = SysUser::select_by_column(&mut rb, "mobile", &item.mobile).await;
    log::info!("select_by_column: {:?}",user_result);

    match user_result {
        Ok(d) => {
            if d.len() == 0 {
                return json!({"code":1,"msg":"用户不存在".to_string()});
            }

            let user = d.get(0).unwrap().clone();
            let id = user.id.unwrap().to_string();
            let username = user.user_name;
            let password = user.password;

            if password.ne(&item.password) {
                return json!({"code":1,"msg":"密码不正确".to_string()});
            }

            let data = SysMenu::select_page(&mut rb, &PageRequest::new(1, 1000)).await;

            let mut btn_menu: Vec<String> = Vec::new();

            for x in data.unwrap().records {
                btn_menu.push(x.api_url.unwrap_or_default());
            }

            match JWTToken::new(&id, &username, btn_menu).create_token("123") {
                Ok(token) => {
                    let resp = BaseResponse {
                        msg: "successful".to_string(),
                        code: 0,
                        data: Some(UserLoginData {
                            mobile: item.mobile.to_string(),
                            token,
                        }),
                    };

                    json!(&resp)
                }
                Err(err) => {
                    let er = match err {
                        WhoUnfollowedError::JwtTokenError(s) => { s }
                        _ => "no math error".to_string()
                    };

                    json!({"code":1,"msg":er})
                }
            }
        }

        Err(err) => {
            log::info!("select_by_column: {:?}",err);
            return json!({"code":1,"msg":"查询用户异常".to_string()});
        }
    }
}


#[post("/query_user_role", data = "<item>")]
pub async fn query_user_role(item: Json<QueryUserRoleReq>, _auth: Token) -> Value {
    log::info!("query_user_role params: {:?}", item);
    let mut rb = RB.to_owned();

    let sys_role = SysRole::select_page(&mut rb, &PageRequest::new(1, 1000)).await;

    let mut sys_role_list: Vec<UserRoleList> = Vec::new();
    let mut user_role_ids: Vec<i32> = Vec::new();

    for x in sys_role.unwrap().records {
        sys_role_list.push(UserRoleList {
            id: x.id.unwrap(),
            status_id: x.status_id,
            sort: x.sort,
            role_name: x.role_name,
            remark: x.remark.unwrap_or_default(),
            create_time: x.create_time.unwrap().0.to_string(),
            update_time: x.update_time.unwrap().0.to_string(),
        });

        user_role_ids.push(x.id.unwrap_or_default());
    }

    let resp = QueryUserRoleResp {
        msg: "successful".to_string(),
        code: 0,
        data: QueryUserRoleData {
            sys_role_list,
            user_role_ids,
        },
    };

    json!(&resp)
}

#[post("/update_user_role", data = "<item>")]
pub async fn update_user_role(item: Json<UpdateUserRoleReq>, _auth: Token) -> Value {
    log::info!("update_user_role params: {:?}", item);
    let mut rb = RB.to_owned();

    let user_role = item.0;
    let user_id = user_role.user_id;
    let role_ids = &user_role.role_ids;
    let len = user_role.role_ids.len();

    if user_id == 1 {
        return json!({"code":1,"msg":"不能修改超级管理员的角色"});
    }

    let sys_result = SysUserRole::delete_by_column(&mut rb, "user_id", user_id).await;

    if sys_result.is_err() {
        return json!({"code":1,"msg":"更新用户角色异常"});
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

    let result = SysUserRole::insert_batch(&mut rb, &sys_role_user_list, len as u64).await;

    json!(&handle_result(result))
}

#[get("/query_user_menu")]
pub async fn query_user_menu(auth: Token) -> Value {
    log::info!("query_user_menu params: {:?}", auth);
    let mut rb = RB.to_owned();

    //根据id查询用户
    let result = SysUser::select_by_id(&mut rb, auth.id).await;

    match result {
        Ok(sys_user) => {
            match sys_user {
                // 用户不存在的情况
                None => {
                    json!(BaseResponse {
                        msg: "用户不存在".to_string(),
                        code: 1,
                        data: Some(""),
                    })
                }
                Some(user) => {
                    let user_role = SysUserRole::select_by_column(&mut rb, "user_id", user.id).await;
                    // 判断是不是超级管理员
                    let mut is_admin = false;

                    for x in user_role.unwrap() {
                        if x.role_id == 1 {
                            is_admin = true;
                            break;
                        }
                    }

                    let sys_menu_list: Vec<SysMenu>;

                    if is_admin {
                        sys_menu_list = SysMenu::select_all(&mut rb).await.unwrap_or_default();
                    } else {
                        sys_menu_list = RB.query_decode("select u.* from sys_user_role t left join sys_role usr on t.role_id = usr.id left join sys_role_menu srm on usr.id = srm.role_id left join sys_menu u on srm.menu_id = u.id where t.user_id = ? order by u.id asc", vec![to_value!(user.id)]).await.unwrap();
                    }

                    let mut sys_menu_map: HashMap<i32, MenuUserList> = HashMap::new();
                    let mut sys_menu: Vec<MenuUserList> = Vec::new();
                    let mut btn_menu: Vec<String> = Vec::new();
                    let mut sys_menu_parent_ids: Vec<i32> = Vec::new();

                    for x in sys_menu_list {
                        let y = x.clone();
                        if y.menu_type != 3 {
                            sys_menu_map.insert(y.id.unwrap(), MenuUserList {
                                id: y.id.unwrap(),
                                parent_id: y.parent_id,
                                name: y.menu_name,
                                icon: y.menu_icon.unwrap_or_default(),
                                api_url: y.api_url.as_ref().unwrap().to_string(),
                                menu_type: y.menu_type,
                                path: y.menu_url.unwrap_or_default(),
                            });
                            sys_menu_parent_ids.push(y.parent_id.clone())
                        }

                        btn_menu.push(x.api_url.unwrap_or_default());
                    }

                    for menu_id in sys_menu_parent_ids {
                        let s_menu_result = SysMenu::select_by_id(&mut rb, menu_id).await.unwrap();
                        match s_menu_result {
                            None => {}
                            Some(y) => {
                                sys_menu_map.insert(y.id.unwrap(), MenuUserList {
                                    id: y.id.unwrap(),
                                    parent_id: y.parent_id,
                                    name: y.menu_name,
                                    icon: y.menu_icon.unwrap_or_default(),
                                    api_url: y.api_url.as_ref().unwrap().to_string(),
                                    menu_type: y.menu_type,
                                    path: y.menu_url.unwrap_or_default(),
                                });
                            }
                        }
                    }

                    let mut sys_menu_ids: Vec<i32> = Vec::new();
                    for menu in &sys_menu_map {
                        sys_menu_ids.push(menu.0.abs())
                    }

                    sys_menu_ids.sort();

                    for id in sys_menu_ids {
                        let menu = sys_menu_map.get(&id).cloned().unwrap();
                        sys_menu.push(menu)
                    }

                    let resp = BaseResponse {
                        msg: "successful".to_string(),
                        code: 0,
                        data: Some(QueryUserMenuData {
                            sys_menu,
                            btn_menu,
                            avatar: "https://gw.alipayobjects.com/zos/antfincdn/XAosXuNZyF/BiazfanxmamNRoxxVxka.png".to_string(),
                            name: user.user_name,
                        }),
                    };
                    json!(resp)
                }
            }
        }
        // 查询用户数据库异常
        Err(err) => {
            json!(BaseResponse {
                msg: err.to_string(),
                code: 1,
                data: Some(""),
            })
        }
    }
}

// 查询用户列表
#[post("/user_list", data = "<item>")]
pub async fn user_list(item: Json<UserListReq>, _auth: Token) -> Value {
    log::info!("query user_list params: {:?}", &item);
    let mut rb = RB.to_owned();

    let mobile = item.mobile.as_deref().unwrap_or_default();
    let status_id = item.status_id.as_deref().unwrap_or_default();

    let page_req = &PageRequest::new(item.page_no.clone(), item.page_size.clone());
    let result = SysUser::select_page_by_name(&mut rb, page_req, mobile, status_id).await;

    let resp = match result {
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

            UserListResp {
                msg: "successful".to_string(),
                code: 0,
                success: true,
                total,
                data: Some(list_data),
            }
        }
        Err(err) => {
            UserListResp {
                msg: err.to_string(),
                code: 1,
                success: true,
                total: 0,
                data: None,
            }
        }
    };

    json!(&resp)
}

// 添加用户信息
#[post("/user_save", data = "<item>")]
pub async fn user_save(item: Json<UserSaveReq>, _auth: Token) -> Value {
    log::info!("user_save params: {:?}", &item);

    let user = item.0;

    let mut rb = RB.to_owned();
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

    let result = SysUser::insert(&mut rb, &sys_user).await;

    json!(&handle_result(result))
}

// 更新用户信息
#[post("/user_update", data = "<item>")]
pub async fn user_update(item: Json<UserUpdateReq>, _auth: Token) -> Value {
    log::info!("user_update params: {:?}", &item);

    let user = item.0;

    let mut rb = RB.to_owned();
    let result = SysUser::select_by_id(&mut rb, user.id.clone()).await.unwrap();

    match result {
        None => {
            json!(BaseResponse {
                msg: "用户不存在".to_string(),
                code: 1,
                data: Some("None".to_string()),
            })
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

            let result = SysUser::update_by_column(&mut rb, &sys_user, "id").await;

            json!(&handle_result(result))
        }
    }
}

// 删除用户信息
#[post("/user_delete", data = "<item>")]
pub async fn user_delete(item: Json<UserDeleteReq>, _auth: Token) -> Value {
    log::info!("user_delete params: {:?}", &item);
    let mut rb = RB.to_owned();

    let result = SysUser::delete_in_column(&mut rb, "id", &item.ids).await;

    json!(&handle_result(result))
}

// 更新用户密码
#[post("/update_user_password", data = "<item>")]
pub async fn update_user_password(item: Json<UpdateUserPwdReq>, _auth: Token) -> Value {
    log::info!("update_user_pwd params: {:?}", &item);

    let user_pwd = item.0;

    let mut rb = RB.to_owned();

    let sys_user_result = SysUser::select_by_id(&mut rb, user_pwd.id).await;

    match sys_user_result {
        Ok(user_result) => {
            match user_result {
                None => {
                    let resp = BaseResponse {
                        msg: "用户不存在".to_string(),
                        code: 1,
                        data: None,
                    };
                    json!(&resp)
                }
                Some(mut user) => {
                    if user.password == user_pwd.pwd {
                        user.password = user_pwd.re_pwd;
                        let result = SysUser::update_by_column(&mut rb, &user, "id").await;

                        json!(&handle_result(result))
                    } else {
                        let resp = BaseResponse {
                            msg: "旧密码不正确".to_string(),
                            code: 1,
                            data: None,
                        };
                        json!(&resp)
                    }
                }
            }
        }
        Err(err) => {
            json!({"code":1,"msg":err.to_string()})
        }
    }
}
