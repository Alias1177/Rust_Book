fn main(){
   //тут ошибка тк компилятор должен знать какого типа будет переменная во время компиляции 
   //let condition = true;

   // let number = if condition { 5 } else { "six" };

    //println!("The value of number is: {number}");

    let c =12;

    if c>5 {
        println!("c>5");
    }else if c<12{
        println!("c<12");
    }else{
        println!("c delault")
    }
}
