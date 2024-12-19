use rbatis::plugin::page::PageRequest;
use rbatis::rbdc::datetime::DateTime;
use rocket::serde::json::{Json, Value};

use crate::common::result::BaseResponse;
use crate::middleware::auth::Token;
use crate::model::system::sys_menu_model::Menu;
use crate::model::system::sys_role_menu_model::{query_menu_by_role, RoleMenu};
use crate::model::system::sys_role_model::Role;
use crate::model::system::sys_user_role_model::UserRole;
use crate::vo::system::sys_role_vo::*;
use crate::RB;
use rbs::to_value;

/*
 *添加角色信息
 *author：刘飞华
 *date：2024/12/16 14:51:10
 */
#[post("/addRole", data = "<item>")]
pub async fn add_sys_role(item: Json<AddRoleReq>, _auth: Token) -> Value {
    log::info!("add sys_role params: {:?}", &item);

    let req = item.0;

    let sys_role = Role {
        id: None,                 //主键
        role_name: req.role_name, //名称
        status_id: req.status_id, //状态(1:正常，0:禁用)
        sort: req.sort,           //排序
        remark: req.remark,       //备注
        create_time: None,        //创建时间
        update_time: None,        //修改时间
    };

    let result = Role::insert(&mut RB.clone(), &sys_role).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *删除角色信息
 *author：刘飞华
 *date：2024/12/16 14:51:10
 */
#[post("/deleteRole", data = "<item>")]
pub async fn delete_sys_role(item: Json<DeleteRoleReq>, _auth: Token) -> Value {
    log::info!("delete sys_role params: {:?}", &item);

    let ids = item.ids.clone();
    let user_role_list = UserRole::select_in_column(&mut RB.clone(), "role_id", &ids)
        .await
        .unwrap_or_default();

    if user_role_list.len() > 0 {
        return BaseResponse::<String>::err_result_msg("角色已被使用,不能直接删除".to_string());
    }

    let result = Role::delete_in_column(&mut RB.clone(), "id", &item.ids).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *更新角色信息
 *author：刘飞华
 *date：2024/12/16 14:51:10
 */
#[post("/updateRole", data = "<item>")]
pub async fn update_sys_role(item: Json<UpdateRoleReq>, _auth: Token) -> Value {
    log::info!("update sys_role params: {:?}", &item);

    let req = item.0;

    let sys_role = Role {
        id: Some(req.id),         //主键
        role_name: req.role_name, //名称
        status_id: req.status_id, //状态(1:正常，0:禁用)
        sort: req.sort,           //排序
        remark: req.remark,       //备注
        create_time: None,        //创建时间
        update_time: None,        //修改时间
    };

    let result = Role::update_by_column(&mut RB.clone(), &sys_role, "id").await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *更新角色信息状态
 *author：刘飞华
 *date：2024/12/16 14:51:10
 */
#[post("/updateRoleStatus", data = "<item>")]
pub async fn update_sys_role_status(item: Json<UpdateRoleStatusReq>, _auth: Token) -> Value {
    log::info!("update sys_role_status params: {:?}", &item);

    let req = item.0;

    let param = vec![to_value!(req.status), to_value!(req.ids)];
    let result = &mut RB
        .clone()
        .exec("update sys_role set status = ? where id in ?", param)
        .await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *查询角色信息详情
 *author：刘飞华
 *date：2024/12/16 14:51:10
 */
#[post("/queryRoleDetail", data = "<item>")]
pub async fn query_sys_role_detail(item: Json<QueryRoleDetailReq>, _auth: Token) -> Value {
    log::info!("query sys_role_detail params: {:?}", &item);

    let result = Role::select_by_id(&mut RB.clone(), &item.id).await;

    match result {
        Ok(d) => {
            let x = d.unwrap();

            let sys_role = QueryRoleDetailResp {
                id: x.id.unwrap(),                      //主键
                role_name: x.role_name,                 //名称
                status_id: x.status_id,                 //状态(1:正常，0:禁用)
                sort: x.sort,                           //排序
                remark: x.remark,                       //备注
                create_time: x.create_time.unwrap().0.to_string(), //创建时间
                update_time: x.update_time.unwrap().0.to_string(), //修改时间
            };

            BaseResponse::<QueryRoleDetailResp>::ok_result_data(sys_role)
        }
        Err(err) => BaseResponse::<QueryRoleDetailResp>::err_result_data(
            QueryRoleDetailResp::new(),
            err.to_string(),
        ),
    }
}

/*
 *查询角色信息列表
 *author：刘飞华
 *date：2024/12/16 14:51:10
 */
#[post("/queryRoleList", data = "<item>")]
pub async fn query_sys_role_list(item: Json<QueryRoleListReq>, _auth: Token) -> Value {
    log::info!("query sys_role_list params: {:?}", &item);

    let role_name = item.role_name.as_deref().unwrap_or_default();
    let status_id = item.status_id.unwrap_or_default();
    let page = &PageRequest::new(item.page_no.clone(), item.page_size.clone());
    let result = Role::select_page_by_name(&mut RB.clone(), page, role_name, status_id).await;

    let mut sys_role_list_data: Vec<RoleListDataResp> = Vec::new();
    match result {
        Ok(d) => {
            let total = d.total;

            for x in d.records {
                let sys_role = RoleListDataResp {
                    id: x.id.unwrap(),                                 //主键
                    role_name: x.role_name,                            //名称
                    status_id: x.status_id,                            //状态(1:正常，0:禁用)
                    sort: x.sort,                                      //排序
                    remark: x.remark,                                  //备注
                    create_time: x.create_time.unwrap().0.to_string(), //创建时间
                    update_time: x.update_time.unwrap().0.to_string(), //修改时间
                };
                sys_role_list_data.push(sys_role)
            }

            BaseResponse::<Vec<RoleListDataResp>>::ok_result_page(sys_role_list_data, total)
        }
        Err(err) => BaseResponse::<Vec<RoleListDataResp>>::err_result_page(
            RoleListDataResp::new(),
            err.to_string(),
        ),
    }
}

/*
 *查询角色关联的菜单
 *author：刘飞华
 *date：2024/12/16 14:51:10
 */
#[post("/query_role_menu", data = "<item>")]
pub async fn query_role_menu(item: Json<QueryRoleMenuReq>, _auth: Token) -> Value {
    log::info!("query_role_menu params: {:?}", &item);

    // 查询所有菜单
    let menu_list = Menu::select_all(&mut RB.clone()).await.unwrap_or_default();

    let mut menu_data_list: Vec<MenuDataList> = Vec::new();
    let mut role_menu_ids: Vec<i64> = Vec::new();

    for y in menu_list {
        let x = y.clone();
        menu_data_list.push(MenuDataList {
            id: x.id.unwrap(),
            parent_id: x.parent_id,
            title: x.menu_name,
            key: y.id.unwrap().to_string(),
            label: y.menu_name,
            is_penultimate: y.parent_id == 2,
        });
        role_menu_ids.push(x.id.unwrap())
    }

    //不是超级管理员的时候,就要查询角色和菜单的关联
    if item.role_id != 1 {
        role_menu_ids.clear();
        let role_menu_list = query_menu_by_role(&mut RB.clone(), item.role_id.clone())
            .await
            .unwrap_or_default();

        for x in role_menu_list {
            let m_id = x.get("menu_id").unwrap().clone();
            role_menu_ids.push(m_id)
        }
    }

    BaseResponse::<QueryRoleMenuData>::ok_result_data(QueryRoleMenuData {
        menu_ids: role_menu_ids,
        menu_list: menu_data_list,
    })
}

/*
 *更新角色关联的菜单
 *author：刘飞华
 *date：2024/12/16 14:51:10
 */
#[post("/update_role_menu", data = "<item>")]
pub async fn update_role_menu(item: Json<UpdateRoleMenuReq>, _auth: Token) -> Value {
    log::info!("update_role_menu params: {:?}", &item);
    let role_id = item.role_id.clone();

    let role_menu_result = RoleMenu::delete_by_column(&mut RB.clone(), "role_id", &role_id).await;

    match role_menu_result {
        Ok(_) => {
            let mut menu_role: Vec<RoleMenu> = Vec::new();

            for id in &item.menu_ids {
                let menu_id = id.clone();
                menu_role.push(RoleMenu {
                    id: None,
                    create_time: Some(DateTime::now()),
                    menu_id,
                    role_id: role_id.clone(),
                })
            }

            let result =
                RoleMenu::insert_batch(&mut RB.clone(), &menu_role, item.menu_ids.len() as u64)
                    .await;

            match result {
                Ok(_u) => BaseResponse::<String>::ok_result(),
                Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
            }
        }
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}