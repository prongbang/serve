use warp::log::{Info, Log};

pub fn setup_logging() {
    std::env::set_var("RUST_LOG", "server");
    pretty_env_logger::init();
}

pub fn log() -> Log<impl Fn(Info<'_>) + Copy + Sized> {
    return warp::log("server");
}