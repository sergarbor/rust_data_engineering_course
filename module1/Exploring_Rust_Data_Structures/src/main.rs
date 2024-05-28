use clap::Parser;
use std::collections::VecDeque;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    collection_type: String,
}

fn main() {
    let my_arg = Args::parse();

    let collection = match my_arg.collection_type.as_str() {
        "Vector" => {
            let mut collection1: Vec<i32> = Vec::new();
            collection1.push(1);
            collection1.push(2);
            collection1
        },
        "VecDeque" => {
            let mut collection2: VecDeque<i32> = VecDeque::new();
            collection2.push_back(4);
            collection2.push_front(3);
            collection2.into()
        },
        _ => panic!("Invalid collection type"),
    };

    // Use the collection variable here
    println!("Created a {}", my_arg.collection_type);
    println!("Collection {:#?}", collection);
}
