fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
    let guess: u32 = "42".parse().expect("Not a number!");
}

const MAX_POINTS: u32 = 100_000;

fn calc() {
    // addition
    // 足し算
    let sum = 5 + 10;

    // subtraction
    // 引き算
    let difference = 95.5 - 4.3;

    // multiplication
    // 掛け算
    let product = 4 * 30;

    // division
    // 割り算
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0
    // 結果は0

    // remainder
    // 余り
    let remainder = 43 % 5;
}
