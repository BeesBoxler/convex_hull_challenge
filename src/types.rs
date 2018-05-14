use std::fmt;

pub enum CrossProduct {
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

pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn angle_with(&self, p2: &Point) -> f32{
        return f32::atan(((p2.y-self.y)/(p2.x-self.x)) as f32)
    }

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