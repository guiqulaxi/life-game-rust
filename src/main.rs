extern crate piston_window;
extern crate rand;

mod drawing;
mod game;
use game::Game;
use drawing::to_gui_coord_u32;
use piston_window::*;
use piston_window::types::Color;

const BACK_COLOR: Color = [1.0, 1.0, 1.0, 1.0];

fn main() {
    
       let (width, height) = (100, 50);

    // Create a window
    let mut window: PistonWindow = WindowSettings::new("Life Game",
        [to_gui_coord_u32(width), to_gui_coord_u32(height)]).exit_on_esc(true).build().unwrap();

    let mut game = Game::new(width, height);
    // Event loop
    while let Some(event) = window.next() {
        
        // Catch the events of the keyboard
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.next();
        }

        // Draw all of them
        window.draw_2d(&event, |c, g| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });

        // Update the state of the game
        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
