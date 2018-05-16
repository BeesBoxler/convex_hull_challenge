extern crate rand;

mod utilities;
use rand::Rng;
use utilities::Point as Point;

fn generate_points(count:i32) -> Vec<Point> {
    let mut rng = rand::thread_rng();
    let mut points: Vec<Point> = Vec::new();
    for _ in 0..count {
        let x:i32 = rng.gen_range(0,100); // No negtives, 0-100 seems more than enough
        let y:i32 = rng.gen_range(0,100);
        points.push(Point{x,y})
    } 
    return points
}

fn main() {

    let mut path: Vec<Point> = Vec::new();

    // Let's make a point (badum tss)
    let mut points = generate_points(16);
    for p in &points {
        println!("{}", p)
    }

    // Lets find the bottom-most point
    let mut min = points[0].clone();
    for p in &points {
        if p.y < min.y {
            min = *p;
        } else if p.y == min.y { // And if there is more than one, chose the left-most 
            if p.x < min.x {
                min = *p
            }
        }
    }
    println!("Minimum point: {}", min);


    //and sort all the points by angle made with point (min) and the x axis

    utilities::sort_by_angle(&mut points, min);

    path.push(points[0].clone());
    path.push(points[1].clone());
    path.push(points[2].clone());

    // At the moment this only keeps turning right. I need to check if there are any
    // points to the left. This is also backwards compared to the actual algorithm which
    // works counter-clockwise as opposed to this which works clockwise. 
    
    for (i,p) in points[..].into_iter().enumerate() {
        if i < 3 { continue }
        loop {
            match p.direction_from(&path[path.len()-2], &path[path.len()-1]) {
                utilities::CrossProduct::Left => { path.push(*p); break},
                _ => { path.pop(); }
            }
        }
    }

    for p in path {
        println!("{}", p)
    }
}
