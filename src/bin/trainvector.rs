fn main() {
    
    let vector = vec![3, 7, 7, 2, 9, 3, 7, 1, 9];
    let mut unique = Vec::new();

    for &i in &vector{
        if !unique.contains(&i) {
            unique.push(i);
        }
    }

    println!("{:?}", unique);

    let (vec1, vec2)= split_even_odd(&vector);

    println!("{:?}", vec1);
    println!("{:?}", vec2);
}

fn split_even_odd(events: &[i32]) -> (Vec<i32>, Vec<i32>){
    let mut even = Vec::new();
    let mut odd = Vec::new();

    for &event in events {
        if event%2 == 0 {
            even.push(event);
        }else{
            odd.push(event);
        }
    }

    (even, odd)
}
