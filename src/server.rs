use std::time::Duration;

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};
use rand::{distributions::Alphanumeric, Rng}; // 0.8
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    type SayHelloStream = ReceiverStream<Result<HelloReply, Status>>;
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<Self::SayHelloStream>, Status> {
        let (tx, rx) = mpsc::channel(4);

        // let m = generate_matrix(10, 10, rng.gen_range(0..10) as f64);
        // let t = m[0][0];
        println!("{}", request.into_inner().name);
        tokio::spawn(async move {
            loop {
                // let rng = rand::thread_rng();
                let s: String = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(100)
                    .map(char::from)
                    .collect();
                let reply = HelloReply {
                    message: format!("{}", s),
                };
                tx.send(Ok(reply)).await.unwrap_or_default();
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyGreeter::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
