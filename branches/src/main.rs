
// &s1をcalcuate_lengthに渡し、その定義では、String型ではなく、&Stringを受け取っている
// &記号が参照
fn main() {
  let s1 = String::from("hello");
  let len = calculate_length(&s1);
  // '{}'の長さは、{}です
  println!("The length of '{}' is {}.", s1, len);

  change_string();
}

// 一つしか可変な参照を持てない
// この制約がある利点は、コンパイラがコンパイル時にデータ競合を防ぐことができる点です。 データ競合とは、競合条件と類似していて、これら3つの振る舞いが起きる時に発生します:
// - 2つ以上のポインタが同じデータに同時にアクセスする。
// - 少なくとも一つのポインタがデータに書き込みを行っている。
// - データへのアクセスを同期する機構が使用されていない。
fn change_string () {
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
