pub mod window;
pub mod event;

pub struct App {
    window: window::WindowHandler,
}

impl App {
    pub fn new() -> Self {
        // Initialisation de la fenêtre
        let window = window::WindowHandler::new();
        Self { window }
    }

    pub fn run(self) {
        // Exécuter la boucle d'événements
        event::run(self.window);
    }
}
