// 作业 4
// 为枚举交通信号灯实现一个 trait，trait里包含一个返回时间的方法，不同的灯持续的时间不同

fn main() {
  println!("Hello, world! work 4");
  println!("light red time: {}", Light::Red.time());
  println!("light green time: {}", Light::Green.time());
  println!("light yellow time: {}", Light::Yellow.time());
}

enum Light {
  Red,
  Green,
  Yellow,
}

impl Light {
  fn time(&self) -> u8 {
      match self {
          Light::Red => 50,
          Light::Green => 60,
          Light::Yellow => 3,
      }
  }
}
