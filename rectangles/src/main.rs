#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool{
        self.width > rect.width && self.height > rect.height
    }
}

fn main() {
    let rect = Rectangle{
        width: 30,
        height: 50
    };

    let rect2 = Rectangle{
        width: 50,
        height: 20
    };

    let rect3 = Rectangle{
        width: 10,
        height: 40
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    println!("{:#?}", rect);

    println!("{}", rect.can_hold(&rect2));
    println!("{}", rect.can_hold(&rect3));
}