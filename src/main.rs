use hangul::HangulExt;

fn pronounced_as(hangul: &String, ) { 
 
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let hangul: &String = &args[1];
    let hangul: String = hangul.to_string();
    
    let vector: Vec<char> = hangul.chars().flat_map(|c| c.jamos().unwrap()).collect();
    let mut i: i32 = 1;

    println!("You entered: {}", hangul);
    print!("{} is comprised of: ", hangul);
    
    for elem in vector.iter() {
        print!("{}, ", elem);
    }
}

