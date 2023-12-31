extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{EventLoop, ButtonEvent, ButtonState, Button, Key};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use std::collections::LinkedList;
use std::iter::FromIterator;

#[derive(Clone, PartialEq)]
enum Direction {
    RIGHT, LEFT, UP, DOWN
}

struct Game {
    gl: GlGraphics,
    snake: Snake,
}

impl Game {
    fn render(&mut self, args: &RenderArgs) {
        // use graphics;
        
        let green: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |_c, gl| {
            graphics::clear(green, gl); // Clear the screen.
        });

        self.snake.render(&mut self.gl, args);
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.snake.update()
    }

    fn pressed(&mut self, btn: &Button ) {

        let last_direction = self.snake.dir.clone();

        self.snake.dir = match btn {
            &Button::Keyboard(Key::Up) if last_direction != Direction::DOWN => Direction::UP,
            &Button::Keyboard(Key::Down) if last_direction != Direction::UP => Direction::DOWN,
            &Button::Keyboard(Key::Left) if last_direction != Direction::RIGHT => Direction::LEFT,
            &Button::Keyboard(Key::Right) if last_direction != Direction::LEFT => Direction::RIGHT,
            _ => last_direction,
        };
    }
}

struct Snake {
    body: LinkedList<(i32, i32)>,
    dir: Direction,
}

impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let red: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let squares: Vec<graphics::types::Rectangle> = self.body
            .iter()
            .map(|&(x, y)| {
                graphics::rectangle::square(
                    ( x * 20 ) as f64, 
                    ( y * 20 ) as f64, 
                    20_f64
                );
            })
            .collect();


        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            squares.into_iter()
                .for_each(|square| graphics::rectangle(red, square, transform, gl));
        });
    }

    fn update(&mut self) {
        let mut new_head = (*self.body.front().expect("Snake has no body")).clone();

        match self.dir {
            Direction::LEFT => new_head.0 -= 1,
            Direction::RIGHT => new_head.0 += 1,
            Direction::UP => new_head.1 -= 1,
            Direction::DOWN => new_head.1 += 1,
        }

        self.body.push_front(new_head);
        self.body.pop_back().unwrap();
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    
    let mut window: Window = WindowSettings::new( 
        "Snake Game", 
        [200, 200]
    )
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let snake = Snake {
        body: LinkedList::from_iter((vec![(0,0), (0,1)]).into_iter()),
        dir: Direction::RIGHT,
    };

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snake,
    };

    let mut events = Events::new(EventSettings::new()).ups(8);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }

        if let Some(args) = e.update_args() {
            game.update(&args);
        }

        if let Some(args) = e.button_args() {
            if args.state == ButtonState::Press {
                game.pressed(&args.button);
            }
        }
    }
}
