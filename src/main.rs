extern crate pong;
extern crate sfml;

use pong::*;
use sfml::graphics::*;
use sfml::system::Clock;
use sfml::window::{Event, Key};

fn main() {
    let mut win = create_window();
    let size = win.size();
    
    let font = Font::from_file("res/UbuntuMono-R.ttf").unwrap();
    let mut sb = Scoreboard::new(&size, &font);
    
    let mut padl = paddle::Paddle::new(&size, paddle::Side::Left);
    let mut padr = paddle::Paddle::new(&size, paddle::Side::Right);
    
    let mut ball = Ball::new(&size);
    
    let mut clock = Clock::start();
    'game: loop {
        let delta = clock.restart().as_seconds();
        
        win.clear(&Color::rgb(2, 10, 40));
        win.draw(&sb);
        win.draw(&padl);
        win.draw(&padr);
        win.draw(&ball);
        win.display();
        
        
        let keys = playerinput::handle_input();
        
        if keys.l_up {
            padl.on_up(&size, delta);
        } else if keys.l_down {
            padl.on_down(&size, delta);
        }
        
        if keys.r_up {
            padr.on_up(&size, delta);
        } else if keys.r_down {
            padr.on_down(&size, delta);
        }
        
        ball.update(&size, padl.rect(), padr.rect(), delta);
        
        let who_scored = ball.check_scoring(&size);
        match who_scored {
            WhoScored::Left | WhoScored::Right => {
                ball.restart(&size);
                sb.on_score(who_scored);
            },
            _ => {},
        }
        
        while let Some(e) = win.poll_event() {
            match e {
                Event::Closed | Event::KeyPressed { code: Key::Escape, .. }
                    => break 'game,
                
                _ => {},
            }
        }
    }
}
