mod definition;

pub use crate::application::definition::Definition;
use crate::window;
pub use crate::window::Window;

pub struct Application {
    pub application_definition: Definition
}

impl Application {
    pub fn run(&self) {
        Window {
            definition: window::Definition::build(self.application_definition.title.clone())
        }.run();
    }
}
