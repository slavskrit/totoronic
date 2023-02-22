use std::time::Duration;

use http::Method;
use rand::{distributions::Alphanumeric, Rng};
use telemetry::telemetry_service_server::{TelemetryService, TelemetryServiceServer};
use telemetry::{HeatMapRequest, HeatMapResponse};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};
use tonic_web::GrpcWebLayer;
use tower_http::cors::{Any, CorsLayer};

pub mod telemetry {
    tonic::include_proto!("telemetry");
}

#[derive(Default)]
pub struct TelemetryImpl {}

#[tonic::async_trait]
impl TelemetryService for TelemetryImpl {
    type GetHeatMapStream = ReceiverStream<Result<HeatMapResponse, Status>>;

    async fn get_heat_map(
        &self,
        request: Request<HeatMapRequest>,
    ) -> Result<Response<Self::GetHeatMapStream>, Status> {
        let (tx, rx) = mpsc::channel(4);
        println!("Incoming request: {}", request.into_inner().name);
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_millis(1000)).await;
                // let rng = rand::thread_rng();
                let s: String = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(100)
                    .map(char::from)
                    .collect();
                // println!("{s}");
                let reply = HeatMapResponse {
                    message: format!("{}", s),
                };
                tx.send(Ok(reply)).await.unwrap_or_default();
            }
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = "8000";
    let addr = format!("[::1]:{port}").parse().unwrap();
    let greeter = TelemetryServiceServer::new(TelemetryImpl::default());

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
