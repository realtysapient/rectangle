struct Rectangle {
    height: u32,
    width: u32,

}

fn main() {
    let rect = Rectangle {
        height:50,
        width:30,
    };
    println!("The area of the rectangle is {}",area(&rect));
}
//mycoment

fn area(rect:&Rectangle) -> u32 {
    rect.height * rect.width
}
