use crate::vo::request::HelloWorldReq;
use crate::vo::response::HelloResult;
use crate::vo::ResultCode;

pub async fn analyze(request: HelloWorldReq) -> Result<HelloResult, ResultCode> {



    Ok(HelloResult{
        id: "".to_string(),
        tag_define_id: "".to_string(),
        code: "".to_string(),
        name: "".to_string(),
    })
}



