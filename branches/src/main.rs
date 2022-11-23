
// &s1をcalcuate_lengthに渡し、その定義では、String型ではなく、&Stringを受け取っている
// &記号が参照
fn main() {
  let s1 = String::from("hello");

  let len = calculate_length(&s1);

  // '{}'の長さは、{}です
  println!("The length of '{}' is {}.", s1, len);
}


fn calculate_length(s: &String) -> usize {
    s.len()
}