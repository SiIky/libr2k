use std::cmp;

mod tests;
pub mod dict;

use dict::{Dict, KanaConvertionTable};

pub fn to_hiragana(s: &String, d: &dict::Dict) -> String {
    unimplemented!();
}

pub fn to_katagana(s: &String, d: &dict::Dict) -> String {
    unimplemented!();
}

pub fn to_kana(s: &String, d: &dict::Dict) -> String {
    let vec: Vec<String> = syllables(d, s);
    let mut ret: String = String::new();
    for syl in vec {
        ret.push_str(&normalize(&syl));
    }
    ret
}

// converts everything as is according to the dict
pub fn gen_convert(s: &String, d: &dict::Dict) -> String {
    let vec: Vec<String> = syllables(d, s);
    let mut ret: String = String::new();
    for syl in vec {
        ret.push_str(&convert_syllable(d, &syl));
    }
    ret
}

// do_work :: Dict -> String -> String
// do_work = concat $ map convert_syllable $ syllables
//
// syllables :: Dict -> String -> [String]
// syllables _ [] = []
// syllables d s@(x:xs)
//     | fst cond = (fst $ snd $ cond):(syllables d (snd $ snd $ cond))
//     | otherwise = [x]:(syllables d xs)
//     where
//          max = let len = length s in if (>) len 3 then 3 else len
//       -- aux :: Int -> Dict -> String -> (Bool, (String, String))
//          aux 0 _ _ = (false, ([], []))
//          aux _ _ [] = (false, ([], []))
//          aux n d s
//              | contains_key d (take n s) = (true, (take n s, drop n s)
//              | otherwise = syllables (n-1) d s
//          cond = aux max d s

fn syllables(d: &dict::Dict, original: &String) -> Vec<String> {
    if original.len() == 0 {
        return vec![];
    }

    let mut ret: Vec<String> = vec![];
    let maxlen = cmp::min(original.chars().count() + 1, 4);

    for l in (1..maxlen).rev() {
        let tmp: String = original.chars().take(l).collect();

        if is_syllable(d, &tmp) {
            ret.push(tmp);
            let skipped: String = original.chars().skip(l).collect();
            let mut rest = syllables(d, &skipped);

            ret.append(&mut rest);

            return ret;
        }
    }

    // if a match wasn't found, add the first char to ret
    // let mut taken: Vec<String> = vec![original.chars().take(1).collect()];
    // ret.append(&mut taken);

    let taken: String = original.chars().take(1).collect();
    ret.push(taken);

    let skipped: String = original.chars().skip(1).collect();
    let mut rest = syllables(d, &skipped);
    ret.append(&mut rest);

    ret
}

fn is_syllable(d: &dict::Dict, k: &String) -> bool {
    let u = k.clone();
    let l = k.clone();
    u.to_uppercase();
    l.to_lowercase();
    d.contains_key(&u) || d.contains_key(&l)
}

fn normalize(s: &String) -> String {
    match s.chars().fold(false, |acc, c| acc || c.is_uppercase()) {
        true => s.to_uppercase(),
        false => s.clone(),
    }
}

fn convert_syllable(d: &dict::Dict, ow: &String) -> String {
    match d.get(&ow.clone()) {
        Some(c) => c.clone(),
        None => ow.clone(),
    }
}

pub fn do_work(d: &dict::Dict, w: &String) -> String {
    let mut ret = String::new();
    let sylvec: Vec<String> = syllables(d, w)
        .iter()
        .map(|ref s| convert_syllable(d, &s))
        .collect();

    for s in sylvec {
        ret.push_str(s.as_str());
    }

    ret
}

pub fn get_dict() -> dict::Dict {
    let mut ret = Dict::new();
    ret.init();
    ret
}
