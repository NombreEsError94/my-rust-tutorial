use std::io;

fn main(){
    println!("What do you want to do?");
    println!("1 - Convert from Celsius to Fahrenheit");
    println!("2 - Convert from Fahrenheit to Celsius");

    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading user input");


    let choice: i32 = match input.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Input not a number");
            return;
        }
    };

    println!("Input the number you want to convert");

    input.clear();

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading user input");

    let value: f32 = input.trim().parse().expect("Invalid number");

    let result: f32;

    if choice == 1 {
        result = convert_from_celsius_to_fahrenheit(value);
    }else{
        result = convert_from_fahrenheit_to_celsius(value);
    }

    println!("Result is {result}");
}

fn convert_from_celsius_to_fahrenheit(x: f32) -> f32{
    x * (9.0 / 5.0) + 32.0
}

fn convert_from_fahrenheit_to_celsius(x: f32) -> f32{
    (x - 32.0) * (5.0 / 9.0)
}