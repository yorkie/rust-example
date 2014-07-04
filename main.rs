
static BASE_NUM: int = 10;

// try to define a struct in rust
struct Point {
  x: int,
  y: int
}

fn main() {
  
  let mut hi: int = BASE_NUM;
  loop {
    hi = hi - 1i;
    if hi == 0i {
      break;
    }
    println!("hi -> {}", hi);
  }

  // redefine
  let hi = "haha";
  println!("hi, got a string -> {}", hi);

  // test match
  let age = 14i;
  match age*2 {
    a @ 0..40 => println!("{} years old", a),
    _ => println!("older than 21")
  }

  // test struct: Point
  let mp = Point {
    x: 1,
    y: 10
  };
  
  match inc_point(mp) {
    p @ Point { x: 1..2, y: yy } => println!("{},{}", p.x, yy),
    _ => println!("other!")
  }
}

fn inc_point(p: Point) -> Point {
  Point {
    x: p.x + 1,
    y: p.y + 2
  }
}

