use std::f32;

enum CrossProduct {
    Left,
    Right,
    Collinear
}

struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    fn angle_with(&self, p2: Point) -> f32{
        return f32::atan((p2.y-self.y)/(p2.x-self.x))
    }

    fn direction_from(&self, p1: Point, p2: Point) -> CrossProduct{
        let p3 = self;
        let val: f32 = (p2.x - p1.x)*(p3.y - p1.y)-(p2.y - p1.y)*(p3.x - p1.x);
        if val < 0.0 {
            return CrossProduct::Right
        } else if val > 0.0 {
            return CrossProduct::Left
        } else {
            return CrossProduct::Collinear
        }
    }


}

fn main() {

    let mut points: Vec<Point> = Vec::new();
    for i in 0..15 {

    } 
    println!("Hello, world!");
}
