#[macro_use]
mod impl_macros;
mod divisions;
mod lazy_buffer;

fn main() {
  let it = (1..5).into_iter().combinations(3).collect();
  println!("Hello, world! {:?}", it);
}
