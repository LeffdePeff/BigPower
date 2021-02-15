use gmp::mpz::Mpz;

use std::fs;
use std::io::stdin;

fn main()
{   
    let first_number = Mpz::from(get_input("Enter first number: ").trim().parse::<i32>().unwrap());
    let second_number = get_input("Enter second number: ").trim().parse::<u32>().unwrap();
    let calculate_length_bool:bool = get_input("Calculate length? true/false").trim().parse().unwrap();
    let file_name = first_number.to_string() + "^" + second_number.to_string().as_str() + "_answer.txt";
    let file_name_length = first_number.to_string() + "^" + second_number.to_string().as_str() + "_length.txt";

    println!("Calculating...");
    let answer = first_number.pow(second_number);

    println!("Converting answer to string...");
    let answer_string = answer.to_string();

    if calculate_length_bool {
        println!("Calculating length...");
        let answer_length = answer_string.len();

        println!("The answer has {} numbers.", answer_length);
        
        if answer_length <= 20 {
            println!("The answer is: {}", answer_length);
        }

        println!("Writing length to {} ...", file_name_length);
        write_to_file(file_name_length, answer_length.to_string());
    }

    println!("Writing answer to {} ...", file_name);
    write_to_file(file_name, answer_string);

    println!("Done!");
}

fn write_to_file(file_name: String, content: String)
{
    fs::write(file_name, content).expect("Unable to write file");
}

fn get_input(pr: &str) -> String {
    println!("{}", pr);
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("Failed");
    buffer
}