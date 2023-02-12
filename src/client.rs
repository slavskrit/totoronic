use futures::StreamExt;
use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    // for x in 0..10 {
    let request = tonic::Request::new(HelloRequest {
        name: format!("Tonic").into(),
    });
    let mut response = client.say_hello(request).await?.into_inner();
    while let Some(res) = response.message().await? {
        println!("NOTE = {:?}", res);
    }
    // println!("RESPONSE={:?}", response);
    // }

    // let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
    //     .connect()
    //     .await?;
    //     let mut client = SayClient::new(channel);
    //     let request = tonic::Request::new(
    //         SayRequest {
    //            name:String::from("anshul")
    //         },
    //     );
    // // now the response is stream
    //     let mut response = client.send_stream(request).await?.into_inner();
    // // listening to stream
    //     while let Some(res) = response.message().await? {
    //         println!("NOTE = {:?}", res);
    //     }
    //     Ok(())

    Ok(())
}
