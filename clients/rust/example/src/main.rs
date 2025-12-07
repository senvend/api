use senvend_api::protos::api::v1::Uuid4;
use senvend_api::protos::api::v1::{
    AgeApiSuccess, AgeApiSuccessReason, AgeStartRequest, AgeSuccess, PayApiSuccess,
    PayApiSuccessReason, PayApproved, PayGoodsIssued, PayRequest, PayStart, PaySuccess,
    pay_request::Request, pay_response::Result as PayResult,
};
use senvend_api::protos::local::v1::pay_service_client::PayServiceClient;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::Endpoint;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let ip = std::env::var("TERMINAL_IP").expect("TERMINAL_IP must be set");
    let port = std::env::var("TERMINAL_PORT").unwrap_or("11111".to_string());
    let url = format!("http://{}:{}", ip, port);
    println!("Connecting to terminal at {}", url);
    let endpoint = Endpoint::from_shared(url)
        .expect("Invalid endpoint URL")
        .connect_timeout(std::time::Duration::from_secs(5));

    example_pay(endpoint).await.expect("example_pay failed");
}

async fn example_pay(endpoint: Endpoint) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = PayServiceClient::connect(endpoint).await?;
    println!("PayService connected!");

    let pay_uuid = Uuid::new_v4();
    let pay_uuid_proto = Uuid4::try_from(pay_uuid).expect("Failed to convert UUID");

    let (tx, rx) = mpsc::channel::<PayRequest>(8);
    let outbound = ReceiverStream::new(rx);

    let pay_start = PayRequest {
        id: Some(pay_uuid_proto),
        request: Some(Request::Start(PayStart {
            amount: 100,
            age_verification: Some(AgeStartRequest { min_age: 18 }),
        })),
    };
    println!("Sending PayStart request: {:?}", pay_start);
    tx.send(pay_start).await?;

    let mut response_stream = client.pay(outbound).await?.into_inner();

    let age_started = response_stream
        .message()
        .await?
        .expect("Expected a response to PayStart");
    println!("Received start response: {:?}", age_started);
    match age_started.result {
        Some(PayResult::AgeApiSuccess(AgeApiSuccess { reason }))
            if reason == AgeApiSuccessReason::VerificationStarted as i32 =>
        {
            println!("Age verification started");
        }
        other => {
            println!("Unexpected response to PayStart: {:?}", other);
            return Err("Failed to start age verification".into());
        }
    }

    let age_response = response_stream
        .message()
        .await?
        .expect("Expected an age verification response");
    println!("Received age verification response: {:?}", age_response);
    match age_response.result {
        Some(PayResult::AgeSuccess(AgeSuccess {})) => {
            println!("Age verification successful");
        }
        other => {
            println!("Unexpected age verification response: {:?}", other);
            return Err("Age verification failed".into());
        }
    }

    let start_resp = response_stream
        .message()
        .await?
        .expect("Expected a response to PayStart");
    println!("Received start response: {:?}", start_resp);
    match start_resp.result {
        Some(PayResult::ApiSuccess(PayApiSuccess { reason }))
            if reason == PayApiSuccessReason::PaymentStarted as i32 =>
        {
            println!("Payment started successfully");
        }
        other => {
            println!("Unexpected response to PayStart: {:?}", other);
            return Err("Failed to start payment".into());
        }
    }

    let pay_response = response_stream
        .message()
        .await?
        .expect("Expected a response after PayStart");
    println!("Received payment response: {:?}", pay_response);
    match pay_response.result {
        Some(PayResult::Approved(PayApproved {})) => {
            println!("Payment approved!");
        }
        other => {
            println!("Unexpected payment response: {:?}", other);
            return Err("Payment not approved".into());
        }
    }

    let pay_goods_issued = PayRequest {
        id: None,
        request: Some(Request::GoodsIssued(PayGoodsIssued { partial_amount: 0 })),
    };
    tx.send(pay_goods_issued).await?;
    println!("Sent GoodsIssued notification");

    let pay_goods_issued_response = response_stream
        .message()
        .await?
        .expect("Expected a response to GoodsIssued");
    println!("Received final response: {:?}", pay_goods_issued_response);
    match pay_goods_issued_response.result {
        Some(PayResult::ApiSuccess(PayApiSuccess { reason }))
            if reason == PayApiSuccessReason::GoodsIssuedAccepted as i32 =>
        {
            println!("Issue of goods acknowledged by terminal");
        }
        other => {
            println!("Unexpected response to GoodsIssued: {:?}", other);
            return Err("Failed to acknowledge goods issued".into());
        }
    }

    let pay_response = response_stream
        .message()
        .await?
        .expect("Expected a final response for payment completion");
    println!(
        "Received final payment completion response: {:?}",
        pay_response
    );
    match pay_response.result {
        Some(PayResult::Success(PaySuccess {})) => {
            println!("Payment completed successfully!");
        }
        other => {
            println!("Unexpected final payment response: {:?}", other);
            return Err("Payment completion failed".into());
        }
    }

    Ok(())
}
