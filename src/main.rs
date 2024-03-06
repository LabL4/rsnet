mod app;
mod gui;
mod utils;
mod renderer;
mod scene;

fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { app::event_loop::run().await })
}
