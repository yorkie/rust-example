
enum Name {
    {
      x: int
    }
}

fn main() {
  struct House { owner: Box<Person> }
  struct Person { age: int }
  let mut house = box House {
    owner: box Person {age: 30}
  };
  
  {
    let mut age= &mut house.owner.age;
    *age += 10;
  }

  print!("{}\n", house.owner.age);
}
