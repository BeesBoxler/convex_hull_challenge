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
    println!("Generated points:" );
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
    path.push(min);


    //and sort all the points by angle made with point (min) and the x axis

    utilities::sort_by_angle(&mut points, min);

    points.push(min); //We want it to finish on the same point

    //Add the first three points to the path

    for i in 0..3 {
        path.push(points[i].clone());
    }

    // Graham Scan. 
    
    for (i,p) in points[..].into_iter().enumerate() {
        if i < 3 { continue } //Scip the points we've already added.
        loop {
            match p.direction_from(&path[path.len()-2], &path[path.len()-1]) {
                utilities::CrossProduct::Left => { path.push(*p); break}, //if it's a left turn, add it to the stack and continue
                _ => { path.pop(); } //Otherwise remove the topmost item from the stack and try again
            }
        }
    }
    println!();
    println!("Path:" );
    for p in path {
        println!("{}", p) //Print the path
    }
}
