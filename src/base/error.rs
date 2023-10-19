use axum::{
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
};

use std::str::FromStr;
use std::num::ParseIntError;

// 应用程序错误类型
#[derive(Debug,Clone)]
pub enum AppErrorType {
    Db,
    Notfound,
    Duplicate,
    Crypt,
    Forbidden,
    ParseError,
    Serialize,
    ThreadError,
    Chrono,
    StdIo,
    Net,
}

// 应用程序错误
#[derive(Debug)]
pub struct AppError {
    pub message: Option<String>,
    pub cause: Option<Box<dyn std::error::Error>>,
    pub types: AppErrorType,
}

impl AppError {
    fn new(
        message: Option<String>,
        cause: Option<Box<dyn std::error::Error>>,
        types: AppErrorType,
    ) -> Self {
        Self {
            message,
            cause,
            types,
        }
    }

    fn from_err(cause: Box<dyn std::error::Error>, types: AppErrorType) -> Self {
        Self::new(None, Some(cause), types)
    }

    fn from_str(msg: &str, types: AppErrorType) -> Self {
        Self::new(Some(msg.to_string()), None, types)
    }
    pub fn notfound_opt(message: Option<String>) -> Self {
        Self::new(message, None, AppErrorType::Notfound)
    }
    pub fn notfound_msg(msg: &str) -> Self {
        Self::notfound_opt(Some(msg.to_string()))
    }
    pub fn notfound() -> Self {
        Self::notfound_msg("not found error")
    }
    pub fn duplicate(msg: &str) -> Self {
        Self::from_str(msg, AppErrorType::Duplicate)
    }

    pub fn forbidden() -> Self {
        Self::from_str("unauthorized", AppErrorType::Forbidden)
    }

    pub fn thread_err_msg() -> Self {
        Self::from_str("thread error", AppErrorType::ThreadError)
    }

    pub fn parse_error() -> Self {
        Self::from_str("parse error", AppErrorType::ParseError)
    }

    pub fn io_error() -> Self {
        Self::from_str("read file error", AppErrorType::StdIo)
    }

    pub fn response(self) -> axum::response::Response {
        match self.types {
            AppErrorType::Forbidden  => {
                let mut hm = HeaderMap::new();
                hm.insert(header::LOCATION, "Unauthorized".parse().unwrap());
                (StatusCode::UNAUTHORIZED, hm, ()).into_response()
            }
            _ => self
                .message
                .unwrap_or("error occured".to_string())
                .into_response(),
        }
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}



impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        Self::from_err(Box::new(err), AppErrorType::Net)
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        Self::from_err(Box::new(err), AppErrorType::StdIo)
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(err: std::num::ParseIntError) -> Self {
        Self::from_err(Box::new(err), AppErrorType::ParseError)
    }
}

impl From<tokio::task::JoinError> for AppError {
    fn from(err: tokio::task::JoinError) -> Self {
        Self::from_err(Box::new(err), AppErrorType::ThreadError)
    }
}

impl FromStr for AppError {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 在这里进行字符串到AppError的转换
        // 如果转换成功，返回Ok(AppError::...)，否则返回Err
        // 例如，假设你的AppError是一个表示整数解析错误的枚举类型，可以这样写：
        match s.parse::<i32>() {
            Ok(_) => Ok(AppError::parse_error()),
            Err(e) => Err(e),
        }
    }
}

impl std::error::Error for AppError {}


impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        self.response()
    }
}