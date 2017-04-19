use std::cmp;

extern crate kana;
use kana::{combine, half2kana, nowidespace, nowideyen, wide2ascii};

pub mod dict;
use dict::{Dict, KanaConversionTable};

#[derive(Copy, Clone, Debug)]
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

        let w: String = syllables(&d, &w)
            .iter()
            .map(|ref s| convert_syllable(d, &s))
            .collect();

        ret.push_str(w.as_str());
    }

    ret
}

fn syllables(d: &Dict, original: &String) -> Vec<String> {
    if original.len() == 0 {
        return vec![];
    }

    let mut n: usize = 1;
    let mut ret: Vec<String> = vec![];
    let maxlen = cmp::min(original.chars().count(), d.max_len()) + 1;

    // find a syllable
    for l in (1..maxlen).rev() { // [1, 2, .., maxlen-1].rev()
        let tmp: String = original.chars().take(l).collect();

        if is_syllable(d, &tmp) {
            n = l;
            break;
        }
    }

    let taken: String = original.chars().take(n).collect();
    ret.push(taken);

    let skipped: String = original.chars().skip(n).collect();
    let mut rest: Vec<String> = syllables(d, &skipped);
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
    let mut ret: String = s.clone();

    ret = nowidespace(ret.as_str());
    ret = nowideyen(ret.as_str());
    ret = wide2ascii(ret.as_str());

    ret = combine(ret.as_str());
    ret = half2kana(ret.as_str());

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
