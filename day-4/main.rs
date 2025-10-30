fn greet() {
    println!("Hello, Hunterdii!");
}

fn main() {
    greet();
}

fn greet_user(name:&str){
    println!("Hello{}!",name);
}
fn main(){
  greet_user("kehinde");
}

fn add(a:i32 , c:i64) -> i32{
    a + c;
}

fn main(){
  let sum = add(5, 3);
  println!("The sum of {} , {}", sum)
}

fn outter_function(){
   fn inner_function(){
    println!("THE VALUE OR FUNCTION IS INNER");
   }
   inner_function();
}
fn main(){
  outter_function();
}

// caluclate the double values that are used to be able to work with more than one operators
  fn caluclates(a:i32 , b:i32 ) -> (i32 , i32){
     (a + b , a * b);
  }
  fn main(){
   let (sum , multiply) = caluclates(5 , 3);
   println!("The total value is {}", sum , multiply);
  }