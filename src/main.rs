#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,

}

fn main() {
    let scale =2;
    let rect = Rectangle {
        height: dbg!(50*scale),
        width:30,
    };
    //println!("The area of the rectangle is {:?}",area(&rect));
    //println!("rect1 is {:#?}", rect);
    dbg!(&rect);
}
//mycoment

fn area(rect:&Rectangle) -> u32 {
    rect.height * rect.width
}
