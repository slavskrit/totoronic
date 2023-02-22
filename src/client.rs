use telemetry::telemetry_service_client::TelemetryServiceClient;
use telemetry::HeatMapRequest;

pub mod telemetry {
    tonic::include_proto!("telemetry");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TelemetryServiceClient::connect("http://[::1]:8000").await?;

    let request = tonic::Request::new(HeatMapRequest {
        name: format!("yo").into(),
    });
    let mut response = client.get_heat_map(request).await?.into_inner();
    while let Some(res) = response.message().await? {
        println!("{}", res.message);
    }
    Ok(())
}
