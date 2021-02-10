use gmp;

use std::fs;
use std::io::stdin;

fn main()
{   
    let num1 = gmp::mpz::Mpz::from(get_input("Enter first number: ").trim().parse::<i32>().unwrap());
    let num2 = get_input("Enter second number: ").trim().parse::<u32>().unwrap();
    let length_bool:bool = get_input("Calculate length? true/false").trim().parse().unwrap();
    let file_name = num1.to_string() + "^" + num2.to_string().as_str() + "_answer.txt";
    let file_name_length = num1.to_string() + "^" + num2.to_string().as_str() + "_length.txt";

    println!("Calculating...");
    let answer = num1.pow(num2);

    println!("Converting answer to string...");
    let str_answer = answer.to_string();

    if length_bool {
        println!("Calculating length...");
        let answer_len = str_answer.len();

        println!("The answer has {} numbers.", answer_len);
        
        if answer_len <= 20 {
            println!("The answer is: {}", str_answer);
        }

        println!("Writing length to {} ...", file_name_length);
        write_to_file(file_name_length, answer_len.to_string());
    }

    println!("Writing answer to {} ...", file_name);
    write_to_file(file_name, str_answer);

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