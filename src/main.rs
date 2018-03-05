#![deny(missing_docs)]

//! A RepGame2 game.

extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate sdl2_window;


use piston::window::WindowSettings;
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::RenderEvent;
use sdl2_window::Sdl2Window;
use opengl_graphics::{Filter, GlGraphics, GlyphCache, OpenGL, TextureSettings};

pub use gameboard::Gameboard;
pub use gameboard_controller::GameboardController;
pub use gameboard_view::{GameboardView, GameboardViewSettings};

mod gameboard;
mod gameboard_controller;
mod gameboard_view;

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("RepGame2", [400; 2])
        .opengl(opengl)
        .exit_on_esc(true)
        .fullscreen(false)
        .resizable(true)
        .vsync(false);
    let mut window: Sdl2Window = settings.build().expect("Could not create window");

    let mut events = Events::new(EventSettings::new().lazy(false));
    let mut gl = GlGraphics::new(opengl);

    let gameboard = Gameboard::new();
    let mut gameboard_controller = GameboardController::new(gameboard);
    let gameboard_view_settings = GameboardViewSettings::new();
    let gameboard_view = GameboardView::new(gameboard_view_settings);

    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let ref mut glyphs = GlyphCache::new("assets/FiraSans-Regular.ttf", (), texture_settings)
        .expect("Could not load font");

    while let Some(e) = events.next(&mut window) {
        gameboard_controller.event(
            gameboard_view.settings.position,
            gameboard_view.settings.size,
            &e,
        );
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::clear;

                clear([1.0; 4], g);
                gameboard_view.draw(&gameboard_controller, glyphs, &c, g);
            });
        }
    }
}
