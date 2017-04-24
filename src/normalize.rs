extern crate kana;
use self::kana::{combine, half2kana, nowidespace, nowideyen, wide2ascii};

use super::conv_type::ConvType;

pub trait Normalize {
    type To;
    fn normalize(&self) -> Self::To;
}

impl Normalize for String {
    type To = String;

    fn normalize(&self) -> Self::To {
        let mut ret: Self::To = self.clone();

        ret = nowidespace(ret.as_str());
        ret = nowideyen(ret.as_str());
        ret = wide2ascii(ret.as_str());

        ret = combine(ret.as_str());
        ret = half2kana(ret.as_str());

        ret
    }
}

impl<'a> Normalize for ConvType<&'a String> {
    type To = ConvType<String>;

    fn normalize(&self) -> Self::To {
        self.map(|s| s.normalize())
    }
}
