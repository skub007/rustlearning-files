use std::io;

use rand::prelude::*;
use rand::rngs;

fn main() {
    let mut rng = rngs::ThreadRng::default();
    let rnn1 = rng.gen_range(-10..=0);
    let rnn2 = rng.gen_range(0..=10);
    let rn = rng.gen_range(rnn1..=rnn2);
    println!(
        "A Random Number has been generated from range {} to {} what do you think it is ? -> ",
        rnn1, rnn2
    );
    let mut unn = String::new();
    io::stdin()
        .read_line(&mut unn)
        .expect("Enter One Number....");
    let un = unn.trim().parse::<i32>().expect("err converting...");
    //let un  =
    match rn != un {
        true => {
            println!("Err...U R WRONG LOL...TRY AGAIN!!");
        }
        false => {
            println!("Yay! You won!!!!");
        }
    }
}
