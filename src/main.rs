use clap::Parser;
use error_handling::{Cli, Outcome};

#[tokio::main]
async fn main() -> Outcome {
    let args = Cli::parse();
    args.run().await
}
