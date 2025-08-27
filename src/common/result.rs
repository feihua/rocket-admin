use crate::common::error::AppResult;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
use serde::Serialize;
use std::fmt::Debug;

// 统一返回vo
#[derive(Serialize, Debug, Clone)]
pub struct BaseResponse<T>
where
    T: Serialize + Debug,
{
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

#[derive(Serialize, Debug, Clone)]
pub struct ResponsePage<T>
where
    T: Serialize + Debug,
{
    pub code: i32,
    pub msg: String,
    pub total: u64,
    pub success: bool,
    pub data: Option<T>,
}

pub fn ok_result() -> AppResult<Value> {
    Ok(json!(BaseResponse {
        msg: "操作成功".to_string(),
        code: 0,
        data: Some("None".to_string()),
    }))
}

pub fn ok_result_msg(msg: &str) -> AppResult<Value> {
    Ok(json!(BaseResponse {
        msg: msg.to_string(),
        code: 0,
        data: Some("None".to_string()),
    }))
}

pub fn ok_result_data<T: Serialize + Debug>(data: T) -> AppResult<Value> {
    Ok(json!(BaseResponse {
        msg: "操作成功".to_string(),
        code: 0,
        data: Some(data),
    }))
}

pub fn err_result_msg(msg: String) -> AppResult<Value> {
    Ok(json!(BaseResponse {
        msg: msg.to_string(),
        code: 1,
        data: Some("None".to_string()),
    }))
}

pub fn ok_result_page<T: Serialize + Debug>(data: T, total: u64) -> AppResult<Value> {
    Ok(json!(ResponsePage {
        msg: "操作成功".to_string(),
        code: 0,
        success: true,
        data: Some(data),
        total,
    }))
}
