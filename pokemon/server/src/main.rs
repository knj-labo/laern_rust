#[actix_rt::main]
async fn main(){
  // HOST 0.0.0.0, PORT 4000
  let url = "0.0.0.0:4000";
  println!("server started on {} using actix", url);
}