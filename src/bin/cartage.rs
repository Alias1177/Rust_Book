fn main(){
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value is: {},{},{}", x, y, z);

    let first = tup.0;
    println!("{}",first);
    
    let second = tup.1;
    println!("{}",second);
    
    let third = tup.2;
    println!("{}",third);
}
