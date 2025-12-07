mod simulation;
mod renderer;
mod job_system;

fn main() {
    tracing_subscriber::fmt::init();

    if let Err(e) = renderer::run() {
        eprintln!("Application error: {}", e);
    }
}
