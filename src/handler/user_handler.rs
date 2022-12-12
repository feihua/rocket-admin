use rocket::serde::json::{Json, Value};
use rocket::serde::json::serde_json::json;
use rbatis::rbdc::datetime::FastDateTime;
use rbatis::sql::{PageRequest};
use crate::model::entity::{SysMenu, SysRole, SysRoleUser, SysUser};
use crate::utils::error::WhoUnfollowedError;
use crate::utils::jwt_util::JWTToken;
use crate::vo::user_vo::*;
use crate::vo::{BaseResponse, handle_result};
use crate::RB;
use crate::utils::auth::Token;

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
            let username = user.real_name.unwrap();
            let password = user.password.unwrap();

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
            status_id: x.status_id.unwrap(),
            sort: x.sort.unwrap(),
            role_name: x.role_name.unwrap_or_default(),
            remark: x.remark.unwrap_or_default(),
            create_time: x.gmt_create.unwrap().0.to_string(),
            update_time: x.gmt_modified.unwrap().0.to_string(),
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

    let sys_result = SysRoleUser::delete_by_column(&mut rb, "user_id", user_id).await;

    if sys_result.is_err() {
        return json!({"code":1,"msg":"更新用户角色异常"});
    }

    let mut sys_role_user_list: Vec<SysRoleUser> = Vec::new();
    for role_id in role_ids {
        sys_role_user_list.push(SysRoleUser {
            id: None,
            gmt_create: Some(FastDateTime::now()),
            gmt_modified: Some(FastDateTime::now()),
            status_id: Some(1),
            sort: Some(1),
            role_id: Some(*role_id),
            user_id: Some(user_id),
        })
    }

    let result = SysRoleUser::insert_batch(&mut rb, &sys_role_user_list, len as u64).await;

    json!(&handle_result(result))
}

#[get("/query_user_menu")]
pub async fn query_user_menu(auth: Token) -> Value {
    log::info!("query_user_menu params: {:?}", auth);
    let mut rb = RB.to_owned();

    let sys_user = SysUser::select_by_column(&mut rb, "id", "1").await;

    let data = SysMenu::select_page(&mut rb, &PageRequest::new(1, 1000)).await;

    let mut sys_menu: Vec<MenuUserList> = Vec::new();
    let mut btn_menu: Vec<String> = Vec::new();
    let mut btn_menu_str: String = String::new();

    for x in data.unwrap().records {
        let y = x.clone();
        if y.menu_type != Some(3) {
            sys_menu.push(MenuUserList {
                id: y.id.unwrap(),
                parent_id: y.parent_id.unwrap(),
                name: y.menu_name.unwrap_or_default(),
                icon: y.menu_icon.unwrap_or_default(),
                api_url: y.api_url.as_ref().unwrap().to_string(),
                menu_type: y.menu_type.unwrap(),
                path: y.menu_url.unwrap_or_default(),
            });
        }

        btn_menu.push(x.api_url.unwrap_or_default());
        btn_menu_str.push_str(&x.menu_name.unwrap_or_default());
        btn_menu_str.push_str(&",")
    }

    let resp = BaseResponse {
        msg: "successful".to_string(),
        code: 0,
        data: Some(QueryUserMenuData {
            sys_menu,
            btn_menu,
            avatar: "https://gw.alipayobjects.com/zos/antfincdn/XAosXuNZyF/BiazfanxmamNRoxxVxka.png".to_string(),
            name: sys_user.unwrap_or_default().get(0).unwrap().real_name.as_ref().expect("用户名不存在").to_string(),
        }),
    };

    json!(&resp)
}


