fn main(){
    let mut s1 = String::from("hello");
    let slice:&str = &s1[0..2];
    s1.clear();
    println!("{}",slice);
}