
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

    let five = Some(5);
    let six = pluse_one(five);
    let none = pluse_one(None);
    println!("{}", six);
    println!("{}", none);

    let scroll_number = 9;
    match scroll_number {
        3 => println!("three"),
        5 => println!("five"),
        9 => println!("nine"),
        _ => println!("other")
    }
}

