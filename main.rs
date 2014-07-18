
use std::task::spawn;

enum List<T> {
  Cons(T, Box<List<T>>),
  Nil
}

static BASE_NUM: int = 10;

// try to define a struct in rust
struct Point {
  x: int,
  y: int
}

fn main() {
 
  fn call_twice(f: ||) { f(); f(); }
  let closure = || { 
    println!("I'm a closure, and it doesn't matter what type I am") 
  };
  fn function() { 
    println!("I'm a normal function")
  }
  call_twice(closure);
  call_twice(function);


  let mut x = 10i;
  let closure = || -> () {
    x += 10i;
    println!("x->{}", x)
  };
  closure();
  closure();

  // use owned closures
  spawn(proc() {
    println!("i'm new task");
  });

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

  let list = Cons::<int>(1, box Cons(2, box Cons(3, box Nil)));

  // shallowly copy
  let x = box 5i;
  let y = x.clone();

  println!("shallowly copy: {}", y);


}

fn inc_point(p: Point) -> Point {
  Point {
    x: p.x + 1,
    y: p.y + 2
  }
}

