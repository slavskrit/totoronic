use std::time::Duration;

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};
use http::Method;
use rand::{distributions::Alphanumeric, Rng};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};
use tonic_web::GrpcWebLayer;
use tower_http::cors::{Any, CorsLayer};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn test(&self, request: Request<HelloRequest>) -> Result<Response<HelloReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }

    type SayHelloStream = ReceiverStream<Result<HelloReply, Status>>;
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<Self::SayHelloStream>, Status> {
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
                let reply = HelloReply {
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
    let greeter = GreeterServer::new(MyGreeter::default());

    println!("Starting server at {port}");

    let cors = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods([Method::POST])
        .allow_origin(Any); // gives permision to any IP to call this service

    Server::builder()
        .accept_http1(true)
        .layer(cors)
        .layer(GrpcWebLayer::new())
        .add_service(greeter)
        // .add_service(tonic_web::enable(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
