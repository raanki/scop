use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    // Création de la boucle d'événements
    let event_loop = EventLoop::new();

    // Création de la fenêtre
    let window = WindowBuilder::new()
        .with_title("Vulkan Application")
        .with_inner_size(winit::dpi::LogicalSize::new(1024, 768)) // Taille de la fenêtre
        .build(&event_loop)
        .unwrap();

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
