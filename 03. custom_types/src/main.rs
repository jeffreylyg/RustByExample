fn main() {
    structures();  //3.1
    println!("");
    enums();       //3.2
    println!("");
    enums_use();   //3.2.1 
    println!("");
    enum_c_like(); //3.2.2
    println!("");  
    constants();   //3.3
}

fn structures() {
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8
    }

    // A unit struct
    struct Nil;

    // A tuple struct
    struct Pair(i32, f32);

    // A struct with two fields
    struct Point {
        x: f32,
        y: f32,
    }

    // Structs can be reused as fields of another struct
    #[allow(dead_code)]
    struct Rectangle {
        p1: Point,
        p2: Point,
    }

    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Destructure the point using a `let` binding
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    // Instantiate a unit struct
    let _nil = Nil;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}

fn enums() {
    // Create an `enum` to classify someone. Note how both names
    // and type information together specify the variant:
    // `Engineer != Scientist` and `Height(i32) != Weight(i32)`. Each
    // is different and independent.
    enum Person {
        // An `enum` may either be `unit-like`,
        Engineer,
        Scientist,
        // like tuple structs,
        Height(i32),
        Weight(i32),
        // or like structures.
        Info { name: String, height: i32 }
    }

    // A function which takes a `Person` enum as an argument and
    // returns nothing.
    fn inspect(p: Person) {
        // Usage of an `enum` must cover all cases (irrefutable)
        // so a `match` is used to branch over it.
        match p {
            Person::Engineer  => println!("Is an engineer!"),
            Person::Scientist => println!("Is a scientist!"),
            // Destructure `i` from inside the `enum`.
            Person::Height(i) => println!("Has a height of {}.", i),
            Person::Weight(i) => println!("Has a weight of {}.", i),
            // Destructure `Info` into `name` and `height`.
            Person::Info { name, height } => {
                println!("{} is {} tall!", name, height);
            },
        }
    }

    let person   = Person::Height(18);
    let amira    = Person::Weight(10);
    // `to_owned()` creates an owned `String` from a string slice.
    let dave     = Person::Info { name: "Dave".to_owned(), height: 72 };
    let rebecca  = Person::Scientist;
    let rohan    = Person::Engineer;

    inspect(person);
    inspect(amira);
    inspect(dave);
    inspect(rebecca);
    inspect(rohan);
}

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn enums_use() {
    use Status::{Poor, Rich};
    use Work::*;
    // Equivalent to `Status::Poor`.
    let status = Poor;
    // Equivalent to `Work::Civilian`.
    let work = Civilian;

    match status {
        // Note the lack of scoping because of the explicit `use` above.
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        // Note again the lack of scoping.
        Civilian => println!("Civilians work!"),
        Soldier  => println!("Soldiers fight!"),
    }
}

fn enum_c_like() {
    // An attribute to hide warnings for unused code.
    #![allow(dead_code)]

    // enum with implicit discriminator (starts at 0)
    enum Number {
        Zero,
        One,
        Two,
    }

    // enum with explicit discriminator
    enum Color {
        Red = 0xff0000,
        Green = 0x00ff00,
        Blue = 0x0000ff,
    }

    // `enums` can be cast as integers.
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}

/**
 * Rust has two different types of constants which can be declared in any scope including global. 
 * Both require explicit type annotation:
 *
 * const: An unchangeable value (the common case).
 * static: A possibly mutable variable with 'static lifetime.
 *
 * One special case is the "string" literal. 
 * It can be assigned directly to a static variable without modification
 * because its type signature: &'static str has the required lifetime of 'static.
 * All other reference types must be specifically annotated so that they fulfill the 'static lifetime. 
 * This may seem minor though because the required explicit annotation hides the distinction.
 */
fn constants() {
    static LANGUAGE: &'static str = "Rust";
    const  THRESHOLD: i32 = 10;

    fn is_big(n: i32) -> bool {
        // Access constant in some function
        n > THRESHOLD
    }

    let n = 16;

    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });
}