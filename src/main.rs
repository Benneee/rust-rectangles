fn main() {
    // In this block, I'm going to write a program to calculate the area of a rectangle in different ways and eventually ending up with Structs
    // Method 1: procdural
    let width = 3;
    let height = 4;

    println!(
        "The area of the rectangle is {} meter square",
        rectangle_area(width, height)
    );

    // Method 2: using tuples
    // This way, we don't have to repeat the width and height, and we can just pass a simple expression to get the area
    let rect1 = (3, 4);
    // This works great when we do not need to care about the dimensions (width, height). However, if this rectangle was to be plotted
    // Using a tuple here would be bug-prone and open to misinterpretation
    println!(
        "The area of the rectangle is {} meter square (using tuple)",
        area_by_tuple(rect1)
    );

    // Using the struct created below
    let rect2 = Rectangle {
        width: 3,
        height: 4,
    };

    // To calculate the area, we use the method declared below
    println!(
        "The area of the rectangle is {} square meter (using struct)",
        rect2.area_by_struct()
    )
}

fn rectangle_area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_by_tuple(dimensions: (u32, u32)) -> u32 {
    // We access the properties of the tuple by index for the product
    // While this works, it's prone to bugs and misinterpretation
    dimensions.0 * dimensions.1
}

// Introducing the struct
struct Rectangle {
    width: u32,
    height: u32,
}
// Structs have methods associated with them, sort of like a function we can use to calculate the area of the rectangle
// We use the impl keyword to create the function
impl Rectangle {
    // The first parameter of a method is self and it represents the instance of the struct the method is being called on
    // self gives us access to the properties of the struct
    fn area_by_struct(&self) -> u32 {
        self.width * self.height
    }
}
