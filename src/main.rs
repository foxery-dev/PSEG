use text_io::read;

mod evaluate;

fn main() {
  print!("Enter Password for Evaluation: ");
  let password: String = read!();
  println!("{}", evaluate::evaluate(password.as_str()));
}
