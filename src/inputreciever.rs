use sfml::system::Vector2u;

pub trait InputReciever {
    fn on_up(&mut self, win_size: &Vector2u, delta_seconds: f32);
    fn on_down(&mut self, win_size: &Vector2u, delta_seconds: f32);
}
