// do_work :: Dict -> String -> String
// do_work d s = concat $ map (f d) (words s)
//     where f = (\(d, w) -> concat $ map (convert_syllable d) (syllables w)

// syllables :: Dict -> String -> [String]
// syllables _ [] = []
// syllables d s@(x:xs)
//     | fst cond = (fst $ snd $ cond):(syllables d (snd $ snd $ cond))
//     | otherwise = [x]:(syllables d xs)
//     where
//          max = let len = length s in if (>) len 3 then 3 else len
//       -- aux :: Int -> Dict -> String -> (Bool, (String, String))
//          aux 0 _ _ = (False, ([], []))
//          aux _ _ [] = (False, ([], []))
//          aux n d s
//              | contains_key d (take n s) = (True, (take n s, drop n s))
//              | otherwise = aux (n-1) d s
//          cond = aux max d s

// to_hiragana :: Dict -> String -> String
// to_hiragana d h = do_work d (to_lowercase h)

// to_katakana :: Dict -> String -> String
// to_katakana d k = do_work d (to_uppercase k)

// to_kana :: Dict -> String -> String
// to_kana = do_work

use std::cmp;

pub mod dict;

use dict::Dict;

pub fn to_hiragana(d: &Dict, s: &String) -> String {
    let s = s.to_lowercase();
    gen_convert(d, &s)
}

pub fn to_katakana(d: &Dict, s: &String) -> String {
    let s = s.to_uppercase();
    gen_convert(d, &s)
}

pub fn to_kana(d: &Dict, s: &String) -> String {
    gen_convert(d, &s)
}

// Converts a String as is
fn gen_convert(d: &Dict, s: &String) -> String {
    let mut ret: String = String::new();

    for w in s.split_whitespace() {
        let w: String = normalize(&w.to_string());
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

// Given a String (expected to be normalized), returns `true` if it is a syllable and
// `false` otherwise.
fn is_syllable(d: &Dict, k: &String) -> bool {
    d.contains_key(k)
}

// Searches for uppercase characters in a String and, if it finds any, turns the whole String to
// uppercase.
fn normalize(s: &String) -> String {
    match s.chars().fold(false, |acc, c| acc || c.is_uppercase()) {
        true => s.to_uppercase(),
        false => s.to_string(),
    }
}

// Converts a String (expected to be a syllable) to kana.
pub fn convert_syllable(d: &Dict, ow: &String) -> String {
    // <pingveno> Because String impls Borrow<str>
    // `&ow.to_string()` => `ow`
    match d.get(&normalize(&ow)) {
        Some(c) => c.clone(),
        None => ow.clone(),
    }
}
