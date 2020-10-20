use mini_redis::{client, Result};
use tokio_compat_02::FutureExt; // To try with tokio 0.3, needs the compat() method from this trait to let tokio 0.2 code working. https://tokio.rs/blog/2020-10-tokio-0-3

#[tokio::main]
pub async fn main() -> Result<()> {
    // Open a connection to the mini-redis address.
    let mut client = client::connect("127.0.0.1:6379").compat().await?;

    // Set the key "hello" with value "world"
    client.set("hello", "world".into()).compat().await?;

    // Get key "hello"
    let result = client.get("hello").compat().await?;

    println!("got value from the server; result={:?}", result);

    Ok(())
}