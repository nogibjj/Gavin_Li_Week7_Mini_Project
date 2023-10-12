use std::collections::HashMap;
use clap::Parser;

fn logic(numbers: Vec<i32>) -> Vec<(i32, u32)> {
    let mut frequencies = HashMap::new();

    for num in numbers {
        let frequency = frequencies.entry(num).or_insert(0);
        *frequency += 1;
    }

    let mut result = Vec::new();

    for (num, frequency) in frequencies {
        result.push((num, frequency));
    }

    result
}

/// cli tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    ///
    #[arg(short, long)]
    default: bool,

    #[arg(short,long,default_value="None")]
    cust_list: String,
}

fn string_to_list(string: String) -> Vec<i32> {
    let numbers: Vec<i32> = string.split(',')
                                  .filter_map(|n| n.parse().ok())
                                  .collect();
    println!("{:?}", numbers);
    numbers
}

fn main() {
    let defualt_nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 3];
    let args = Args::parse();
    if args.default {
        let result = logic(defualt_nums);
        println!("The count of each number is {:?}", result);
    } else {
        if args.cust_list == "None" {
            println!("Please use default list, or use --cust_list flag to input customized list");
        } else {
            // deal with the string list
            println!("The count of each number is {:?}",
                     logic(string_to_list(args.cust_list)));
        }
    }
}