struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
    fn only_f32(x: f32, y: f32) -> Point<f32, f32> {
        Point {x, y}
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    let new_f32_point: Point<f32, f32> = Point::<f32, f32>::only_f32(12.0, 12.0);

    // This will not work 
    // let new_i16_point: Point<u16, u16> = Point::<u16, u16>::only_f32(120, 120);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
