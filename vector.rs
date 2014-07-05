
fn main() {
  let fix_zeros: [int, ..6] = [0, ..6];
  let mut i: uint = 0;
  loop {
    if i > 5 {
      break;
    } else {
      println!("{}->{}", i, fix_zeros[i]);
      i += 1;
    }
  } 
}
