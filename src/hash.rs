use hangul::HangulExt;
use unic_ucd_hangul::compose_syllable;
use std::collections::HashMap;
//holds info about normal hangul string, hangul string decomposed into jamo, and how its pronounced respectively
pub struct HangulInfo {
    pub hangul: String,
    pub decomposed: Vec<char>,
    pub pronounced_as: String,
}

//every word "한국어를_배우고_싶어요" ( _ where there is new word), should be a seperate entry into the hashmap
pub fn fill_hashmap(hangul: & str, hashmap: &mut HashMap<String, HangulInfo>) {                      
    let vec = vec![hangul];
    let vec = decomp(vec);
    let pronounced_as = pronounce(&vec);

    let hangul = String::from(hangul);

    let value = HangulInfo {
        hangul: hangul.clone(),
        decomposed: vec,
        pronounced_as: pronounced_as,
    };

    //create hashmap, used the struct as the value (k,v)
    hashmap.insert(String::from(hangul), value);
}

fn decomp(original: Vec<&str>) -> Vec<char> { 
    let mut vec: Vec<char> = Vec::new();

    for elem in original.into_iter() {
        vec = elem.chars().flat_map(|c| c.jamos().unwrap()).collect();
    }

    return vec;
}

//TODO: start implementing the rules. not sure where to start, 받침 or the otherweird ones, look into er
//REMEBER: make sure u dont transfer ownership of decomps values, because u need them to imake the hangulinfo struct in fill_hashmap
fn pronounce(decomp: &Vec<char>) -> String {
    return String::from("fuck");
}