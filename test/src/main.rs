extern crate boehm;

#[deriving(Show)]
struct Foo {
    a : int,
    b : int
}

fn main() {

  boehm::init();

  println!("Bytes Free: {}", boehm::free_bytes());

  let a = Foo{ a : 1i, b : 2i };
  let foo = boehm::Boehm::new(a);

  println!("Bytes Free: {}", boehm::free_bytes());

  let mut foo2 = boehm::Boehm::new(Foo{ a : 100i, b : 200i });

  println!("Bytes Free: {}", boehm::free_bytes());

  println!("{}", foo);
  println!("{}", foo2);

  *foo2 = *foo;
  println!("{}", foo);
  println!("{}", foo2);
}