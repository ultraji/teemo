use clap::Parser;
use clap::{crate_authors, crate_version, crate_description};

use teemo::Teemo;

#[derive(Parser)]
#[command(author = crate_authors!())]
#[command(version = crate_version!())]
#[command(about = crate_description!())]
#[command(help_template = " {about-section}\nAuthor: {author} \nVersion: {version} \n\n {usage-heading} {usage} \n {all-args} {tab}")]
struct Args {
    /// Set config file for teemo
    #[arg(short, long, value_name = "FILE", default_value_t = String::from("./teemo.yaml"))]
    pub config_file: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error>{
    env_logger::init();

    let args = Args::parse();

    Teemo::new(&args.config_file)?.run().await
}