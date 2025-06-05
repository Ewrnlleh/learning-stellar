// mkdir soroban
// cd soroban
// cargo new code_in_rust


// go to terminal -> cargo build -> it will add target, cargo.lock, and .gitingnore



fn main() {
    // variables
    // let -> immutable
    // let mut -> mutable

    // basic data types -> Rust is a type language

    let x: i32 = 16;

    // to print the output out, we need to write a macro => println! macro

    println!("{}", x); // -> this is the first version

    let z: String = String::from("Hello, Soroban!"); //mutable string
    let y: &str = "Hello, Stellar!"; //immutable string

    println!("{y}");
    println!("{z}"); // tihs is the second version

    // functions -> fn - > pub fn - > public fn - > private

    // pub fn event (name: String) {
        // let name: String = String::from ("James");
        // println!("{}", name);
    //}

    let e: EventForKids = EventForKids {
        name: String = String::from("KidsCo"),
        date: String = String::from("04.03.2024"),
        number_of_participants: u32 = 1000,
        place: String = String::from("NY, USA")
    };

    // add your enum in here...



}


// structs -> compiling many items in one class

struct EventForKids {
    name: String,
    date: String,
    number_of_participants: u32,
    place: String
}

//enums -> compiling errors in once class
enum ErrorsForEvent{
    NoEvent,
    CancelledEvent,
    EventType
}