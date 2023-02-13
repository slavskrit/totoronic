use http::Method;
use telemetry::telemetry_server::{Telemetry, TelemetryServer};
use telemetry::{HeatMapRequest, HeatMapResponse};
use tonic::{transport::Server, Request, Response, Status};
use tonic_web::GrpcWebLayer;
use tower_http::cors::{Any, CorsLayer};

pub mod telemetry {
    tonic::include_proto!("telemetry");
}

#[derive(Default)]
pub struct TelemetryImpl {}

#[tonic::async_trait]
impl Telemetry for TelemetryImpl {
    async fn get_heat_map(
        &self,
        request: Request<HeatMapRequest>,
    ) -> Result<Response<HeatMapResponse>, Status> {
        let reply = telemetry::HeatMapResponse {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = "8000";
    let addr = format!("[::1]:{port}").parse().unwrap();
    let greeter = TelemetryServer::new(TelemetryImpl::default());

    println!("Starting server at {port}");

    let cors = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods([Method::POST])
        .allow_origin(Any);

    Server::builder()
        .accept_http1(true)
        .layer(cors)
        .layer(GrpcWebLayer::new())
        .add_service(greeter)
        .serve(addr)
        .await?;

    Ok(())
}
