fn main(){
    //тут уже не можем использовать s1
    //это касаеться всехтипов данных которые владеют ресурсами
    let s1 = String::from("hello");
    let s2 = s1;
    //переменная s1 перестает существовать
    println!("{}",s2);
    another_function(s2);
    //переменная s2 перестает существовать
    


    //тут все ок тк не происходит владение
    let n1 = 12;
    let n2 = n1;
    println!("{}{}", n1,n2);
    another_function2(n2);
    println!("{}{}",n1,n2);



    let ss1 = String::from("hello");
    let ss2 = ss1.clone();
    println!("{}{}",ss1,ss2);

    another_function3(ss1);
    //ss1 перестает существовать тк мы отдали его на владение функции
    
    println!("{}",ss2);
}


fn another_function(s1: String){
    println!("{}",s1);
}

fn another_function2(n1: i32){
    println!("{}",n1);
}

fn another_function3(ss1: String){
    println!("{}",ss1);
}
