use sfml::graphics::*;
use sfml::system::{Vector2f, Vector2u};

const SIZE: f32 = 32.;
const DEFAULT_VELOCITY: f32 = -128.;

pub struct Ball<'s> {
    rect: RectangleShape<'s>,
    velocity: Vector2f,
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
            velocity: Self::new_velocity(),
        }
    }
    
    pub fn restart(&mut self) {
        self.velocity = Self::new_velocity();
    }
    
    pub fn update(&mut self, win_size: &Vector2u, delta_seconds: f32) {
        self.rect.move_(&(self.velocity * delta_seconds));
        
        let current_pos = self.rect.position();
        
        let lower_border = win_size.y as f32 - self.rect.size().y;
        let upper_border = 0.;
        
        if self.rect.position().y > lower_border {
            //println!("offscreen");
            self.rect.set_position2f(current_pos.x, lower_border);
            self.velocity.y = -self.velocity.y;
        } else if self.rect.position().y < upper_border {
            self.rect.set_position2f(current_pos.x, upper_border);
            self.velocity.y = -self.velocity.y;
        }
    }
    
    fn new_velocity() -> Vector2f {
        Vector2f::new(DEFAULT_VELOCITY, DEFAULT_VELOCITY)
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
