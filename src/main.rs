extern crate rand;

mod types;
use rand::Rng;

use types::Point as Point;

fn generate_points(count:i32) -> Vec<Point> {
    let mut rng = rand::thread_rng();
    let mut points: Vec<Point> = Vec::new();
    for _ in 0..count {
        let x:i32 = rng.gen_range(0,100);
        let y:i32 = rng.gen_range(0,100);
        points.push(Point{x,y})
    } 
    return points
}

fn main() {


    let p1 = Point { x: 12, y: 10};
    let p2 = Point { x: 102, y: 10};
    let p3 = Point { x: 102, y: 100};

    println!("{}",p1.angle_with(&p2));
    println!("{}",p1.direction_from(&p2, &p3));

    let points = generate_points(16);
    
    for point in &points {
        println!("{}", point)
    }
}
