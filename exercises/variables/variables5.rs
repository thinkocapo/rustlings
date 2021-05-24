// variables5.rs
// Make me compile! Execute the command `rustlings hint variables5` if you want a hint :)

// I AM NOT DONE

fn main() {
    // let spaces = "   ";
    // let spaces = spaces.len();
    // spaces = 234; <-- doesn't work unless add 'mut'
    // println!("Spell a spaces : {}", spaces);

    let mut number = "T-H-R-E-E";
    let mut number = 3;
    println!("Spell a Number : {}", number);
    number = 3; // <-- need 'mut' because reassigning here
    println!("Number plus two is : {}", number + 2);
}
