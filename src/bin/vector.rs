#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main(){

    let mut vector= Vec::new();
    //let vec = vec![1,2,3];
    
    
    vector.push(5);
    vector.push(6);
    vector.push(7);

    println!("{:?}", vector);
    println!("{}", vector[0]);

    let third: &i32 = &vector[2];
    println!("The third element is {}", third);

    let second: Option<&i32>= vector.get(1);
        match second {
            Some(second) => println!("The second element is {}", second),
            None => println!("There is no second element."),
        }

    for i in &mut vector {
        println!("add +50 = {i}");
        *i += 50;
    }
    for i in &vector {
        println!("{}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in &row {
        println!("{:?}", i);
    }
}
