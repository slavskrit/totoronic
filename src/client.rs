use telemetry::telemetry_client::TelemetryClient;
use telemetry::HeatMapRequest;

pub mod telemetry {
    tonic::include_proto!("telemetry");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TelemetryClient::connect("http://[::1]:8000").await?;

    let request = tonic::Request::new(HeatMapRequest {
        name: format!("yo").into(),
    });
    let response = client.get_heat_map(request).await?.into_inner();
    println!("{}", response.message);
    Ok(())
}
