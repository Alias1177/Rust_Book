struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


fn main(){

 let u = &User {
     active: true,
     username: String::from("username"),
     email: String::from("email"),
     sign_in_count: 1,
     
 };
    println!("{}{}{}{}", u.active, u.username, u.email, u.sign_in_count);
}
