use sfml::graphics::*;
use sfml::system::{Vector2f, Vector2u};

use inputreciever::InputReciever;

const SPEED: f32 = 400.;

pub struct Paddle<'s> {
    rect: RectangleShape<'s>,
}

#[derive(Clone, Copy, Debug)]
pub enum Side {
    Left,
    Right,
}

impl<'s> Paddle<'s> {
    pub fn new(win_size: &Vector2u, side: Side) -> Paddle<'s> {
        let mut rect = RectangleShape::with_size(&Vector2f::new(32., 128.));
        
        let size = rect.size();
        match side {
            Side::Left => rect.set_position2f(0., win_size.y as f32 / 2. - size.y / 2.),
            Side::Right => rect.set_position2f(win_size.x as f32 - size.x, win_size.y as f32 / 2. - size.y / 2.),
        }
        
        rect.set_fill_color(&Color::rgb(30, 175, 25));
        
        Paddle {
            rect,
        }
    }
    
    pub fn rect(&self) -> FloatRect {
        self.rect.global_bounds()
    }
    
    fn ensure_onscreen(&mut self, win_size: &Vector2u) {
        let current_pos = self.rect.position();
        let lower_border = win_size.y as f32 - self.rect.size().y;
        let upper_border = 0.;
        
        if self.rect.position().y > lower_border {
            //println!("offscreen");
            self.rect.set_position2f(current_pos.x, lower_border);
        } else if self.rect.position().y < upper_border {
            self.rect.set_position2f(current_pos.x, upper_border);
        }
    }
}

impl<'s> Drawable for Paddle<'s> {
    fn draw<'se, 'tex, 'sh, 'shte>(
        &'se self, 
        target: &mut RenderTarget, 
        states: RenderStates<'tex, 'sh, 'shte>
    )
    where 'se: 'sh {
        target.draw_rectangle_shape(&self.rect, states)
    }
}

impl<'s> InputReciever for Paddle<'s> {
    fn on_up(&mut self, win_size: &Vector2u, delta_seconds: f32) {
        //println!("up: {:?}", self.side);
        self.rect.move2f(0., -SPEED * delta_seconds);
        self.ensure_onscreen(win_size);
    }
    
    fn on_down(&mut self, win_size: &Vector2u, delta_seconds: f32) {
        //println!("down: {:?}", self.side);
        self.rect.move2f(0., SPEED * delta_seconds);
        self.ensure_onscreen(win_size);
    }
}
