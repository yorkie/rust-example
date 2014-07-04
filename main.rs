
fn main() {
  let mut hi = 10i;
  loop {
    hi = hi - 1i;
    if hi == 0i {
      return;
    }
    println!("hi -> {}", hi);
  }
}

