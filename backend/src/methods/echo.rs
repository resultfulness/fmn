use crate::models::{
    errors::APIError, requests::EchoRequest, responses::EchoResponse,
};

pub async fn get_root(request: EchoRequest) -> Result<EchoResponse, APIError> {
    if request.message == "error" {
        Err(APIError::IDontLikeErrorWord)
    } else {
        Ok(EchoResponse {
            message: request.message,
        })
    }
}
