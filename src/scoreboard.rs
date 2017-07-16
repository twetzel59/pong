use sfml::graphics::*;
use sfml::system::{Vector2f, Vector2u};

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
        let mut text = Text::new_init("test", font, 30);
        text.set_fill_color(&Color::white());
        
        Scoreboard {
            text,
            score: Default::default(),
        }
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
