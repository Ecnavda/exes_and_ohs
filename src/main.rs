use std::io;

fn main() {
    let mut a = String::new();

    println!("Enter a string.");

    io::stdin().read_line(&mut a)
        .expect("Couldn't read your input.");
    
    //Remove trailing newline character
    let a = a.trim();  
    
    if xo(&a) {
        println!("true");
    } else {
        println!("false");
    }
}

fn xo(string: &str) -> bool {
    let mut Xs = 0;
    let mut Os = 0;
    
    //For loops implicitly call to_iter() method
    for letter in string.chars() {
        match letter {
            'x' | 'X' => Xs = Xs + 1,
            'o' | 'O' => Os = Os + 1,
            _ => {},
        }
    }

    if Xs == Os {
        return true;
    } else {
        return false;
    }
}
