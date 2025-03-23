fn get_nth_arg(n: usize) -> String{
    std::env::args().nth(n).unwrap()
}

#[derive(Debug)]
pub struct Args {
   pub image_01: String,
   pub image_02: String,
   pub output: String
}

impl Args {
    pub fn new() -> Self {
      Args {
      image_01: get_nth_arg(1),
      image_02: get_nth_arg(2),
      output: get_nth_arg(3)
      }
    }
}