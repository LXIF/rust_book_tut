#[derive(Debug, Copy, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }

    fn set_to_max(&mut self, other: Rectangle) {
        *self = self.max(other);
    }
}

impl Rectangle {
    //can totally have multiple impls, cool like that.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let scale = 2;
    let mut rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 20,
    };
    let rect3 = Rectangle {
        width: 150,
        height: 500,
    };

    println!("rect1 is {:#?}", rect1);
    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        //area(&rect1)
        rect1.area()
    );

    if rect1.width() {
        println!("Rectangle haz nonzero width; it is {}", rect1.width)
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(rect3));

    let my_square = Rectangle::square(7);

    println!("my_square is {:#?}", my_square);

    rect1.set_width(2);
    println!("rect1 is {:#?}", rect1);
    rect1.set_to_max(rect3);
    println!("rect1 is {:#?}", rect1);

    let rect_ref = &mut rect1;
    println!(
        "The area of the rectangle is {} square pixels.",
        rect_ref.area()
    );
}

// fn area(dimensions: &Rectangle) -> u32 {
//     // let (width, height) = dimensions;
//     // width * height
//     // dimensions.0 * dimensions.1
//     dimensions.width * dimensions.height
// }
