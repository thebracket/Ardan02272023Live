struct Radians(f32);
struct Degrees(f32);

impl Degrees {
    fn to_radians(&self) -> Radians {
        Radians(self.0.to_radians())
    }
}

impl From<Degrees> for Radians {
    fn from(value: Degrees) -> Self {
        Radians(value.0.to_radians())
    }
}

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

fn project_angle<A: Into<Radians>>(start: Point, angle: A, radius: f32) -> Point {
    let angle: Radians = angle.into();
    Point {
        x: 0.0 - (start.x + radius * f32::sin(angle.0)),
        y: start.y + radius * f32::cos(angle.0),
    }
}

fn take_my_text<S: ToString>(text: S) {
    let text = text.to_string();
    // Work with the string
}

fn main() {
    let start = Point{x:0.0, y:0.0};
    let finish = project_angle(start, Degrees(180.0), 10.0);
    println!("{finish:?}");

    take_my_text("Hello!");
    take_my_text("Hello World!".to_string());

}