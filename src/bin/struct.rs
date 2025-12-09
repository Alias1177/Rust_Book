struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Book{
    title: String,
    pages: u32
}
//кортежная структура
struct Color(i32,i32,i32);

struct AlwaysEqual;

fn main(){
    let subject = AlwaysEqual;
    

 let black = Color(0,0,0);
    println!("{}",black.0);
    
 let u = &User {
     active: true,
     username: String::from("username"),
     email: String::from("email"),
     sign_in_count: 1,
     
 };

    println!("{}{}{}{}", u.active, u.username, u.email, u.sign_in_count);
    let title = String::from("The Hobbit");
    let pages = 295;

    let mut my_book = Book {
        title,
        pages,
    };

    println!("{}{}", my_book.title, my_book.pages);
    my_book.pages = 50;

    let my_book2 = Book {
        pages: 50,
        ..my_book
    };
    
    println!("{}{}", my_book2.title, my_book2.pages);

}
