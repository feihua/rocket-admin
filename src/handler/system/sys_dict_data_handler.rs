use rbatis::plugin::page::PageRequest;
use rocket::serde::json::{Json, Value};

use crate::common::result::BaseResponse;
use crate::middleware::auth::Token;
use crate::model::system::sys_dict_data_model::DictData;
use crate::utils::time_util::time_to_string;
use crate::vo::system::sys_dict_data_vo::*;
use crate::RB;
use rbs::value;

/*
 *添加字典数据表
 *author：刘飞华
 *date：2025/01/09 16:16:41
 */
#[post("/system/dictData/addDictData", data = "<item>")]
pub async fn add_sys_dict_data(item: Json<AddDictDataReq>, _auth: Token) -> Value {
    log::info!("add sys_dict_data params: {:?}", &item);
    let rb = &mut RB.clone();
    let req = item.0;

    let res_by_dict_label =
        DictData::select_by_dict_label(rb, &req.dict_type, &req.dict_label).await;
    match res_by_dict_label {
        Ok(r) => {
            if r.is_some() {
                return BaseResponse::<String>::err_result_msg(
                    "新增字典数据失败,字典标签已存在".to_string(),
                );
            }
        }
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
    }

    let res_by_dict_value =
        DictData::select_by_dict_value(rb, &req.dict_type, &req.dict_value).await;
    match res_by_dict_value {
        Ok(r) => {
            if r.is_some() {
                return BaseResponse::<String>::err_result_msg(
                    "新增字典数据失败,字典键值已存在".to_string(),
                );
            }
        }
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
    }

    let sys_dict_data = DictData {
        dict_code: None,                        //字典编码
        dict_sort: req.dict_sort,               //字典排序
        dict_label: req.dict_label,             //字典标签
        dict_value: req.dict_value,             //字典键值
        dict_type: req.dict_type,               //字典类型
        css_class: req.css_class,               //样式属性（其他样式扩展）
        list_class: req.list_class,             //格回显样式
        is_default: req.is_default,             //是否默认（Y是 N否）
        status: req.status,                     //状态（0：停用，1:正常）
        remark: req.remark.unwrap_or_default(), //备注
        create_time: None,                      //创建时间
        update_time: None,                      //修改时间
    };

    let result = DictData::insert(rb, &sys_dict_data).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *删除字典数据表
 *author：刘飞华
 *date：2025/01/09 16:16:41
 */
#[post("/system/dictData/deleteDictData", data = "<item>")]
pub async fn delete_sys_dict_data(item: Json<DeleteDictDataReq>, _auth: Token) -> Value {
    log::info!("delete sys_dict_data params: {:?}", &item);
    let rb = &mut RB.clone();

    let result = DictData::delete_by_map(rb, value! {"id": &item.ids}).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *更新字典数据表
 *author：刘飞华
 *date：2025/01/09 16:16:41
 */
#[post("/system/dictData/updateDictData", data = "<item>")]
pub async fn update_sys_dict_data(item: Json<UpdateDictDataReq>, _auth: Token) -> Value {
    log::info!("update sys_dict_data params: {:?}", &item);
    let rb = &mut RB.clone();
    let req = item.0;

    let result = DictData::select_by_id(rb, &req.dict_code).await;
    match result {
        Ok(r) => {
            if r.is_none() {
                return BaseResponse::<String>::err_result_msg(
                    "更新字典数据失败,字典数据不存在".to_string(),
                );
            }
        }
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
    }

    let res_by_dict_label =
        DictData::select_by_dict_label(rb, &req.dict_type, &req.dict_label).await;
    match res_by_dict_label {
        Ok(r) => {
            if r.is_some() && r.clone().unwrap().dict_code.unwrap_or_default() != req.dict_code {
                return BaseResponse::<String>::err_result_msg(
                    "新增字典数据失败,字典标签已存在".to_string(),
                );
            }
        }
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
    }

    let res_by_dict_value =
        DictData::select_by_dict_value(rb, &req.dict_type, &req.dict_value).await;
    match res_by_dict_value {
        Ok(r) => {
            if r.is_some() && r.clone().unwrap().dict_code.unwrap_or_default() != req.dict_code {
                return BaseResponse::<String>::err_result_msg(
                    "新增字典数据失败,字典键值已存在".to_string(),
                );
            }
        }
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
    }

    let sys_dict_data = DictData {
        dict_code: Some(req.dict_code),         //字典编码
        dict_sort: req.dict_sort,               //字典排序
        dict_label: req.dict_label,             //字典标签
        dict_value: req.dict_value,             //字典键值
        dict_type: req.dict_type,               //字典类型
        css_class: req.css_class,               //样式属性（其他样式扩展）
        list_class: req.list_class,             //格回显样式
        is_default: req.is_default,             //是否默认（Y是 N否）
        status: req.status,                     //状态（0：停用，1:正常）
        remark: req.remark.unwrap_or_default(), //备注
        create_time: None,                      //创建时间
        update_time: None,                      //修改时间
    };

    let result =
        DictData::update_by_map(rb, &sys_dict_data, value! {"dict_code": &req.dict_code}).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *更新字典数据表状态
 *author：刘飞华
 *date：2025/01/09 16:16:41
 */
#[post("/system/dictData/updateDictDataStatus", data = "<item>")]
pub async fn update_sys_dict_data_status(
    item: Json<UpdateDictDataStatusReq>,
    _auth: Token,
) -> Value {
    log::info!("update sys_dict_data_status params: {:?}", &item);
    let rb = &mut RB.clone();
    let req = item.0;

    let update_sql = format!(
        "update sys_dict_data set status = ? where id in ({})",
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
 *查询字典数据表详情
 *author：刘飞华
 *date：2025/01/09 16:16:41
 */
#[post("/system/dictData/queryDictDataDetail", data = "<item>")]
pub async fn query_sys_dict_data_detail(item: Json<QueryDictDataDetailReq>, _auth: Token) -> Value {
    log::info!("query sys_dict_data_detail params: {:?}", &item);
    let rb = &mut RB.clone();

    let result = DictData::select_by_id(rb, &item.id).await;

    match result {
        Ok(opt_sys_dict_data) => {
            if opt_sys_dict_data.is_none() {
                return BaseResponse::<QueryDictDataDetailResp>::err_result_data(
                    QueryDictDataDetailResp::new(),
                    "字典数据表不存在".to_string(),
                );
            }
            let x = opt_sys_dict_data.unwrap();

            let sys_dict_data = QueryDictDataDetailResp {
                dict_code: x.dict_code.unwrap_or_default(), //字典编码
                dict_sort: x.dict_sort,                     //字典排序
                dict_label: x.dict_label,                   //字典标签
                dict_value: x.dict_value,                   //字典键值
                dict_type: x.dict_type,                     //字典类型
                css_class: x.css_class,                     //样式属性（其他样式扩展）
                list_class: x.list_class,                   //格回显样式
                is_default: x.is_default,                   //是否默认（Y是 N否）
                status: x.status,                           //状态（0：停用，1:正常）
                remark: x.remark,                           //备注
                create_time: time_to_string(x.create_time), //创建时间
                update_time: time_to_string(x.update_time), //修改时间
            };

            BaseResponse::<QueryDictDataDetailResp>::ok_result_data(sys_dict_data)
        }
        Err(err) => BaseResponse::<QueryDictDataDetailResp>::err_result_data(
            QueryDictDataDetailResp::new(),
            err.to_string(),
        ),
    }
}

/*
 *查询字典数据表列表
 *author：刘飞华
 *date：2025/01/09 16:16:41
 */
#[post("/system/dictData/queryDictDataList", data = "<item>")]
pub async fn query_sys_dict_data_list(item: Json<QueryDictDataListReq>, _auth: Token) -> Value {
    log::info!("query sys_dict_data_list params: {:?}", &item);
    let rb = &mut RB.clone();

    let dict_label = item.dict_label.as_deref().unwrap_or_default(); //字典标签
    let dict_type = item.dict_type.as_deref().unwrap_or_default(); //字典类型
    let status = item.status.unwrap_or(2); //状态（0：停用，1:正常）

    let page = &PageRequest::new(item.page_no, item.page_size);
    let result = DictData::select_dict_data_list(rb, page, dict_label, dict_type, status).await;

    let mut sys_dict_data_list: Vec<DictDataListDataResp> = Vec::new();
    match result {
        Ok(d) => {
            let total = d.total;

            for x in d.records {
                sys_dict_data_list.push(DictDataListDataResp {
                    dict_code: x.dict_code.unwrap_or_default(), //字典编码
                    dict_sort: x.dict_sort,                     //字典排序
                    dict_label: x.dict_label,                   //字典标签
                    dict_value: x.dict_value,                   //字典键值
                    dict_type: x.dict_type,                     //字典类型
                    css_class: x.css_class,                     //样式属性（其他样式扩展）
                    list_class: x.list_class,                   //格回显样式
                    is_default: x.is_default,                   //是否默认（Y是 N否）
                    status: x.status,                           //状态（0：停用，1:正常）
                    remark: x.remark,                           //备注
                    create_time: time_to_string(x.create_time), //创建时间
                    update_time: time_to_string(x.update_time), //修改时间
                })
            }

            BaseResponse::ok_result_page(sys_dict_data_list, total)
        }
        Err(err) => BaseResponse::err_result_page(DictDataListDataResp::new(), err.to_string()),
    }
}
