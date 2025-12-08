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
}
