
#[derive(Debug)]
enum State{
    California,
    Texas,
    Washington
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(State)
}

fn value_coin(coin: Coin) -> u32{
    match coin{
        Coin::Penny =>1,
        Coin::Nickel =>5,
        Coin::Dime =>10,
        Coin::Quarter(state) => {
            println!("State quarter from {:#?}!", state);
            25
        }
    }
}

fn pluse_one(x :Option<i32>)->i32{
    match x{
        None => 0,
        Some(i) => i + 1
    }
}

fn main() {
    let state = State::Texas;
    let coin = Coin::Quarter(state);
    value_coin(coin);
    
    let coin2 =State::California;
    let coin3 = Coin::Quarter(coin2);
    value_coin(coin3);
    
    let coin4 = Coin::Penny;
    let coin5 = Coin::Nickel;
    let coin6 = Coin::Dime;
    let state2 = State::Washington;
    let five = Some(5);
    let six = pluse_one(five);
    let none = pluse_one(None);
    println!("{}", six);
    println!("{}", none);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => println!("other")
    }

    let scroll_number = 9;
    match scroll_number {
        3 => println!("three"),
        5 => println!("five"),
        9 => println!("nine"),
        _ => println!("other")
    }
}

