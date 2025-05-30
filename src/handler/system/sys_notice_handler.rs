use rbatis::plugin::page::PageRequest;
use rocket::serde::json::{Json, Value};

use crate::common::result::BaseResponse;
use crate::middleware::auth::Token;
use crate::model::system::sys_notice_model::Notice;
use crate::utils::time_util::time_to_string;
use crate::vo::system::sys_notice_vo::*;
use crate::RB;
use rbs::value;

/*
 *添加通知公告表
 *author：刘飞华
 *date：2025/01/09 16:16:41
 */
#[post("/system/notice/addNotice", data = "<item>")]
pub async fn add_sys_notice(item: Json<AddNoticeReq>, _auth: Token) -> Value {
    log::info!("add sys_notice params: {:?}", &item);
    let rb = &mut RB.clone();
    let req = item.0;

    let res = Notice::select_by_title(rb, &req.notice_title).await;
    match res {
        Ok(r) => {
            if r.is_some() {
                return BaseResponse::<String>::err_result_msg("公告标题已存在".to_string());
            }
        }
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
    }

    let sys_notice = Notice {
        id: None,                               //公告ID
        notice_title: req.notice_title,         //公告标题
        notice_type: req.notice_type,           //公告类型（1:通知,2:公告）
        notice_content: req.notice_content,     //公告内容
        status: req.status,                     //公告状态（0:关闭,1:正常 ）
        remark: req.remark.unwrap_or_default(), //备注
        create_time: None,                      //创建时间
        update_time: None,                      //修改时间
    };

    let result = Notice::insert(rb, &sys_notice).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *删除通知公告表
 *author：刘飞华
 *date：2025/01/09 16:16:41
 */
#[post("/system/notice/deleteNotice", data = "<item>")]
pub async fn delete_sys_notice(item: Json<DeleteNoticeReq>, _auth: Token) -> Value {
    log::info!("delete sys_notice params: {:?}", &item);
    let rb = &mut RB.clone();

    let result = Notice::delete_by_map(rb, value! {"id": &item.ids}).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *更新通知公告表
 *author：刘飞华
 *date：2025/01/09 16:16:41
 */
#[post("/system/notice/updateNotice", data = "<item>")]
pub async fn update_sys_notice(item: Json<UpdateNoticeReq>, _auth: Token) -> Value {
    log::info!("update sys_notice params: {:?}", &item);
    let rb = &mut RB.clone();
    let req = item.0;

    let result = Notice::select_by_id(rb, &req.id).await;

    match result {
        Ok(d) => {
            if d.is_none() {
                return BaseResponse::<String>::err_result_msg("通知公告表不存在".to_string());
            }
        }
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
    };

    let res = Notice::select_by_title(rb, &req.notice_title).await;

    match res {
        Ok(r) => {
            if r.is_some() && r.unwrap().id.unwrap_or_default() != req.id {
                return BaseResponse::<String>::err_result_msg("公告标题已存在".to_string());
            }
        }
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
    }

    let sys_notice = Notice {
        id: Some(req.id),                       //公告ID
        notice_title: req.notice_title,         //公告标题
        notice_type: req.notice_type,           //公告类型（1:通知,2:公告）
        notice_content: req.notice_content,     //公告内容
        status: req.status,                     //公告状态（0:关闭,1:正常 ）
        remark: req.remark.unwrap_or_default(), //备注
        create_time: None,                      //创建时间
        update_time: None,                      //修改时间
    };

    let result = Notice::update_by_map(rb, &sys_notice, value! {"id": &req.id}).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *更新通知公告表状态
 *author：刘飞华
 *date：2025/01/09 16:16:41
 */
#[post("/system/notice/updateNoticeStatus", data = "<item>")]
pub async fn update_sys_notice_status(item: Json<UpdateNoticeStatusReq>, _auth: Token) -> Value {
    log::info!("update sys_notice_status params: {:?}", &item);
    let rb = &mut RB.clone();
    let req = item.0;

    let update_sql = format!(
        "update sys_notice set status = ? where id in ({})",
        req.ids
            .iter()
            .map(|_| "?")
            .collect::<Vec<&str>>()
            .join(", ")
    );

    let mut param = vec![value!(req.status)];
    param.extend(req.ids.iter().map(|&id| value!(id)));
    let result = rb.exec(&update_sql, param).await;
    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *查询通知公告表详情
 *author：刘飞华
 *date：2025/01/09 16:16:41
 */
#[post("/system/notice/queryNoticeDetail", data = "<item>")]
pub async fn query_sys_notice_detail(item: Json<QueryNoticeDetailReq>, _auth: Token) -> Value {
    log::info!("query sys_notice_detail params: {:?}", &item);
    let rb = &mut RB.clone();

    let result = Notice::select_by_id(rb, &item.id).await;

    match result {
        Ok(opt_sys_notice) => {
            if opt_sys_notice.is_none() {
                return BaseResponse::<QueryNoticeDetailResp>::err_result_data(
                    QueryNoticeDetailResp::new(),
                    "通知公告表不存在".to_string(),
                );
            }
            let x = opt_sys_notice.unwrap();

            let sys_notice = QueryNoticeDetailResp {
                id: x.id.unwrap_or_default(),               //公告ID
                notice_title: x.notice_title,               //公告标题
                notice_type: x.notice_type,                 //公告类型（1:通知,2:公告）
                notice_content: x.notice_content,           //公告内容
                status: x.status,                           //公告状态（0:关闭,1:正常 ）
                remark: x.remark,                           //备注
                create_time: time_to_string(x.create_time), //创建时间
                update_time: time_to_string(x.update_time), //修改时间
            };

            BaseResponse::<QueryNoticeDetailResp>::ok_result_data(sys_notice)
        }
        Err(err) => BaseResponse::<QueryNoticeDetailResp>::err_result_data(
            QueryNoticeDetailResp::new(),
            err.to_string(),
        ),
    }
}

/*
 *查询通知公告表列表
 *author：刘飞华
 *date：2025/01/09 16:16:41
 */
#[post("/system/notice/queryNoticeList", data = "<item>")]
pub async fn query_sys_notice_list(item: Json<QueryNoticeListReq>, _auth: Token) -> Value {
    log::info!("query sys_notice_list params: {:?}", &item);
    let rb = &mut RB.clone();

    let notice_title = item.notice_title.as_deref().unwrap_or_default();
    let notice_type = item.notice_type.unwrap_or(0); //公告类型（1:通知,2:公告）
    let status = item.status.unwrap_or(2); //公告状态（0:关闭,1:正常 ）

    let page = &PageRequest::new(item.page_no, item.page_size);
    let result = Notice::select_sys_notice_list(rb, page, notice_title, notice_type, status).await;

    let mut data: Vec<NoticeListDataResp> = Vec::new();
    match result {
        Ok(d) => {
            let total = d.total;

            for x in d.records {
                data.push(NoticeListDataResp {
                    id: x.id.unwrap_or_default(),               //公告ID
                    notice_title: x.notice_title,               //公告标题
                    notice_type: x.notice_type,                 //公告类型（1:通知,2:公告）
                    notice_content: x.notice_content,           //公告内容
                    status: x.status,                           //公告状态（0:关闭,1:正常 ）
                    remark: x.remark,                           //备注
                    create_time: time_to_string(x.create_time), //创建时间
                    update_time: time_to_string(x.update_time), //修改时间
                })
            }

            BaseResponse::ok_result_page(data, total)
        }
        Err(err) => BaseResponse::err_result_page(NoticeListDataResp::new(), err.to_string()),
    }
}
