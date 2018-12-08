
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);

    let mut a = {
        let x = 3;
        x + 1
    }; //This is an expression as it returns a value

    println!("The value of a is {}", a);
}

fn plus_one(x: i32) -> i32 {
    // This will return error if we place semicolon at the end, as it would become a statement.
    // Statements doesn't return value, without semicolon it is considered as expression.
    x + 1
}
