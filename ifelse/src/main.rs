fn main() {
 for n in 1..101{
     if n %15 ==0 {
         println!("fizzbuz");
     }else if n % 3==0{
         println!("b=fizz");
     }else if n % 5 ==0{
         println!("buzz");
     }else{
         println!("{}",n);
     }
 }
}
