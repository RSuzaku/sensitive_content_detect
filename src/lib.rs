pub mod base;
pub mod core;
pub mod init;
pub mod handlers;
pub mod model;
pub mod response;
pub mod routers;
pub mod utils;


use crate::{
    base::error::AppError,
    response::ResVO
};

use axum::Json;

pub type ResponseResult<T> = std::result::Result<Json<ResVO<T>>, AppError>;
pub type NewResult<T> = std::result::Result<T, AppError>;