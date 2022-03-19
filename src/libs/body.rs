pub mod Body {
    use sdl2::{rect::Point, render::Canvas, video::Window, pixels::Color};
    use crate::libs::draw::Draw;

    pub struct Rectangle {
        point: Point,
        width: i32,
        height: i32,
        color: Color,
        points: [Point; 5],
        direction: Point,
    }

    impl Rectangle {
        pub fn new(point: Point, width: i32, height: i32, color: Color) -> Self {
            Self{
                point,
                width,
                height,
                color,
                direction : Point::new(0,0),
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
            if self.point.x() == other.point.x() || 
            self.point.y() == other.point.y()  {
                return true;
            }

            false
        }
    }

    pub trait GameBody{
        fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String>;
        fn update(&mut self) -> Result<(), String>;
        fn setDirection(&mut self, direction: Point);
    }

    impl GameBody for Rectangle {
        fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
            canvas.set_draw_color(self.color);

            let result = canvas.draw_lines_with_color(&self.points[..], self.color);
            
            Ok(())
        }
        
        fn update(&mut self) -> Result<(), String> {
            self.point += self.direction;
            self.points = [
                    Point::new(self.point.x + self.direction.x, self.point.y+ self.direction.y), 
                    Point::new(self.point.x + self.direction.x + self.width, self.point.y+ self.direction.y), 
                    Point::new(self.point.x + self.direction.x + self.width, self.point.y + self.direction.y + self.height), 
                    Point::new(self.point.x + self.direction.x, self.point.y + self.direction.y + self.height),
                    Point::new(self.point.x + self.direction.x, self.point.y+ self.direction.y)
                    ];
            Ok(())
        }

        fn setDirection(&mut self, direction: Point) {
            self.direction = direction;
        }
    }
}