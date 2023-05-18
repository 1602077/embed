use embed::embedder_client::EmbedderClient;
use embed::EmbedRequest;

pub mod embed {
    tonic::include_proto!("embed");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = EmbedderClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(EmbedRequest {
        body: "this is a query to emebd".into(),
    });

    let response = client.embed(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
