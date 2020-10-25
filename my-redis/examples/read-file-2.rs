use tokio::io::{self, AsyncReadExt};
use tokio::fs::File;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut f = File::open("my-redis/examples/foo.txt").await?;
    let mut buffer = Vec::new();

    // read the whole file
    f.read_to_end(&mut buffer).await?;

    println!("The content: {}", std::str::from_utf8(buffer.as_ref()).unwrap_or(""));
    Ok(())
}

// https://tokio.rs/tokio/tutorial/io
