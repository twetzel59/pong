extern crate sfml;

use sfml::graphics::RenderWindow;
use sfml::window::{ContextSettings, style, VideoMode};

pub use inputreciever::InputReciever;
pub use ball::Ball;
pub use ball::WhoScored;
pub use scoreboard::Scoreboard;

pub mod paddle;
pub mod inputreciever;
pub mod playerinput;
pub mod ball;
pub mod scoreboard;

pub fn create_window() -> RenderWindow {
    RenderWindow::new(VideoMode::new(900, 700, 32), "rsPong",
                      style::TITLEBAR | style::CLOSE, &ContextSettings::default())
                  .unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
