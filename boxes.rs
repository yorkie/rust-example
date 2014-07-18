
use std::os;
use std::io::process::{Command,InheritFd,ExitStatus,ExitSignal};

fn main() {
  
  let (cmd, args) = process(os::args());
  println!("cmd:{}, args:{}", cmd, args);
  
  let x = box foo();
  println!("{}", x);

  let x = box 5i;
  let mut y = box 5i;
  *y += 2;
}

fn foo() -> (u32, u32) {
  (5, 5)
}

fn process(args: Vec<String>) -> (String, Vec<String>) {
  let mut args = Vec::from_slice(args.tail());
  let head = args.shift().unwrap_or("--help".to_str());
  (head, args)
}

