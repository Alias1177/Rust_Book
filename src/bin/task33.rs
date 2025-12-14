use std::collections::HashMap;

fn divide(a: i32, b: i32) -> Result<i32, String>{
    b.checked_div(a).ok_or("Деление на ноль".to_string())?;
    Ok(a / b)
}

fn parse_number(s: &str) -> Result<i32, String>{
        let v = s.parse::<i32>().map_err(|e| e.to_string())?;
        Ok(v)
}

fn sqrt_positive(x: i32) -> Result<f64, String>{
    let res = (!x.is_negative()).then_some(x.pow(2)).ok_or("Отрицательное число".to_string())?;
    Ok(res as f64)
}

fn first_char(s: &str) -> Result<char, String>{
    let res = s.chars().next().ok_or("Пустая строка".to_string())?;
    Ok(res)
}

fn get_element(v: &Vec<i32>, index: usize) -> Result<i32, String> {
   let result = *v.get(index).ok_or("выходит за границы вектора".to_string())?;
   Ok(result)
}

fn get_port(cfg: &HashMap<String, String>) -> Result<u16, String> {
    let port =cfg.get("port")
        .ok_or("Порт не найден")?
        .parse::<u16>()
        .map_err(|e| e.to_string())?;
    Ok(port)
}

fn main() {
    match divide(10, 2) {
        Ok(v) => println!("Результат: {}", v),
        Err(e) => println!("Ошибка: {}", e),
    }

    match divide(10, 2) {
        Ok(v) => println!("Результат: {}", v),
        Err(e) => println!("Ошибка: {}", e),
    }

    match parse_number("10") {
        Ok(v) => println!("Результат: {}", v),
        Err(e) => println!("Ошибка: {}", e),
    }

    match sqrt_positive(10) {
        Ok(v) => println!("Результат: {}", v),
        Err(e) => println!("Ошибка: {}", e),
    }

    match first_char("Hello") {
        Ok(v) => println!("Результат: {}", v),
        Err(e) => println!("Ошибка: {}", e),
    }

    match get_element(&vec![1, 2, 3], 7) {
        Ok(v) => println!("Результат: {}", v),
        Err(e) => println!("Ошибка: {}", e),
    }

    match get_port(&HashMap::from([("port".to_string(), "80".to_string())])) {
        Ok(v) => println!("Результат: {}", v),
        Err(e) => println!("Ошибка: {}", e),
    }
}
