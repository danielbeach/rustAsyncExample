use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use reqwest::Error;
use futures::future::join_all;


#[tokio::main]
async fn main() -> Result<(), Error> {
    let urls = [
        "https://divvy-tripdata.s3.amazonaws.com/202305-divvy-tripdata.zip",
        "https://divvy-tripdata.s3.amazonaws.com/202304-divvy-tripdata.zip",
        "https://divvy-tripdata.s3.amazonaws.com/202303-divvy-tripdata.zip",
        "https://divvy-tripdata.s3.amazonaws.com/202302-divvy-tripdata.zip",
    ];

    // Create a future for each download and run them all in parallel
    let downloads: Vec<_> = urls.iter().map(|url| download_file(url)).collect();
    let results = join_all(downloads).await;

    // Check for errors
    for result in results {
        match result {
            Ok(filename) => println!("Downloaded to {}", filename),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    Ok(())
}

async fn download_file(url: &str) -> Result<String, Error> {
    // Create a path to save the file
    let filename = url.split('/').last().unwrap_or("file");

    // Make the HTTP get request
    let response = reqwest::get(url).await?;

    // Make sure the request was successful
    assert!(response.status().is_success());

    // Stream the body, write to file
    let mut file = File::create(filename).await.unwrap();
    let bytes = response.bytes().await.unwrap();
    file.write_all(&bytes).await.unwrap();

    Ok(filename.to_string())
}
