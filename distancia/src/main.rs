use std::ops::Deref;

fn main() {
    let p: Point = Point { x: 0.0, y: 4.0 };
    let q: Point = Point { x: 3.0, y: 0.0 };
    println!("{}", distance(&p, &q));
    let a: ColoredPoint = ColoredPoint {
        point: p,
        color: 1,
        temp: Some(37.5),
    };
    println!("x:{} y:{} t:{:?}", a.point.x, a.point.y, a.temp);
}

fn distance(p: &Point, q: &Point) -> f64 {
    p.distance(q)
}

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance(&self, q: &Point) -> f64 {
        ((self.x - q.x).powi(2) + (self.y - q.y).powi(2)).sqrt()
    }
}

struct ColoredPoint {
    point: Point,
    color: u32,
    temp: Option<f64>,
}

impl Deref for ColoredPoint {
    type Target = Point;
    fn deref(&self) -> &Self::Target {
        &self.point
    }
}

impl AsRef<u32> for ColoredPoint {
    fn as_ref(&self) -> &u32 {
        &self.color
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn points() {
        let p = Point { x: 0.0, y: 4.0 };
        let q = Point { x: 3.0, y: 0.0 };
        assert_eq!(distance(&p, &q), 5.0);
    }

    #[test]
    fn colored_points() {
        let p = ColoredPoint {
            point: Point { x: 0.0, y: 4.0 },
            color: 1,
            temp: None,
        };
        let q = ColoredPoint {
            point: Point { x: 3.0, y: 0.0 },
            color: 2,
            temp: None,
        };
        assert_eq!(distance(&p, &q), 5.0);
    }

    #[test]
    fn test_color() {
        let p = ColoredPoint {
            point: Point { x: 0.0, y: 4.0 },
            color: 1,
            temp:None
        };
        let n: &u32 = p.as_ref();
        println!("Color: {}", n);
        assert_eq!(n, &1);
    }
}
