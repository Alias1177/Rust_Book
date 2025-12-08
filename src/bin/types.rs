
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;


fn main(){
    let a : i64 =33;
    let b : f64 = 1.11;
    let c : bool= true;
    let d : char = 'F';
    let name = String::from("Rust");
    let ex :&str="OK";//не безопастный

    let z = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {z}");

    //if в расте с возврощаемым значением может быть,нужно чекнуть
    let flag: bool =true;

    let num = if flag{
        1
    }else{
        2
    };

    let s ="Jong";
    let stroka: &str="qwerty";
    let srr =String::from("Jonh");

    println!("a={},b={},c={},d={},name={}",a,b,c,d,name);
    println!("{}",ex);
    println!("{}",num);
    println!("{}",s);
    println!("{}",srr);
    println!("{}",stroka);
    println!("{}",THREE_HOURS_IN_SECONDS);
}
