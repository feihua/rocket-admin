use rocket::http::ContentType;
use rocket::response::Responder;
use rocket::serde::json::serde_json;
use rocket::{response, Request, Response};
use std::io::Cursor;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    // #[error("Failed to complete an HTTP request")]
    // Http { #[from] source: reqwest::Error },
    //
    #[error("Failed to read the cache file")]
    DiskCacheRead { source: std::io::Error },
    //
    // #[error("Failed to update the cache file")]
    // DiskCacheWrite { source: std::io::Error },
    #[error("")]
    JwtTokenError(String),

    #[error("数据库错误: {0}")]
    DbError(#[from] rbatis::Error),

    #[error("业务异常: {0}")]
    BusinessError(&'static str),
}
pub type AppResult<T> = Result<T, AppError>;

impl<'r> Responder<'r, 'static> for AppError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        // let error_msg = format!("{}", self);
        let error_msg = serde_json::json!({
            "msg": self.to_string(),
            "code": 1,
        })
        .to_string();
        Response::build()
            .header(ContentType::JSON)
            // .status(match self {
            //     AppError::BusinessError(_) => rocket::http::Status::BadRequest,
            //     AppError::JwtTokenError(_) => rocket::http::Status::Unauthorized,
            //     _ => rocket::http::Status::InternalServerError,
            // })
            .sized_body(error_msg.len(), Cursor::new(error_msg))
            .ok()
    }
}
