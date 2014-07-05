
fn main() {
  let x = box foo();
  println!("{}", x);

  let x = box 5i;
  let mut y = box 5i;
  *y += 2;
}

fn foo() -> (u32, u32) {
  (5, 5)
}

