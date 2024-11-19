use serde::Serialize;
use warp::{Filter};
use crate::exception_handler::handle_rejection;
use crate::service;
use crate::vo::request::{HelloWorldReq};
use crate::vo::response::{ApiResponse};
use crate::vo::ResultCode;

fn wrapper_result(result: Result<(), ResultCode>) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(code) = result {
        Err(warp::reject::custom(code))
    } else {
        Ok(warp::reply::json(&ApiResponse::success()))
    }
}

fn wrapper_result_with_data<T>(result: Result<T, ResultCode>) -> Result<impl warp::Reply, warp::Rejection>
where
    T: Serialize,
{
    match result {
        Ok(r) => {
            Ok(warp::reply::json(&ApiResponse::success_with_data(r)))
        }
        Err(code) => {
            Err(warp::reject::custom(code))
        }
    }
}


pub fn route() -> impl Filter<Extract=(impl warp::Reply,)> + Clone + Send + Sync + 'static {
    let prefix = warp::path!("api"/..);


    let paper_reset_route = warp::path!("tag"/"analyze")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|request: HelloWorldReq| async {
            wrapper_result_with_data(service::analyze(request).await)
        });


    prefix
        .and(paper_reset_route)
        .with(warp::log("service-auto-tag-csl"))
        .recover(handle_rejection)
}

