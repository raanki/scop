use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};
use super::window::WindowHandler;

pub fn run(window_handler: WindowHandler) {
    let WindowHandler { event_loop, window: _ } = window_handler;

    // Boucle principale
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit, // Quitter l'application
            _ => (),
        }
    });
}
