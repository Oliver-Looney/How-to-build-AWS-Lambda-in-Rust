use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use aws_lambda_events::encodings::Body;
use lambda_runtime::{Error, LambdaEvent};
use serde_json::json;
use serde::Deserialize;

#[tokio::main]
async fn main() -> Result<(), Error>{
    let lambda_handler = lambda_runtime::service_fn(handler);
    lambda_runtime::run(lambda_handler).await?;
    Ok(())
}

#[derive(Deserialize)]
struct Event {
    name: String
}

async fn handler(lambda_event: LambdaEvent<ApiGatewayProxyRequest>) -> Result<ApiGatewayProxyResponse, Error> {

    let event: Event = serde_json::from_str(&lambda_event.payload.body.unwrap())?;

    return Ok(ApiGatewayProxyResponse{
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(
            Body::Text(
                json!({
                    "message": format!("Hello {}", event.name)
                }).to_string())),
        is_base64_encoded: Some(false),
    })
}
