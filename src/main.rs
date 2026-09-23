use anyhow::Result;
use clap::Parser;
use colored::*;
use futures::stream::{self, StreamExt};
use reqwest::Client;
use std::time::{Duration, Instant};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// URL to test
    url: String,

    /// Total number of requests
    #[arg(short = 'n', long, default_value_t = 100)]
    requests: usize,

    /// Number of concurrent requests
    #[arg(short = 'c', long, default_value_t = 10)]
    concurrency: usize,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let client = Client::builder().timeout(Duration::from_secs(10)).build()?;

    println!(
        "{} {} with {} requests (concurrency: {})",
        "Benchmarking".cyan(),
        args.url.bold(),
        args.requests,
        args.concurrency
    );

    let start = Instant::now();

    let requests_stream = stream::iter(0..args.requests).map(|_| {
        let client = client.clone();
        let url = args.url.clone();
        tokio::spawn(async move {
            let req_start = Instant::now();
            let res = client.get(&url).send().await;
            let elapsed = req_start.elapsed();

            match res {
                Ok(response) => Ok((response.status().as_u16(), elapsed)),
                Err(e) => Err(e),
            }
        })
    });

    let mut results = requests_stream.buffer_unordered(args.concurrency);

    let mut successes = 0;
    let mut failures = 0;
    let mut total_time = Duration::new(0, 0);

    while let Some(res) = results.next().await {
        match res {
            Ok(Ok((_status, elapsed))) => {
                successes += 1;
                total_time += elapsed;
            }
            _ => {
                failures += 1;
            }
        }
    }

    let elapsed = start.elapsed();

    println!("\n{}", "Results:".bold());
    println!("  Total time:     {:.2?}", elapsed);
    println!("  Successes:      {}", successes.to_string().green());
    println!("  Failures:       {}", failures.to_string().red());

    if successes > 0 {
        let avg_req_time = total_time / successes as u32;
        println!("  Avg req time:   {:.2?}", avg_req_time);
        println!(
            "  Reqs/sec:       {:.2}",
            (successes as f64) / elapsed.as_secs_f64()
        );
    }

    Ok(())
}
