fn main(){
   let x = String::from("Hello world");
   print_str(x);
   println!("{}",x);
}

fn print_str(data:String){
    println!("{}",data);
}