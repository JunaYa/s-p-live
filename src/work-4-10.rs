trait AreaCalculable {
  fn area(&self) -> f64;
}

struct Circle {
  radius: f64,
}

impl AreaCalculable for Circle {
  fn area(&self) -> f64 {
      std::f64::consts::PI * self.radius * self.radius
  }
}

struct Triangle {
  base: f64,
  height: f64,
}

impl AreaCalculable for Triangle {
  fn area(&self) -> f64 {
      0.5 * self.base * self.height
  }
}

struct Square {
  side: f64,
}

impl AreaCalculable for Square {
  fn area(&self) -> f64 {
      self.side * self.side
  }
}

fn print_area<T: AreaCalculable>(shape: T) {
  println!("Area: {}", shape.area());
}

fn main() {
  let circle = Circle { radius: 5.0 };
  let triangle = Triangle { base: 3.0, height: 4.0 };
  let square = Square { side: 2.5 };

  print_area(circle);
  print_area(triangle);
  print_area(square);
}