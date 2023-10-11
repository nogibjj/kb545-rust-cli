use clap::Parser;
use cli_salad::create_fruit_salad;
use cli_salad::add_veggies;
use std::io;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Your Name <your.email@example.com>",
    about = "Number of fruits and Veggies to include in the salad"
)]
struct Opts {
    #[clap(long)]
    num_fruit: usize,
    #[clap(long)]
    num_veg: usize,
}

fn main() {
    let opts: Opts = Opts::parse();

    // Get the number of fruits the user requested
    let num_fruits = opts.num_fruit;
    let num_veg = opts.num_veg;

    // Create the fruit salad
    let only_fruit = create_fruit_salad(num_fruits);
    let fruit_and_veggie = add_veggies(only_fruit, num_veg);

    // Print the fruit salad in human readable format with a count of fruits used
    println!(
        "The current salad has {} fruits and {} veggies: {:?}",
        num_fruits,
        num_veg,
        fruit_and_veggie
    );
    println!("Do you want to add something to the salad? (y/n)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if input.trim() == "y"{
        println!("What do you want to add?");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut final_salad = fruit_and_veggie.clone();

        final_salad.push(input.trim().to_string());
        println!("Now your salad is {:?}", final_salad)
    }else{
        print!("Sick, you're done.")
    }
}