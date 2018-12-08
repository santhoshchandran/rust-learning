fn main() {
    let x = 2.0;
    let y: f32 = 3.0;

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;


    let t = true;

    let f: bool = false; // with explicit type annotation

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;
   

    println!("The value of y is: {}", y);
    println!("The value of x is {}", tup.0);

    let mut arr = [21,34,4354,565,676];
    println!("The value of arr is {:?}", arr);
    println!("The value of arr 0 is {}", arr[0]);
    arr[4] = 56;
    println!("The value of arr 5 is {}", arr[4]);
    let mut arr2: [i8;10];
    arr2 = [1,2,3,4,5,6,7,8,9,0];
    println!("The value of arr2 is {:?}", arr2.len());
}
