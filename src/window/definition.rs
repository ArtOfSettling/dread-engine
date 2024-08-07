use std::string::ToString;
use crate::constants;

pub struct Definition {
    pub title: String,
    pub width: u32,
    pub height: u32
}

impl Default for Definition {
    fn default() -> Self {
        Definition {
            title: constants::window::WINDOW_TITLE.to_string(),
            width: constants::window::WINDOW_WIDTH,
            height: constants::window::WINDOW_HEIGHT
        }
    }
}

impl Definition {
    pub(crate) fn build(title: String) -> Self {
        Definition {
            title,
            ..Definition::default()
        }
    }
}
