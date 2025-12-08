fn main(){
    println!("Hello, world!");
    another_function();
    sum(1,2);

    let s = sum2(1,2);
    println!("{}",s);

    let five =five();
    println!("{}",five);
}

// в Rust все определяеться в snake_case
fn another_function(){
    println!("Hello from another function");
}

fn sum(x:i32,y:i32){
    println!("The sum of {} and {} is {}",x,y,x+y);
}

// ; нельзя ставить на возвращаемое значения
fn sum2(x:i32,y:i32)->i32{
    x+y
}

fn five()->i32{
    5
}
