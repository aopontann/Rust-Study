const MAX_POINTS: u32 = 100_000;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);     // xの値は{}です
    x = 6;
    println!("The value of x is: {}", x);
    let y = 5;
    println!("The value of y is: {}", y);
    let y = y * 2;
    println!("The value of y is: {}", y);

    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);
}
