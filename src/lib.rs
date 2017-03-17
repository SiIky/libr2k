use std::cmp;

extern crate kana;
use kana::Kana;

pub mod dict;
use dict::{Dict, KanaConversionTable};

pub enum ConvType<T> {
    Auto(T),
    Hira(T),
    Kata(T),
}

pub fn to_kana(d: &Dict, s: ConvType<String>) -> String {
    let s = match s {
        ConvType::Auto(s) => s,
        ConvType::Hira(s) => s.to_lowercase(),
        ConvType::Kata(s) => s.to_uppercase(),
    };
    convert(d, &s)
}

// Converts a String as is
fn convert(d: &Dict, s: &String) -> String {
    let mut ret: String = String::new();

    for w in normalize(s).split_whitespace() {
        let w: String = choose_kana(&w.to_string());
        let ss: Vec<String> = syllables(&d, &w)
            .iter()
            .map(|ref s| convert_syllable(d, &s))
            .collect();
        for syl in ss {
            ret.push_str(syl.as_str());
        }
    }
    ret
}

fn syllables(d: &Dict, original: &String) -> Vec<String> {
    if original.len() == 0 {
        return vec![];
    }

    let mut ret: Vec<String> = vec![];
    let maxlen = cmp::min(original.chars().count(), d.max_len()) + 1;

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

// Given a String (expected to be normalized), returns `true` if it is a syllable and
// `false` otherwise.
fn is_syllable(d: &Dict, k: &String) -> bool {
    d.contains_key(k)
}

// Searches for uppercase characters in a String and, if it finds any, turns the whole String to
// uppercase.
fn choose_kana(s: &String) -> String {
    match s.chars().fold(false, |acc, c| acc || c.is_uppercase()) {
        true => s.to_uppercase(),
        false => s.to_string(),
    }
}

fn normalize(s: &String) -> String {
    let k = Kana::init();
    let mut ret: String = s.clone();

    ret = kana::nowidespace(ret.as_str());
    ret = kana::nowideyen(ret.as_str());
    ret = kana::wide2ascii(ret.as_str());

    ret = k.combine(ret.as_str());
    ret = k.half2kana(ret.as_str());

    ret
}

// Converts a String (expected to be a syllable) to kana.
fn convert_syllable(d: &Dict, ow: &String) -> String {
    // <pingveno> Because String impls Borrow<str>
    // `&ow.to_string()` => `ow`
    match d.get(ow) {
        Some(c) => c.clone(),
        None => ow.clone(),
    }
}
