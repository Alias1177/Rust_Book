
#[derive(Debug)]
struct Rectangle {
    width:u32,
    height:u32
}

fn main(){
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2)); // Должно быть true
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3)); // Должно быть false

    println!("rect1 is {:?}",rect1);

    let ar =rect1.area();
    println!("area is {}",ar);

    let per =rect1.perimetr();
    println!("perimetr is {}",per);
    println!("Alive again? {:?}", rect1);
    
    let sq =Rectangle::square(3);
    println!("square is {:?}",sq);

}

impl Rectangle{
    fn area(&self)->u32{
        self.width*self.height
    }
    fn perimetr(&self)->u32{
        2*self.width+2*self.height
    }
    fn can_hold(&self,other: &Rectangle)->bool{
        self.width>other.width && self.height>other.height
    }

    fn square(num:u32)->Self{
        Rectangle{width:num,height:num}
    }
}

