fn main() {
    let result = return_literal(1);
    println!("{result}");
}

/* 
fn another_function(x: i32, unit_label: char) {
    println!("The value of x is {x}{unit_label}");
}*/

fn return_literal(x: i32) -> i32 {
    x + 1
}