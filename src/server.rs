use std::time::Duration;

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};
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

        let name = request.into_inner().name;
        tokio::spawn(async move {
            loop {
                let reply = HelloReply {
                    message: format!("Hello {}", name.clone()),
                };
                tx.send(Ok(reply)).await.unwrap_or_default();
                // .await
                // .unwrap();
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
    // async fn say_hello(
    //     &self,
    //     request: Request<HelloRequest>,
    // ) -> Result<Response<HelloReply>, Status> {
    //     println!("Got a request from {:?}", request.remote_addr());

    //     let reply = hello_world::HelloReply {
    //         message: format!("Hello {}!", request.into_inner().name),
    //     };
    //     Ok(Response::new(reply))
    // }
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
