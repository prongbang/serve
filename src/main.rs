mod logger;

use clap::Parser;
use warp::Filter;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 8000)]
    port: u16,
    #[arg(short, long, default_value_t = false)]
    silent: bool,
}

#[tokio::main]
async fn main() {
    let version = "0.2.3";

    // Parse command line arguments
    let args = Args::parse();

    // Get the current directory
    let current_dir = std::env::current_dir().expect("Failed to get current directory");

    // Convert the current directory to a string
    let current_path = String::from(current_dir.to_string_lossy().as_ref());

    // Create a Warp filter to handle requests for static assets
    let static_dir = warp::fs::dir(current_path);

    // CORS
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["HEAD", "CONNECT", "TRACE", "GET", "POST", "PUT", "PATCH", "OPTIONS", "DELETE"]);

    // Create a Warp filter for serving the main HTML file
    let index_html = warp::any().and(warp::fs::file("index.html"));

    // Create the address tuple
    let ip_address = [0, 0, 0, 0];
    let addr = (ip_address, args.port);
    let ip = ip_address.iter().map(|&octet| octet.to_string()).collect::<Vec<_>>().join(".");

    // Print the listening address
    println!("{}", format!(r#"
  ___ ___ _____  _____ ____
 (_-</ -_) __/ |/ / -_) __/
/___/\__/_/  |___/\__/_/ (v{})

Listen on http://{}:{}
"#, version, ip, addr.1));

    // Start the Warp server with only the static assets filter
    if args.silent {
        let routes = static_dir.or(index_html).with(cors);
        warp::serve(routes).run(addr).await;
    } else {
        logger::setup_logging();
        let routes = static_dir.or(index_html).with(cors).with(logger::log());
        warp::serve(routes).run(addr).await;
    };
}