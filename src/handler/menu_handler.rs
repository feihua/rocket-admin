use rocket::serde::json::serde_json::json;
use rocket::serde::json::{Json, Value};
use rbatis::rbdc::datetime::FastDateTime;
use rbatis::sql::{PageRequest};

use crate::model::entity::{SysMenu};
use crate::RB;
use crate::vo::handle_result;
use crate::vo::menu_vo::{*};


#[post("/menu_list", data = "<item>")]
pub async fn menu_list(item: Json<MenuListReq>) -> Value {
    log::info!("menu_list params: {:?}", &item);
    let mut rb = RB.to_owned();

    let result = SysMenu::select_page(&mut rb, &PageRequest::new(1, 1000)).await;

    let resp = match result {
        Ok(d) => {
            let total = d.total;

            let mut menu_list: Vec<MenuListData> = Vec::new();

            for x in d.records {
                menu_list.push(MenuListData {
                    id: x.id.unwrap(),
                    sort: x.sort.unwrap(),
                    status_id: x.status_id.unwrap(),
                    parent_id: x.parent_id.unwrap(),
                    menu_name: x.menu_name.as_ref().unwrap().to_string(),
                    label: x.menu_name.unwrap_or_default(),
                    menu_url: x.menu_url.unwrap_or_default(),
                    icon: x.menu_icon.unwrap_or_default(),
                    api_url: x.api_url.unwrap_or_default(),
                    remark: x.remark.unwrap_or_default(),
                    menu_type: x.menu_type.unwrap(),
                    create_time: x.gmt_create.unwrap().0.to_string(),
                    update_time: x.gmt_modified.unwrap().0.to_string(),
                })
            }
            MenuListResp {
                msg: "successful".to_string(),
                code: 0,
                total,
                data: Some(menu_list),
            }
        }
        Err(err) => {
            MenuListResp {
                msg: err.to_string(),
                code: 1,
                total: 0,
                data: None,
            }
        }
    };

    json!(&resp)
}

#[post("/menu_save", data = "<item>")]
pub async fn menu_save(item: Json<MenuSaveReq>) -> Value {
    log::info!("menu_save params: {:?}", &item);
    let mut rb = RB.to_owned();

    let menu = item.0;

    let role = SysMenu {
        id: None,
        gmt_create: Some(FastDateTime::now()),
        gmt_modified: None,
        status_id: Some(menu.status_id),
        sort: Some(menu.sort),
        parent_id: Some(menu.parent_id),
        menu_name: Some(menu.menu_name),
        menu_url: Some(menu.menu_url),
        api_url: Some(menu.api_url),
        menu_icon: Some(menu.icon),
        remark: Some(menu.remark),
        menu_type: Some(menu.menu_type),
    };

    let result = SysMenu::insert(&mut rb, &role).await;

    json!(&handle_result(result))
}

#[post("/menu_update", data = "<item>")]
pub async fn menu_update(item: Json<MenuUpdateReq>) -> Value {
    log::info!("menu_update params: {:?}", &item);
    let mut rb = RB.to_owned();
    let menu = item.0;

    let sys_menu = SysMenu {
        id: Some(menu.id),
        gmt_create: None,
        gmt_modified: Some(FastDateTime::now()),
        status_id: Some(menu.status_id),
        sort: Some(menu.sort),
        parent_id: Some(menu.parent_id),
        menu_name: Some(menu.menu_name),
        menu_url: Some(menu.menu_url),
        api_url: Some(menu.api_url),
        menu_icon: Some(menu.icon),
        remark: Some(menu.remark),
        menu_type: Some(menu.menu_type),
    };

    let result = SysMenu::update_by_column(&mut rb, &sys_menu, "id").await;

    json!(&handle_result(result))
}


#[post("/menu_delete", data = "<item>")]
pub async fn menu_delete(item: Json<MenuDeleteReq>) -> Value {
    log::info!("menu_delete params: {:?}", &item);
    let mut rb = RB.to_owned();

    let result = SysMenu::delete_in_column(&mut rb, "id", &item.ids).await;

    json!(&handle_result(result))
}