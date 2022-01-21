fn main() {
    let number = 6;

    if number < 5 {
        println!("condition was true");       // 条件は真でした
    } else {
        println!("condition was false");      // 条件は偽でした
    }

    let condition = false;
    let number = if condition {
        5
    } else {
        6
        // "six" にするとコンパイルエラーになる
    };

    // numberの値は、{}です
    println!("The value of number is: {}", number);
}
