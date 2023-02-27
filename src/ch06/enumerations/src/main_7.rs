fn main() {
  let mut some: Option<i32>;
  some = Option::Some(10);
  some = None;

  if some == None {
      println!("some is none");
  } else {
      println!("{:#?}", some);
  }
}
