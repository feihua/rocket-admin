use rocket::serde::json::serde_json::json;
use rocket::serde::json::{Json, Value};
use rbatis::rbdc::datetime::DateTime;
use rbatis::sql::{PageRequest};

use crate::model::menu::{SysMenu};
use crate::RB;
use crate::utils::auth::Token;
use crate::vo::handle_result;
use crate::vo::menu_vo::{*};


#[post("/menu_list", data = "<item>")]
pub async fn menu_list(item: Json<MenuListReq>, _auth: Token) -> Value {
    log::info!("menu_list params: {:?}", &item);
    let mut rb = RB.to_owned();

    let result = SysMenu::select_page(&mut rb, &PageRequest::new(1, 1000)).await;

    match result {
        Ok(d) => {
            let total = d.total;

            let mut menu_list: Vec<MenuListData> = Vec::new();

            for x in d.records {
                menu_list.push(MenuListData {
                    id: x.id.unwrap(),
                    sort: x.sort,
                    status_id: x.status_id,
                    parent_id: x.parent_id,
                    menu_name: x.menu_name.clone(),
                    label: x.menu_name,
                    menu_url: x.menu_url.unwrap_or_default(),
                    icon: x.menu_icon.unwrap_or_default(),
                    api_url: x.api_url.unwrap_or_default(),
                    remark: x.remark.unwrap_or_default(),
                    menu_type: x.menu_type,
                    create_time: x.create_time.unwrap().0.to_string(),
                    update_time: x.update_time.unwrap().0.to_string(),
                })
            }

            json!(&MenuListResp {
                msg: "successful".to_string(),
                code: 0,
                total,
                data: Some(menu_list),
            })
        }
        Err(err) => {
            json!({"code":1,"msg":err.to_string()})
        }
    }
}

#[post("/menu_save", data = "<item>")]
pub async fn menu_save(item: Json<MenuSaveReq>, _auth: Token) -> Value {
    log::info!("menu_save params: {:?}", &item);
    let mut rb = RB.to_owned();

    let menu = item.0;

    let role = SysMenu {
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

    let result = SysMenu::insert(&mut rb, &role).await;

    json!(&handle_result(result))
}

#[post("/menu_update", data = "<item>")]
pub async fn menu_update(item: Json<MenuUpdateReq>, _auth: Token) -> Value {
    log::info!("menu_update params: {:?}", &item);
    let mut rb = RB.to_owned();
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

    let result = SysMenu::update_by_column(&mut rb, &sys_menu, "id").await;

    json!(&handle_result(result))
}


#[post("/menu_delete", data = "<item>")]
pub async fn menu_delete(item: Json<MenuDeleteReq>, _auth: Token) -> Value {
    log::info!("menu_delete params: {:?}", &item);
    let mut rb = RB.to_owned();

    let result = SysMenu::delete_by_column(&mut rb, "id", &item.id).await;

    json!(&handle_result(result))
}