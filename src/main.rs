extern crate rand;

use std::fmt;
use std::f32;
use rand::Rng;

enum CrossProduct {
    Left,
    Right,
    Collinear
}

impl fmt::Display for CrossProduct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            CrossProduct::Left => "Left",
            CrossProduct::Right => "Right",
            CrossProduct::Collinear => "Collinear",
        };
        write!(f, "{}", printable)
    }
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    fn angle_with(&self, p2: &Point) -> f32{
        return f32::atan(((p2.y-self.y)/(p2.x-self.x)) as f32)
    }

    fn direction_from(&self, p1: &Point, p2: &Point) -> CrossProduct{
        let p3 = self;
        let val: f32 = ((p2.x - p1.x)*(p3.y - p1.y)-(p2.y - p1.y)*(p3.x - p1.x)) as f32;
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


    let p1 = Point { x: 12, y: 10};
    let p2 = Point { x: 102, y: 10};
    let p3 = Point { x: 102, y: 100};

    println!("{}",p1.angle_with(&p2));
    println!("{}",p1.direction_from(&p2, &p3));


    let mut rng = rand::thread_rng();
    let mut points: Vec<Point> = Vec::new();
    for _ in 0..15 {
        let x:i32 = rng.gen_range(0,100);
        let y:i32 = rng.gen_range(0,100);
        points.push(Point{x,y})


    } 
    for point in points {
        println!("{}", point)
    }
    println!("Hello, world!");
}
