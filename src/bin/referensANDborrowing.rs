fn main(){

    let s =String::from("hello");

    let len = caclulate_len(&s);

    println!("The length of '{}' is {}.", s, len);

    let mut ss =String::from("hello");

    change(&mut ss);    

    let ss1 =&ss;
    let ss2 = &ss;
    //let ss3 = &mut s; ошибка тк у нас уже есть не мутирующие ссылки которые еще не гтчистились
    println!("{}",ss1);
    println!("{}",ss2);
    
    //тут не изменяемые ссылки отчищаються и мы можем создавать мутирующие
    let ss3 =&mut ss;
    println!("{}",ss3);
}

fn caclulate_len(s:&str) -> usize{
    s.len()
}

fn change(s: &mut String){
    s.push_str(", world");
    println!("{}",s);
}
