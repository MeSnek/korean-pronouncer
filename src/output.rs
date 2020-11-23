use std::collections::HashMap;
use super::hash::HangulInfo;

//-h    help 
//-i    outputs like how the for loop is now, serperate each word
//-s    outputs a whole sentence without serperating each word\
pub fn output(p: String, hashmap: &mut HashMap<String, HangulInfo>, order: Vec<String>) {
    match p.as_str() {
        "-h" | "-H" => help(),
        "-i" | "-I" => individual(hashmap, order),
        "-s" | "-S" => sentence(hashmap, order),
        _=>            individual(hashmap, order),
    }
}

pub fn help() {
    print!(
"krpr is a program that tells you how to pronounce korean words/sentences.
I wrote this for personal use so dont expect it to be perfect.
The default mode for krpr is -i, which outputs each word seperately. 

You can specify what mode you want to use by adding a parameter in this format, 
    krpr [-param] (input) 
Input can be a single word or a list of words seperated by spaces.

List of parameters:
    -h  Ignores all input and displays help.
    -i  Displays each word that is seperated by a space as its own word rather than a full sentence.
    -s  Displays all the input as one sentence.")
}
fn individual(hashmap: &mut HashMap<String, HangulInfo>, order: Vec<String>) {
    for elem in order.into_iter() {
        let key = hashmap.get(&elem).unwrap();
        println!("\"{}\" is pronounced as \"{}\"", key.hangul, key.pronounced_as)
    }
}
fn sentence(hashmap: &mut HashMap<String, HangulInfo>, order: Vec<String>) {
    let order2 = order.clone();
    for elem in order.into_iter() {
        let key = hashmap.get(&elem).unwrap();
        print!("{} ", key.hangul);
    }
    println!();
    println!("is pronounced,");
    for elem in order2.into_iter() {
        let key = hashmap.get(&elem).unwrap();
        print!("{} ", key.pronounced_as);
    }
}