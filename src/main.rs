use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 8000)]
    port: u16,
}

#[tokio::main]
async fn main() {
    // Parse command line arguments
    let args = Args::parse();

    // Get the current directory
    let current_dir = std::env::current_dir().expect("Failed to get current directory");

    // Convert the current directory to a string
    let current_path = String::from(current_dir.to_string_lossy().as_ref());

    // Create a Warp filter to handle requests for static assets
    let static_dir = warp::fs::dir(current_path);

    // Combine the log filter with the filter for static assets
    let routes = static_dir;

    // Create the address tuple
    let ip_address = [0, 0, 0, 0];
    let addr = (ip_address, args.port);
    let ip = ip_address.iter().map(|&octet| octet.to_string()).collect::<Vec<_>>().join(".");

    // Print the listening address
    println!("Listening on http://{}:{}", ip, addr.1);

    // Start the Warp server with only the static assets filter
    warp::serve(routes).run(addr).await;
}