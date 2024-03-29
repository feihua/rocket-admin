use rbatis::rbdc::datetime::DateTime;
use rocket::serde::json::{Json, Value};
use rocket::serde::json::serde_json::json;

use crate::model::menu::SysMenu;
use crate::RB;
use crate::utils::auth::Token;
use crate::vo::{err_result_msg, err_result_page, handle_result, ok_result_page};
use crate::vo::menu_vo::{*};

// 查询菜单
#[post("/menu_list", data = "<item>")]
pub async fn menu_list(item: Json<MenuListReq>, _auth: Token) -> Value {
    log::info!("menu_list params: {:?}", &item);


    // 菜单是树形结构不需要分页
    let result = SysMenu::select_all(&mut RB.clone()).await;

    match result {
        Ok(sys_menu_list) => {
            let mut menu_list: Vec<MenuListData> = Vec::new();

            for menu in sys_menu_list {
                menu_list.push(MenuListData {
                    id: menu.id.unwrap(),
                    sort: menu.sort,
                    status_id: menu.status_id,
                    parent_id: menu.parent_id,
                    menu_name: menu.menu_name.clone(),
                    label: menu.menu_name,
                    menu_url: menu.menu_url.unwrap_or_default(),
                    icon: menu.menu_icon.unwrap_or_default(),
                    api_url: menu.api_url.unwrap_or_default(),
                    remark: menu.remark.unwrap_or_default(),
                    menu_type: menu.menu_type,
                    create_time: menu.create_time.unwrap().0.to_string(),
                    update_time: menu.update_time.unwrap().0.to_string(),
                })
            }

            json!(ok_result_page(menu_list, 0))
        }
        Err(err) => {
            json!(err_result_page(err.to_string()))
        }
    }
}

// 添加菜单
#[post("/menu_save", data = "<item>")]
pub async fn menu_save(item: Json<MenuSaveReq>, _auth: Token) -> Value {
    log::info!("menu_save params: {:?}", &item);


    let menu = item.0;

    let sys_menu = SysMenu {
        id: None,
        create_time: Some(DateTime::now()),
        update_time: Some(DateTime::now()),
        status_id: menu.status_id,
        sort: menu.sort,
        parent_id: menu.parent_id.unwrap_or(0),
        menu_name: menu.menu_name,
        menu_url: menu.menu_url,
        api_url: menu.api_url,
        menu_icon: menu.icon,
        remark: menu.remark,
        menu_type: menu.menu_type,
    };

    let result = SysMenu::insert(&mut RB.clone(), &sys_menu).await;

    json!(&handle_result(result))
}

// 更新菜单
#[post("/menu_update", data = "<item>")]
pub async fn menu_update(item: Json<MenuUpdateReq>, _auth: Token) -> Value {
    log::info!("menu_update params: {:?}", &item);

    let menu = item.0;

    let sys_menu = SysMenu {
        id: Some(menu.id),
        create_time: None,
        update_time: Some(DateTime::now()),
        status_id: menu.status_id,
        sort: menu.sort,
        parent_id: menu.parent_id,
        menu_name: menu.menu_name,
        menu_url: menu.menu_url,
        api_url: menu.api_url,
        menu_icon: menu.icon,
        remark: menu.remark,
        menu_type: menu.menu_type,
    };

    let result = SysMenu::update_by_column(&mut RB.clone(), &sys_menu, "id").await;

    json!(&handle_result(result))
}

// 删除菜单信息
#[post("/menu_delete", data = "<item>")]
pub async fn menu_delete(item: Json<MenuDeleteReq>, _auth: Token) -> Value {
    log::info!("menu_delete params: {:?}", &item);


    //有下级的时候 不能直接删除
    let menus = SysMenu::select_by_column(&mut RB.clone(), "parent_id", &item.id).await.unwrap_or_default();

    if menus.len() > 0 {
        return json!(err_result_msg("有下级菜单,不能直接删除".to_string()));
    }

    let result = SysMenu::delete_by_column(&mut RB.clone(), "id", &item.id).await;

    json!(&handle_result(result))
}