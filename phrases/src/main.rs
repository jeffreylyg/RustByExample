extern crate phrases;

use phrases::english::greetings;

fn main() {
    println!("Hello in English: {}", phrases::english::greetings::hello());
    println!("Hi in English: {}", phrases::english::greetings::hi());
    println!("Goodbye in English: {}", phrases::english::farewells::goodbye());

    println!("Hello in Japanese: {}", phrases::japanese::greetings::hello());
    println!("Goodbye in Japanese: {}", phrases::japanese::farewells::goodbye());

    println!("DDD: {}", greetings::hello());
}