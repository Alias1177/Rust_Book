fn main(){
    println!("Hello, world!");
    another_function();
    sun(1,2);
}

// в Rust все определяеться в snake_case
fn another_function(){
    println!("Hello from another function");
}

fn sun(x:i32,y:i32){
    println!("The sum of {} and {} is {}",x,y,x+y);
}
