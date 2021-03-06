extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use std::time::Duration;

use rust_2d_physics::*;

use rust_2d_physics::libs::draw::Draw;
use rust_2d_physics::libs::body::Body::{Rectangle, GameBody};

fn main() { 
    let body1 = Rectangle::new(Point::new(10,10), 100, 300, WHITE);
    let mut body2 = Rectangle::new(Point::new(100,10), 100, 30, RED);
    let mut direction: Point = Point::new(0,0);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-2d-physic", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    canvas.set_draw_color(LIGHT_BLUE);
    canvas.clear();
    canvas.present();


    'running: loop {
        canvas.set_draw_color(LIGHT_BLUE);
        canvas.clear();


        for event in event_pump.poll_iter() {
            match event {
                Event::MouseButtonDown { x, y, .. } => {
                    direction = Point::new(x, y);
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    direction += Point::new(0,-1);
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    direction += Point::new(0,1);
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    direction += Point::new(-1,0);
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    direction += Point::new(1,0);
                },
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        if body1.collide(&body2) {
            canvas.set_draw_color(GREEN);
        }

        canvas.clear();

        body2.setDirection(direction);

        body2.update();

        let result = body1.draw(&mut canvas);
        let result = body2.draw(&mut canvas);



        //let result = canvas.draw_rectangle_with_color(Point::new(100, 100), 150, 300, WHITE);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