#[post("/user_list", data = "<item>")]
pub async fn user_list(item: Json<UserListReq>, _auth: Token) -> Value {
    log::info!("query user_list params: {:?}", &item);
    let mut rb = RB.to_owned();

    let mobile = item.mobile.as_deref().unwrap_or_default();
    let status_id = item.status_id.as_deref().unwrap_or_default();

    let page = &PageRequest::new(item.page_no, item.page_size);
    let result = SysUser::select_page_by_name(&mut rb, page, mobile, status_id).await;

    let resp = match result {
        Ok(d) => {
            let total = d.total;
            let page_no = d.page_no;
            let page_size = d.page_size;

            let mut user_list: Vec<UserListData> = Vec::new();

            for x in d.records {
                user_list.push(UserListData {
                    id: x.id.unwrap(),
                    sort: x.sort.unwrap(),
                    status_id: x.status_id.unwrap(),
                    mobile: x.mobile.unwrap_or_default(),
                    real_name: x.real_name.unwrap_or_default(),
                    remark: x.remark.unwrap_or_default(),
                    create_time: x.gmt_create.unwrap().0.to_string(),
                    update_time: x.gmt_modified.unwrap().0.to_string(),
                })
            }

            UserListResp {
                msg: "successful".to_string(),
                code: 0,
                page_no,
                page_size,
                success: true,
                total,
                data: Some(user_list),
            }
        }
        Err(err) => {
            UserListResp {
                msg: err.to_string(),
                code: 1,
                page_no: 0,
                page_size: 0,
                success: true,
                total: 0,
                data: None,
            }
        }
    };

    json!(&resp)
}


#[post("/user_save", data = "<item>")]
pub async fn user_save(item: Json<UserSaveReq>, _auth: Token) -> Value {
    log::info!("user_save params: {:?}", &item);

    let user = item.0;

    let mut rb = RB.to_owned();
    let sys_user = SysUser {
        id: None,
        gmt_create: Some(FastDateTime::now()),
        gmt_modified: Some(FastDateTime::now()),
        status_id: Some(1),
        sort: Some(1),
        user_no: Some(1),
        mobile: Some(user.mobile),
        real_name: Some(user.real_name),
        remark: Some(user.remark),
        password: Some("123456".to_string()),
    };

    let result = SysUser::insert(&mut rb, &sys_user).await;

    json!(&handle_result(result))
}


#[post("/user_update", data = "<item>")]
pub async fn user_update(item: Json<UserUpdateReq>, _auth: Token) -> Value {
    log::info!("user_update params: {:?}", &item);

    let user = item.0;

    let mut rb = RB.to_owned();
    let sys_user = SysUser {
        id: Some(user.id),
        gmt_create: None,
        gmt_modified: Some(FastDateTime::now()),
        status_id: Some(user.status_id),
        sort: Some(user.sort),
        user_no: None,
        mobile: Some(user.mobile),
        real_name: Some(user.real_name),
        remark: Some(user.remark),
        password: None,
    };

    let result = SysUser::update_by_column(&mut rb, &sys_user, "id").await;

    json!(&handle_result(result))
}


#[post("/user_delete", data = "<item>")]
pub async fn user_delete(item: Json<UserDeleteReq>, _auth: Token) -> Value {
    log::info!("user_delete params: {:?}", &item);
    let mut rb = RB.to_owned();

    let result = SysUser::delete_in_column(&mut rb, "id", &item.ids).await;

    json!(&handle_result(result))
}

#[post("/update_user_password", data = "<item>")]
pub async fn update_user_password(item: Json<UpdateUserPwdReq>, _auth: Token) -> Value {
    log::info!("update_user_pwd params: {:?}", &item);

    let user_pwd = item.0;

    let mut rb = RB.to_owned();

    let user_result = SysUser::select_by_id(&mut rb, &user_pwd.id).await;

    match user_result {
        Ok(user) => {
            let mut sys_user = user.unwrap();
            sys_user.password = Some(user_pwd.re_pwd);
            let result = SysUser::update_by_column(&mut rb, &sys_user, "id").await;

            json!(&handle_result(result))
        }
        Err(err) => {
            json!({"code":1,"msg":err.to_string()})
        }
    }
}
