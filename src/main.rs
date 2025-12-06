mod simulation;
mod renderer;

fn main() {
    tracing_subscriber::fmt::init();

    if let Err(e) = renderer::run() {
        eprintln!("Application error: {}", e);
    }
}
