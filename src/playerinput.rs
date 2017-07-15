use sfml::window::Key;

#[derive(Copy, Clone)]
pub struct PlayerInput {
    pub l_up: bool,
    pub l_down: bool,
    pub r_up: bool,
    pub r_down: bool,
}

pub fn handle_input() -> PlayerInput {
    PlayerInput {
        l_up: Key::W.is_pressed(),
        l_down: Key::S.is_pressed(),
        r_up: Key::Up.is_pressed(),
        r_down: Key::Down.is_pressed(),
    }
}
