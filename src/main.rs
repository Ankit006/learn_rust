fn main()
{
    for val in 1..6{
        println!("{val}");
    }

    println!("------------------");
    // print values in reverse order
    for val in (1..6).rev(){
        println!("{val}");
    }
}