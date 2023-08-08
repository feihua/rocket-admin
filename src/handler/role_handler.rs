use rocket::serde::json::serde_json::json;
use rocket::serde::json::{Json, Value};
use rbatis::rbdc::datetime::DateTime;
use rbatis::sql::{PageRequest};
use crate::model::role::{SysRole};
use crate::model::menu::{SysMenu};
use crate::model::role_menu::{query_menu_by_role, SysRoleMenu};
use crate::RB;
use crate::utils::auth::Token;
use crate::vo::handle_result;
use crate::vo::role_vo::*;


#[post("/role_list", data = "<item>")]
pub async fn role_list(item: Json<RoleListReq>, _auth: Token) -> Value {
    log::info!("role_list params: {:?}", &item);
    let mut rb = RB.to_owned();

    let role_name = item.role_name.as_deref().unwrap_or_default();
    let status_id = item.status_id.as_deref().unwrap_or_default();

    let page = &PageRequest::new(item.page_no, item.page_size);
    let result = SysRole::select_page_by_name(&mut rb, page, role_name, status_id).await;

    match result {
        Ok(d) => {
            let total = d.total;

            let mut role_list: Vec<RoleListData> = Vec::new();

            for x in d.records {
                role_list.push(RoleListData {
                    id: x.id.unwrap(),
                    sort: x.sort,
                    status_id: x.status_id,
                    role_name: x.role_name,
                    remark: x.remark.unwrap_or_default(),
                    create_time: x.create_time.unwrap().0.to_string(),
                    update_time: x.update_time.unwrap().0.to_string(),
                })
            }

            json!(&RoleListResp {
                msg: "successful".to_string(),
                code: 0,
                success: true,
                total,
                data: Some(role_list),
            })
        }
        Err(err) => {
            json!({"code":1,"msg":err.to_string()})
        }
    }
}


#[post("/role_save", data = "<item>")]
pub async fn role_save(item: Json<RoleSaveReq>, _auth: Token) -> Value {
    println!("model: {:?}", &item);
    let mut rb = RB.to_owned();

    let role = item.0;

    let sys_role = SysRole {
        id: None,
        create_time: Some(DateTime::now()),
        update_time: Some(DateTime::now()),
        status_id: role.status_id,
        sort: role.sort,
        role_name: role.role_name,
        remark: role.remark,
    };

    let result = SysRole::insert(&mut rb, &sys_role).await;

    json!(&handle_result(result))
}


#[post("/role_update", data = "<item>")]
pub async fn role_update(item: Json<RoleUpdateReq>, _auth: Token) -> Value {
    println!("item: {:?}", &item);
    let mut rb = RB.to_owned();
    let role = item.0;

    let sys_role = SysRole {
        id: Some(role.id),
        create_time: None,
        update_time: Some(DateTime::now()),
        status_id: role.status_id,
        sort: role.sort,
        role_name: role.role_name,
        remark: role.remark,
    };

    let result = SysRole::update_by_column(&mut rb, &sys_role, "id").await;

    json!(&handle_result(result))
}


#[post("/role_delete", data = "<item>")]
pub async fn role_delete(item: Json<RoleDeleteReq>, _auth: Token) -> Value {
    println!("item: {:?}", &item);
    let mut rb = RB.to_owned();

    let result = SysRole::delete_in_column(&mut rb, "id", &item.ids).await;

    json!(&handle_result(result))
}


#[post("/query_role_menu", data = "<item>")]
pub async fn query_role_menu(item: Json<QueryRoleMenuReq>, _auth: Token) -> Value {
    log::info!("query_role_menu params: {:?}", &item);
    let mut rb = RB.to_owned();

    // 查询所有菜单
    let menu_list = SysMenu::select_all(&mut rb).await.unwrap_or_default();

    let mut menu_data_list: Vec<MenuDataList> = Vec::new();
    let mut role_menu_ids: Vec<i32> = Vec::new();

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
        let role_menu_list = query_menu_by_role(&mut rb, item.role_id.clone()).await.unwrap_or_default();

        for x in role_menu_list {
            let m_id = x.get("menu_id").unwrap().clone();
            role_menu_ids.push(m_id)
        }
    }

    let resp = QueryRoleMenuResp {
        msg: "successful".to_string(),
        code: 0,
        data: QueryRoleMenuData {
            role_menus: role_menu_ids,
            menu_list: menu_data_list,
        },
    };

    json!(resp)
}


#[post("/update_role_menu", data = "<item>")]
pub async fn update_role_menu(item: Json<UpdateRoleMenuReq>, _auth: Token) -> Value {
    log::info!("update_role_menu params: {:?}", &item);
    let role_id = item.role_id.clone();

    let mut rb = RB.to_owned();

    SysRoleMenu::delete_by_column(&mut rb, "role_id", &role_id).await.expect("删除角色菜单异常");

    let mut menu_role: Vec<SysRoleMenu> = Vec::new();

    for x in &item.menu_ids {
        let menu_id = x.clone();
        menu_role.push(SysRoleMenu {
            id: None,
            create_time: Some(DateTime::now()),
            update_time: Some(DateTime::now()),
            status_id: 1,
            sort: 1,
            menu_id,
            role_id: role_id.clone(),
        })
    }

    let result = SysRoleMenu::insert_batch(&mut rb, &menu_role, item.menu_ids.len() as u64).await;

    json!(&handle_result(result))
}
