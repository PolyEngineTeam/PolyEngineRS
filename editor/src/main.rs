// #[macro_use]
// extern crate static_assertions;
// #[macro_use]
// extern crate approx; // For the macro relative_eq!
// extern crate nalgebra as na;

extern crate pretty_env_logger;

mod client;
mod primitives;

use client::ClientApp;
use winit::event_loop::EventLoop;

use polyengine_core::log;

fn main() {
    pretty_env_logger::init();
    log::info!("Starting PolyEngine editor...");

    let event_loop = EventLoop::new();
    let mut app = ClientApp::new(&event_loop);
    event_loop.run(move |event, elwt, control_flow| app.on_event(event, elwt, control_flow));
}
