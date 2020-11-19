use hangul::HangulExt;
use unic_ucd_hangul::compose_syllable;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let hangul = String::from(&args[1]);
    let hangul = &hangul[..];

    let mut hashmap: HashMap<&str, Vec<&str> > = HashMap::new();

    fill_hashmap(hangul, &mut hashmap);

    //let vector: Vec<char> = hangul.chars().flat_map(|c| c.jamos().unwrap()).collect();

    // println!("You entered: {}", hangul);
    // print!("{} is comprised of: ", hangul);
}

fn fill_hashmap(hangul: &str, hashmap: &mut HashMap<&str, Vec<&str> > ) {
    let vec = split(hangul);
    let vec = decomp(vec);
    
    //create hashmap

}

fn split(hangul: &str) -> Vec<&str> { 
    let vec: Vec<&str> = hangul.split(' ').collect();
    return vec;
}

fn decomp(original: Vec<&str>) -> Vec<char> { 
    let mut vec: Vec<char> = Vec::new();
    
    for elem in original.into_iter() {
        vec = elem.chars().flat_map(|c| c.jamos().unwrap()).collect();
    }
    return vec;
}

//TODO: start implementing the rules. not sure where to start, 받침 or the otherweird ones, look into er
fn pronounce(hangul: &String) {  // maybe use &str slice type for generic? 
    
}