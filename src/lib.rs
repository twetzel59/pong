extern crate sfml;

use sfml::graphics::RenderWindow;
use sfml::window::{ContextSettings, style, VideoMode};

pub use inputreciever::InputReciever;
pub use ball::Ball;

pub mod paddle;
pub mod inputreciever;
pub mod playerinput;
pub mod ball;

pub fn create_window() -> RenderWindow {
    RenderWindow::new(VideoMode::new(800, 600, 32), "rsPong",
                      style::TITLEBAR | style::CLOSE, &ContextSettings::default())
                  .unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
