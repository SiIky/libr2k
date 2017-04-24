use super::conv_type::ConvType;

use super::normalize::Normalize;

pub trait PreProcess {
    type To;
    fn preprocess(&self) -> Self::To;
}

impl PreProcess for String {
    type To = String;

    fn preprocess(&self) -> Self::To {
        // `choose_kana()` works best on words
        fn choose_kana(s: &str) -> String {
            match s.chars().any(|c| c.is_uppercase()) {
                true => s.to_uppercase(),
                false => s.to_string(),
            }
        }

        self.split_whitespace()
            .map(|s| choose_kana(s))
            .collect()
    }
}

impl<'a> PreProcess for ConvType<&'a String> {
    type To = ConvType<String>;

    fn preprocess(&self) -> Self::To {
        self.normalize()
            .map3w(|x| x.preprocess(), /* A */
                   |x| x.to_lowercase(), /* H */
                   |x| x.to_uppercase()) /* K */
    }
}
