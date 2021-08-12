use std::io;

fn main() {
  println!("Select the verse: ");
  
  let mut sel_verse = String::new();
  
  io::stdin().read_line(&mut sel_verse)
    .expect("Failed to read line");
  
  let verse: u32 = sel_verse.trim().parse()
    .expect("Please type a number");

  // println!("verse {}", verse);
  christmas_carol(verse);
}

fn christmas_carol(x: u32) {
  println!("Hello {}", x);
}
