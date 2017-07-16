use ball::WhoScored;

use sfml::graphics::*;
use sfml::system::Vector2u;

pub struct Scoreboard<'s> {
    text: Text<'s>,
    score: Score,
}

#[derive(Default)]
struct Score {
    left: u8,
    right: u8,
}

impl<'s> Scoreboard<'s> {
    pub fn new(win_size: &Vector2u, font: &'s Font) -> Scoreboard<'s> {
        let mut text = Text::new_init("0    0", font, 60);
        let bounds = text.global_bounds();
        text.set_position2f(win_size.x as f32 / 2. - bounds.width / 2.,
                            win_size.y as f32 / 2. - bounds.height / 2.);
        text.set_fill_color(&Color::white());
        
        Scoreboard {
            text,
            score: Default::default(),
        }
    }
    
    pub fn on_score(&mut self, who: WhoScored) {
        match who {
            WhoScored::Left => self.score.left += 1,
            WhoScored::Right => self.score.right += 1,
            _ => {},
        };
        
        self.update();
    }
    
    fn update(&mut self) {
        self.text.set_string(&format!("{}    {}", self.score.left, self.score.right));
    }
}

impl<'s> Drawable for Scoreboard<'s> {
    fn draw<'se, 'tex, 'sh, 'shte>(
        &'se self, 
        target: &mut RenderTarget, 
        states: RenderStates<'tex, 'sh, 'shte>
    )
    where 'se: 'sh {
        target.draw_text(&self.text, states);
    }
}
