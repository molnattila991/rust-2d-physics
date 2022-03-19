pub mod Body {
    use sdl2::{rect::Point, render::Canvas, video::Window, pixels::Color};
    use crate::libs::draw::Draw;

    pub struct Rectangle {
        point: Point,
        width: i32,
        height: i32,
        color: Color,
        points: [Point; 5]
    }

    impl Rectangle {
        pub fn new(point: Point, width: i32, height: i32, color: Color) -> Self {
            Self{
                point,
                width,
                height,
                color,
                points : [
                    Point::new(point.x, point.y), 
                    Point::new(point.x + width, point.y), 
                    Point::new(point.x + width, point.y + height), 
                    Point::new(point.x, point.y + height),
                    Point::new(point.x, point.y)
                    ]
            }
        }

        pub fn collide(&self, other: &Rectangle) -> bool {
            if self.point.x() == other.point.x() {
                return true;
            }

            false
        }
    }

    pub trait GameBody{
        fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String>;
    }

    impl GameBody for Rectangle {
        fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
            canvas.set_draw_color(self.color);

            let result = canvas.draw_lines_with_color(&self.points[..], self.color);
            
            Ok(())
        }   
    }
}