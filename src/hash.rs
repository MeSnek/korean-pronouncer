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
//will not be doing 받침 theres ez af just learn em lol
//also not able to really do anything relateed to if something is grammatical or not
/*

    ㄱㄲㅋ ㅈㅊ     ㄷㅌ ㅂㅍ   ㅅㅆ    ㅁㄴㅇㄹㅎ      possible as last letter
    3 1 1  2 2     2 2  3 1    2 2    1 1 1 0 4      possible rules

    nasalization
        ㄱㄲㅋ ->               ㅇ if before ㄴ/ㅁ 격노 -> 경노 / 국물 -> 궁물
        ㄷㅌㅅㅆㅈㅊㅎ ->        ㄴ if before ㄴ/ㅁ 
        ㅂㅃㅍ ->               ㅁ if before ㄴ/ㅁ
    aspiration
        ㅎ ->                   ㅌ if before ㄷ
        ㅎ ->                   ㅊ if before ㅈ
        ㅎ ->                   ㅆ if before ㅅ
        ㅎ ->                   ㅋ if before ㄱ

        ㄱ ->                   ㅋ if before ㅎ 
        ㅂ ->                   ㅍ if before ㅎ
        ㄷㅌㅅㅆㅈㅊㅎ ->        ㅌ if before ㅎ
    assim / alterations with ㄹ
        ㅁㄹ -> ㅁㄴ
        ㅇㄹ -> ㅇㄴ
        ㄱㄹ -> ㅇㄴ
        ㅂㄹ -> ㅁㄴ
        ㄴㄹ -> ㄹㄹ EXCEPTION {의견란[의ː견난]  임진란[임ː진난]  생산량[생산냥]  결단력[결딴녁] 공권력[공꿘녁]  동원령[동ː원녕]  상견례[상견녜]  횡단로[횡단노] 이원론[이ː원논]  입원료[이붠뇨]  구근류[구근뉴]}

*/
fn pronounce(decomp: &Vec<char>) -> String {
    return String::from("fuck");
}