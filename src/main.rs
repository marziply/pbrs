use std::env::args;
use std::fs::File;

fn main() {
  let proto_path = args()
    .nth(1)
    .expect("Missing file path");
  let proto_file = File::open(proto_path).expect("File not found");

  println!("{:?}", proto_file);
}
