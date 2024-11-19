pub mod request;
pub mod response;

use serde::Deserialize;
use serde::Serialize;
use warp::hyper::StatusCode;
use warp::reject::Reject;

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, Eq, PartialOrd, Ord, Hash)]
pub struct ResultCode {
    pub code: &'static str,
    pub msg: &'static str,

    #[serde(skip)]
    pub status: StatusCode,
}

impl Reject for ResultCode {}

macro_rules! result_code {
    (
        $(
            $(#[$docs:meta])*
            ($konst:ident, $code:expr, $msg:expr,$status:expr);
        )+
    ) => {
        impl ResultCode {
            $(
                $(#[$docs])*
                pub const $konst: ResultCode = ResultCode{code:$code,msg:$msg,status:$status};
            )+
        }
    }
}

result_code! {
    (SUCCESS,"00000","SUCCESS",StatusCode::OK);
    (INTERNAL_ERROR,"B0001","Internal error",StatusCode::INTERNAL_SERVER_ERROR);
    (PARAM_ERROR,"B0002","param error",StatusCode::BAD_REQUEST);
    (PAPER_NOT_FOUND,"B0003","paper not found",StatusCode::BAD_REQUEST);
    (METHOD_NOT_ALLOWED,"B0004","Method Not Allowed",StatusCode::METHOD_NOT_ALLOWED);
    (BAD_REQUEST,"B0005","Bad Request",StatusCode::BAD_REQUEST);
    (NOT_FOUND,"B0006","Not Found",StatusCode::NOT_FOUND);
    (NOT_SUPPORT,"B0007","Not Support",StatusCode::FORBIDDEN);
}




