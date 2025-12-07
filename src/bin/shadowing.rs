fn main(){
    let x = 5;
    let x = x + 1;

    //изолированная среда
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);


    let spaces = "12345";
    println!("The string '{}'", spaces);
    let spaces = spaces.len();
    println!("The length of '{}'", spaces);

}
