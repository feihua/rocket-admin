use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
use serde::Serialize;
use std::fmt::Debug;
// 统一返回分页
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

impl<T> ResponsePage<T>
where
    T: Serialize + Debug + Send,
{
    pub fn ok_result_page(data: T, total: u64) -> Value {
        json!(ResponsePage {
            msg: "操作成功".to_string(),
            code: 0,
            success: true,
            data: Some(data),
            total,
        })
    }

    pub fn ok_result(data: T) -> Value {
        json!(ResponsePage {
            msg: "操作成功".to_string(),
            code: 0,
            success: true,
            data: Some(data),
            total: 0,
        })
    }

    pub fn err_result_page(data: T, msg: String) -> Value {
        json!(ResponsePage {
            msg: msg.to_string(),
            code: 1,
            success: false,
            data: Some(data),
            total: 0,
        })
    }
}
