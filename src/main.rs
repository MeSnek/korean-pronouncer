use hangul::HangulExt;
use unic_ucd_hangul::compose_syllable;
use std::collections::HashMap;

fn main() {
    let mut hashmap: HashMap<&str, Vec<&str> > = HashMap::new();
    let args: Vec<String> = std::env::args().skip(1).collect();
    
    for elem in args.into_iter() {
        let hangul = String::from(elem);
        let hangul = &hangul[..];
        fill_hashmap(hangul, &mut hashmap);   
    }
}
//every word "한국어를_배우고_싶어요" (_ where there is new word), should be a seperate entry into the hashmap
fn fill_hashmap(hangul: &str, hashmap: &mut HashMap<&str, Vec<&str>>) {
    //let vec = split(hangul);        //not needed so find way to get rid of without using .split lmao waste of time
    let vec = vec![hangul];
    let vec = decomp(vec);
    let pronounced = pronounce(vec);

    let value = HangulInfo {
        hangul: String::from(hangul),
        decomposed: vec,
        pronounced_as: pronounced,
    };
    //create hashmap, used the struct as the value (k,v)
    //hashmap.insert(hangul, value)
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
fn pronounce(decomp: Vec<char>) -> String {


    return pronounced;
}
//holds info about normal hangul string, hangul string decomposed into jamo, and how its pronounced respectively
struct HangulInfo {
    hangul: String,
    decomposed: Vec<char>,
    pronounced_as: String,
}