use crate::{
    methods::queries::Queries,
    models::{
        errors::APIError, requests::EchoRequest, responses::EchoResponse,
    },
};

pub async fn get_root(
    _: &mut impl Queries,
    request: EchoRequest,
) -> Result<EchoResponse, APIError> {
    if request.message == "error" {
        Err(APIError::InternalError)
    } else {
        Ok(EchoResponse {
            message: request.message,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        methods::{echo::get_root, memory_queries::MemoryQueries},
        models::{
            errors::APIError, requests::EchoRequest, responses::EchoResponse,
        },
    };

    #[tokio::test]
    async fn returns_foo_when_foo() {
        let mut queries = MemoryQueries::default();
        let result = get_root(
            &mut queries,
            EchoRequest {
                message: "foo".into(),
            },
        )
        .await;
        assert_eq!(
            result,
            Ok(EchoResponse {
                message: "foo".into()
            })
        );
    }

    #[tokio::test]
    async fn returns_error_when_error() {
        let mut queries = MemoryQueries::default();
        let result = get_root(
            &mut queries,
            EchoRequest {
                message: "error".into(),
            },
        )
        .await;
        assert_eq!(result, Err(APIError::InternalError));
    }
}
