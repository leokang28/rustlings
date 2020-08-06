// variables2.rs
// Make me compile! Execute the command `rustlings hint variables2` if you want a hint :)

// I AM NOT DONE

fn main() {
    let x = "abc";
    let x:u32 = match x.parse() {
        Ok(num) => num,
        Err(_) => 1,
    };
    println!("{}", x);
    if x == 10 {
        println!("Ten!");
    } else {
        println!("Not ten!");
    }
}
