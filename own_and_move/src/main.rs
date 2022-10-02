fn main() {
    println!("Hello, world!");
}

fn print_padovan() {
    let mut padovan = vec![1,1,1]; // 確保
    for i in 3..10 {
        let next = padovan[i-3] + padovan[i-2];
        padovan.push(next);
    }
    println!("P(1..10) = {:?}", padovan); // 解放
}
