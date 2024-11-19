use std::convert::Infallible;
use warp::{Rejection, Reply};
use crate::vo::ResultCode;

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let json;
    let status;
    if err.is_not_found() {
        json = warp::reply::json(&ResultCode::NOT_FOUND);
        status = ResultCode::NOT_FOUND.status;
    } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {

        json = warp::reply::json(&ResultCode::BAD_REQUEST);
        status = ResultCode::BAD_REQUEST.status;
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        json = warp::reply::json(&ResultCode::METHOD_NOT_ALLOWED);
        status = ResultCode::METHOD_NOT_ALLOWED.status;
    } else if let Some(result_code) = err.find::<ResultCode>() {
        json = warp::reply::json(result_code);
        status = result_code.status;
    } else {
        json = warp::reply::json(&ResultCode::INTERNAL_ERROR);
        status = ResultCode::INTERNAL_ERROR.status;
    }

    Ok(warp::reply::with_status(json, status))
}
