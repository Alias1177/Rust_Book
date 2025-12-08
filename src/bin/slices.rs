fn main(){
    let a = [1,2,3,4,5];

    //от 1 включая до 3 не включая
    let slice = &a[1..3];
    println!("{}", slice.len());
    
    //идем по индексам
    for i in 0..slice.len(){
        println!("{}", i);
    }

    //идем по значениям
    for i in slice{
        println!("{}", i);
    }

    //идем по индексам и значениям
    for (idx,value) in slice.iter().enumerate(){
        println!("{} {}", idx, value);
    }

    let b = [1,2,3,4,5];
    let qwerty = "hello";

    for (idx, value) in b.iter().enumerate(){
        let r= value*3;
        println!("{}", r);
        println!("qqqqqq{} {}", idx, value);
    }

    for (idx, value) in qwerty.chars().enumerate(){
        if value == 'l' {
            continue
        }
        println!("{} {}", idx, value);
    }
}
