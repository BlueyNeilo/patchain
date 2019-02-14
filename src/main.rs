//Patrick Neilson 2019
//Project built following this video series by GeekLaunch: https://www.youtube.com/watch?v=vJdT05zl6jk
//
//A blockchain side project started in Feburary 2019
//
use patchainlib::*;

fn main() {
    let bl = Block::new(0, 0, vec![0; 32], 0, "First block".to_owned());
    println!("{:?}", &bl);
}
