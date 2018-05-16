use std::fmt;
use std::f64;


// The cross product is the result of (x3-x1)(y2-y1) - (y3-y1)(x2-x1)
// If the result is positive, p3 lies to the left of p1->p2, 0 means the
// point is collinear and a negative value shows the point lies to the right.
pub enum CrossProduct {
    Left,
    Right,
    Collinear
}

// Display CrossProduct in a friendly way (This is only for debugging)
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


// Are two cross products equal? ie. Are they both left or right?
impl PartialEq for CrossProduct {
    fn eq(&self, other: &CrossProduct) -> bool {
        self == other
    }
}

// Point type
#[derive(Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    // Compare the angle with another point against the x axis
    pub fn angle_with(&self, p: &Point) -> f64{
        if self.x == p.x { return  f64::consts::PI / 2.0 }
        return ((p.y-self.y) as f64).atan2((p.x-self.x) as f64) // Using atan2 because atan kept giving incorrect results
    }

    // Is a point going to turn left, right, or be collinear?
    pub fn direction_from(&self, p1: &Point, p2: &Point) -> CrossProduct{
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

// Compare points 
impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

// Clone points
impl Clone for Point {
    fn clone(&self) -> Self {
        *self
    }
}

// Display points (for println)
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}


// Bubblesort. Because for 16 points the inefficiency isn't really going to be an issue.
// If we were working with more than 16 points, this could be implimented as a quicksort
// and the angle could be cached.
pub fn sort_by_angle(a: &mut Vec<Point>, p: Point) {
    let mut n = a.len();
    loop {
        let mut swapped = false;
        for i in 1..n-1 {
            if p.angle_with(&a[i-1]) > p.angle_with(&a[i]) {
                a.swap(i-1, i);
                swapped = true
            }

            // If two points have the same angle, we only wnt the further of the two.
            if p.angle_with(&a[i-1]) == p.angle_with(&a[i]) { 
                let r1 = ((&a[i-1].x - p.x + &a[i-1].y - p.y) as f32).sqrt();
                let r2 = ((&a[i].x - p.x + &a[i].y - p.y) as f32).sqrt();
                if r1 != 0.0 && r2 != 0.0 {
                    if r1 < r2 {
                        a.remove(i-1); 
                    } else {
                        a.remove(i);
                    }
                }
            }
        }

        n = n -1;
        if !swapped {
            break
        }
    }
}