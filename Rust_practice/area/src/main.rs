#[derive(Debug)]
struct Triangle {
    base: f32,
    height: f32,
}

#[derive(Debug)]
struct Rectangle {
    width: f32,
    height: f32,
}

#[derive(Debug)]
struct Circle {
    radius: f32,
}

trait Area {
    fn get_area(&self) -> f32;
}

impl Area for Triangle {
    fn get_area(&self) -> f32 {
        self.base * self.height / 2.0
    }
}

impl Area for Rectangle {
    fn get_area(&self) -> f32 {
        self.width * self.height
    }
}

impl Area for Circle {
    fn get_area(&self) -> f32 {
        self.radius * self.radius * std::f32::consts::PI
    }
}

fn print_area<T: Area>(shape: &T) {
    println!("Area: {}\n", shape.get_area());
}

fn main() {
    let tri = Triangle {base: 1.0, height: 2.0};
    println!("Shape: {:?}", tri);
    print_area(&tri);

    let rec = Rectangle {width: 1.0, height: 2.0};
    println!("Shape: {:?}", rec);
    print_area(&rec);

    let cri = Circle {radius: 1.0};
    println!("Shape: {:?}", cri);
    print_area(&cri);
}
