mod hello;
pub use hello::*;

use crate::vo::ResultCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse<T> {
    code: String,
    msg: String,
    data: Option<T>,
}

///rust想要使用零开销的默认Generic，是不能使用（）或者 _ 的，必须自己定义一个不包含任何数据的struct。
/// 如EmptyData。
///
/// # Examples:
///
///```
/// use nullht_script_common::response::{ApiResponse, EmptyData};
///
/// pub trait Success{
///     fn success()->ApiResponse<EmptyData>;
/// }
///
/// impl Success for ApiResponse<EmptyData> {
/// fn success() -> ApiResponse<EmptyData> {
///         ApiResponse::builder()
///             .with_code(String::from(ResultCode::SUCCESS.code))
///             .with_msg(String::from(ResultCode::SUCCESS.msg))
///             .build()
/// }
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct EmptyData;


impl<T> ApiResponse<T> {
    pub fn builder() -> ApiResponseBuilder<T> {
        ApiResponseBuilder::default()
    }
}

pub struct ApiResponseBuilder<T> {
    code: String,
    msg: String,
    data: Option<T>,
}

impl<T> Default for ApiResponseBuilder<T> {
    fn default() -> Self {
        ApiResponseBuilder {
            code: String::default(),
            msg: String::default(),
            data: None,
        }
    }
}

impl<T> ApiResponseBuilder<T> {
    pub fn with_code(mut self, code: String) -> ApiResponseBuilder<T> {
        self.code = code;
        self
    }

    pub fn with_msg(mut self, msg: String) -> ApiResponseBuilder<T> {
        self.msg = msg;
        self
    }

    pub fn with_data(mut self, data: T) -> ApiResponseBuilder<T> {
        self.data = Some(data);
        self
    }

    pub fn build(self) -> ApiResponse<T> {
        ApiResponse {
            code: self.code,
            msg: self.msg,
            data: self.data,
        }
    }
}


impl ApiResponse<EmptyData> {
    pub fn success() -> ApiResponse<EmptyData> {
        ApiResponse::builder()
            .with_code(String::from(ResultCode::SUCCESS.code))
            .with_msg(String::from(ResultCode::SUCCESS.msg))
            .build()
    }
}


impl<T> ApiResponse<T> {
    pub fn success_with_data(data: T) -> ApiResponse<T>
    {
        ApiResponse::builder()
            .with_code(String::from(ResultCode::SUCCESS.code))
            .with_msg(String::from(ResultCode::SUCCESS.msg))
            .with_data(data)
            .build()
    }
}





