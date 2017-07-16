//use std::num::sqrt;

use sfml::graphics::*;
use sfml::system::{Vector2f, Vector2u};

const SIZE: f32 = 32.;
const DEFAULT_VELOCITY: f32 = 425.;
const MIN_VELOCITY: f32 = 300.;
const SLOWING_FACTOR: f32 = 0.9999;
const SPEEDING_FACTOR: f32 = 2.;

pub struct Ball<'s> {
    rect: RectangleShape<'s>,
    velocity: Vector2f,
    last_score: WhoScored,
}

#[derive(Clone, Copy, Debug)]
pub enum WhoScored {
    Neither,
    Left,
    Right,
}

impl<'s> Ball<'s> {
    pub fn new(win_size: &Vector2u) -> Ball<'s> {
        let mut rect = RectangleShape::with_size(&Vector2f::new(SIZE, SIZE));
        let size = rect.size();
        rect.set_position2f(win_size.x as f32 / 2. - size.x / 2.,
                            win_size.y as f32 / 2. - size.y / 2.);
        rect.set_fill_color(&Color::rgb(200, 75, 10));
        
        Ball {
            rect,
            velocity: Self::new_velocity(WhoScored::Neither),
            last_score: WhoScored::Neither,
        }
    }
    
    pub fn restart(&mut self, win_size: &Vector2u) {
        let size = self.rect.size();
        
        self.rect.set_position2f(win_size.x as f32 / 2. - size.x / 2.,
                                 win_size.y as f32 / 2. - size.y / 2.);
        self.velocity = Self::new_velocity(self.last_score);
        
        //println!("{:?}", self.last_score);
    }
    
    pub fn update(&mut self, win_size: &Vector2u, padl: FloatRect, padr: FloatRect, delta_seconds: f32) {
        self.rect.move_(&(self.velocity * delta_seconds));
        
        let current_pos = self.rect.position();
        
        let bounds = self.rect.global_bounds();
        if bounds.left < padl.left + padl.width && bounds.top + bounds.height > padl.top && bounds.top < padl.top + padl.height {
            self.rect.set_position2f(padl.width, current_pos.y);
            self.velocity.x = -self.velocity.x;
            self.velocity *= SPEEDING_FACTOR;
        } else if bounds.left + bounds.width > padr.left && bounds.top + bounds.height > padr.top && bounds.top < padr.top + padr.height {
            self.rect.set_position2f(win_size.x as f32 - padr.width - bounds.width, current_pos.y);
            self.velocity.x = -self.velocity.x;
            self.velocity *= SPEEDING_FACTOR;
        }
        
        
        // Bounce off top and bottom.
        let lower_border = win_size.y as f32 - self.rect.size().y;
        let upper_border = 0.;
        
        if self.rect.position().y > lower_border {
            //println!("offscreen");
            self.rect.set_position2f(current_pos.x, lower_border);
            self.velocity.y = -self.velocity.y;
            self.velocity *= SLOWING_FACTOR * SLOWING_FACTOR;
        } else if self.rect.position().y < upper_border {
            self.rect.set_position2f(current_pos.x, upper_border);
            self.velocity.y = -self.velocity.y;
            self.velocity *= SLOWING_FACTOR * SLOWING_FACTOR;
        }
        
        if (self.velocity.x * self.velocity.x + self.velocity.y * self.velocity.y).sqrt() > MIN_VELOCITY {
            self.velocity *= SLOWING_FACTOR;
        }
        
        /*
        if self.velocity.x < MIN_VELOCITY && self.velocity.y < MIN_VELOCITY {
            self.velocity.x += MIN_VELOCITY * if self.velocity.x < 0. { -1. } else { 1. };
            self.velocity.y += MIN_VELOCITY * if self.velocity.y < 0. { -1. } else { 1. };
        }*/
    }
    
    pub fn check_scoring(&mut self, win_size: &Vector2u) -> WhoScored {
        let left_border = 0.;
        let right_border = win_size.x as f32 - self.rect.size().x;
        
        if self.rect.position().x < left_border {
            self.last_score = WhoScored::Right;
            return WhoScored::Right;
        } else if self.rect.position().x > right_border {
            self.last_score = WhoScored::Left;
            return WhoScored::Left;
        } else {
            self.last_score = WhoScored::Neither;
            return WhoScored::Neither;
        }
    }
    
    fn new_velocity(last_score: WhoScored) -> Vector2f {
        // The player that last scored gets the ball.
        match last_score {
            WhoScored::Left => Vector2f::new(-DEFAULT_VELOCITY, -DEFAULT_VELOCITY / 2.),
            _ => Vector2f::new(DEFAULT_VELOCITY, DEFAULT_VELOCITY / 2.),
        }
    }
}

impl<'s> Drawable for Ball<'s> {
    fn draw<'se, 'tex, 'sh, 'shte>(
        &'se self, 
        target: &mut RenderTarget, 
        states: RenderStates<'tex, 'sh, 'shte>
    )
    where 'se: 'sh {
        target.draw_rectangle_shape(&self.rect, states);
    }
}
