use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

pub struct WindowHandler {
    pub event_loop: EventLoop<()>,
    pub window: Window,
}

impl WindowHandler {
    pub fn new() -> Self {
        // Création de la boucle d'événements
        let event_loop = EventLoop::new();

        // Création de la fenêtre
        let window = WindowBuilder::new()
            .with_title("Vulkan Application")
            .with_inner_size(winit::dpi::LogicalSize::new(1024, 768))
            .build(&event_loop)
            .unwrap();

        Self { event_loop, window }
    }
}
