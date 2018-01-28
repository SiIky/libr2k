use std::collections::HashMap;

use super::conv_type::ConvType;
use super::conv_type::ConvType::*;

mod list;
use self::list::*;

use super::normalize::Normalize;
use super::preprocess::PreProcess;

/// The main character in the whole process.
pub struct KanaTable {
    hmap: HashMap<String, String>,
    maxlen: usize,
}

// Private methods
impl KanaTable {
    fn calc_maxlen(&mut self) {
        let mut maxlen = 0;

        for k in self.hmap.keys() {
            // this works because the keys are ASCII
            maxlen = ::std::cmp::max(maxlen, k.len());
        }

        self.maxlen = maxlen;
    }

    fn maxlen(&self) -> usize {
        self.maxlen
    }

    fn gojira(&self, s: &String) -> String {
        self.syllables(&s)
            .iter()
            .map(|x| self.convert_syllable(x))
            .collect()
    }
}

// Public methods
impl KanaTable {
    /// Create and return a new `KanaTable` ready to be used.
    pub fn new() -> KanaTable {
        // TODO: Complete the map
        let hmap: HashMap<String, String> = vec![
            // # Hiragana

            // ## Monograph
            ("a",   format!("{}", HIRA_A)),   // あ
            ("i",   format!("{}", HIRA_I)),   // い
            ("u",   format!("{}", HIRA_U)),   // う
            ("e",   format!("{}", HIRA_E)),   // え
            ("o",   format!("{}", HIRA_O)),   // お
            ("ka",  format!("{}", HIRA_KA)),  // か
            ("ga",  format!("{}", HIRA_GA)),  // が
            ("ki",  format!("{}", HIRA_KI)),  // き
            ("gi",  format!("{}", HIRA_GI)),  // ぎ
            ("ku",  format!("{}", HIRA_KU)),  // く
            ("gu",  format!("{}", HIRA_GU)),  // ぐ
            ("ke",  format!("{}", HIRA_KE)),  // け
            ("ge",  format!("{}", HIRA_GE)),  // げ
            ("ko",  format!("{}", HIRA_KO)),  // こ
            ("go",  format!("{}", HIRA_GO)),  // ご
            ("sa",  format!("{}", HIRA_SA)),  // さ
            ("za",  format!("{}", HIRA_ZA)),  // ざ
            ("shi", format!("{}", HIRA_SHI)), // し
            ("ji",  format!("{}", HIRA_JI)),  // じ
            ("su",  format!("{}", HIRA_SU)),  // す
            ("zu",  format!("{}", HIRA_ZU)),  // ず
            ("se",  format!("{}", HIRA_SE)),  // せ
            ("ze",  format!("{}", HIRA_ZE)),  // ぜ
            ("so",  format!("{}", HIRA_SO)),  // そ
            ("zo",  format!("{}", HIRA_ZO)),  // ぞ
            ("ta",  format!("{}", HIRA_TA)),  // た
            ("da",  format!("{}", HIRA_DA)),  // だ
            ("chi", format!("{}", HIRA_CHI)), // ち
            ("dji", format!("{}", HIRA_DJI)), // ぢ
            ("tsu", format!("{}", HIRA_TSU)), // つ
            ("dzu", format!("{}", HIRA_DZU)), // づ
            ("te",  format!("{}", HIRA_TE)),  // て
            ("de",  format!("{}", HIRA_DE)),  // で
            ("to",  format!("{}", HIRA_TO)),  // と
            ("do",  format!("{}", HIRA_DO)),  // ど
            ("na",  format!("{}", HIRA_NA)),  // な
            ("ni",  format!("{}", HIRA_NI)),  // に
            ("nu",  format!("{}", HIRA_NU)),  // ぬ
            ("ne",  format!("{}", HIRA_NE)),  // ね
            ("no",  format!("{}", HIRA_NO)),  // の
            ("ha",  format!("{}", HIRA_HA)),  // は
            ("ba",  format!("{}", HIRA_BA)),  // ば
            ("pa",  format!("{}", HIRA_PA)),  // ぱ
            ("hi",  format!("{}", HIRA_HI)),  // ひ
            ("bi",  format!("{}", HIRA_BI)),  // び
            ("pi",  format!("{}", HIRA_PI)),  // ぴ
            ("fu",  format!("{}", HIRA_FU)),  // ふ
            ("bu",  format!("{}", HIRA_BU)),  // ぶ
            ("pu",  format!("{}", HIRA_PU)),  // ぷ
            ("he",  format!("{}", HIRA_HE)),  // へ
            ("be",  format!("{}", HIRA_BE)),  // べ
            ("pe",  format!("{}", HIRA_PE)),  // ぺ
            ("ho",  format!("{}", HIRA_HO)),  // ほ
            ("bo",  format!("{}", HIRA_BO)),  // ぼ
            ("po",  format!("{}", HIRA_PO)),  // ぽ
            ("ma",  format!("{}", HIRA_MA)),  // ま
            ("mi",  format!("{}", HIRA_MI)),  // み
            ("mu",  format!("{}", HIRA_MU)),  // む
            ("me",  format!("{}", HIRA_ME)),  // め
            ("mo",  format!("{}", HIRA_MO)),  // も
            ("ya",  format!("{}", HIRA_YA)),  // や
            ("yu",  format!("{}", HIRA_YU)),  // ゆ
            ("yo",  format!("{}", HIRA_YO)),  // よ
            ("ra",  format!("{}", HIRA_RA)),  // ら
            ("ri",  format!("{}", HIRA_RI)),  // り
            ("ru",  format!("{}", HIRA_RU)),  // る
            ("re",  format!("{}", HIRA_RE)),  // れ
            ("ro",  format!("{}", HIRA_RO)),  // ろ
            ("wa",  format!("{}", HIRA_WA)),  // わ
            ("wi",  format!("{}", HIRA_WI)),  // ゐ
            ("we",  format!("{}", HIRA_WE)),  // ゑ
            ("wo",  format!("{}", HIRA_WO)),  // を
            ("n",   format!("{}", HIRA_N)),   // ん
            ("vu",  format!("{}", HIRA_VU)),  // ゔ

            // ## Digraph
            ("kya", format!("{}{}", HIRA_KI, HIRA_SMALL_YA)),  // きゃ
            ("kyu", format!("{}{}", HIRA_KI, HIRA_SMALL_YU)),  // きゅ
            ("kyo", format!("{}{}", HIRA_KI, HIRA_SMALL_YO)),  // きょ
            ("gya", format!("{}{}", HIRA_GI, HIRA_SMALL_YA)),  // ぎゃ
            ("gyu", format!("{}{}", HIRA_GI, HIRA_SMALL_YU)),  // ぎゅ
            ("gyo", format!("{}{}", HIRA_GI, HIRA_SMALL_YO)),  // ぎょ
            ("sha", format!("{}{}", HIRA_SHI, HIRA_SMALL_YA)), // しゃ
            ("shu", format!("{}{}", HIRA_SHI, HIRA_SMALL_YU)), // しゅ
            ("sho", format!("{}{}", HIRA_SHI, HIRA_SMALL_YO)), // しょ
            ("ja",  format!("{}{}", HIRA_JI, HIRA_SMALL_YA)),  // じゃ
            ("ju",  format!("{}{}", HIRA_JI, HIRA_SMALL_YU)),  // じゅ
            ("jo",  format!("{}{}", HIRA_JI, HIRA_SMALL_YO)),  // じょ
            ("cha", format!("{}{}", HIRA_CHI, HIRA_SMALL_YA)), // ちゃ
            ("chu", format!("{}{}", HIRA_CHI, HIRA_SMALL_YU)), // ちゅ
            ("cho", format!("{}{}", HIRA_CHI, HIRA_SMALL_YO)), // ちょ
            ("dja", format!("{}{}", HIRA_DJI, HIRA_SMALL_YA)), // ぢゃ
            ("dju", format!("{}{}", HIRA_DJI, HIRA_SMALL_YU)), // ぢゅ
            ("djo", format!("{}{}", HIRA_DJI, HIRA_SMALL_YO)), // ぢょ
            ("hya", format!("{}{}", HIRA_HI, HIRA_SMALL_YA)),  // ひゃ
            ("hyu", format!("{}{}", HIRA_HI, HIRA_SMALL_YU)),  // ひゅ
            ("hyo", format!("{}{}", HIRA_HI, HIRA_SMALL_YO)),  // ひょ
            ("bya", format!("{}{}", HIRA_BI, HIRA_SMALL_YA)),  // びゃ
            ("byu", format!("{}{}", HIRA_BI, HIRA_SMALL_YU)),  // びゅ
            ("byo", format!("{}{}", HIRA_BI, HIRA_SMALL_YO)),  // びょ
            ("pya", format!("{}{}", HIRA_PI, HIRA_SMALL_YA)),  // ぴゃ
            ("pyu", format!("{}{}", HIRA_PI, HIRA_SMALL_YU)),  // ぴゅ
            ("pyo", format!("{}{}", HIRA_PI, HIRA_SMALL_YO)),  // ぴょ
            ("nya", format!("{}{}", HIRA_NI, HIRA_SMALL_YA)),  // にゃ
            ("nyu", format!("{}{}", HIRA_NI, HIRA_SMALL_YU)),  // にゅ
            ("nyo", format!("{}{}", HIRA_NI, HIRA_SMALL_YO)),  // にょ

            // ## Pauses (small tsu)
            ("kka",  format!("{}{}", HIRA_SMALL_TSU, HIRA_KA)),  // っか
            ("gga",  format!("{}{}", HIRA_SMALL_TSU, HIRA_GA)),  // っが
            ("kki",  format!("{}{}", HIRA_SMALL_TSU, HIRA_KI)),  // っき
            ("ggi",  format!("{}{}", HIRA_SMALL_TSU, HIRA_GI)),  // っぎ
            ("kku",  format!("{}{}", HIRA_SMALL_TSU, HIRA_KU)),  // っく
            ("ggu",  format!("{}{}", HIRA_SMALL_TSU, HIRA_GU)),  // っぐ
            ("kke",  format!("{}{}", HIRA_SMALL_TSU, HIRA_KE)),  // っけ
            ("gge",  format!("{}{}", HIRA_SMALL_TSU, HIRA_GE)),  // っげ
            ("kko",  format!("{}{}", HIRA_SMALL_TSU, HIRA_KO)),  // っこ
            ("ggo",  format!("{}{}", HIRA_SMALL_TSU, HIRA_GO)),  // っご
            ("ssa",  format!("{}{}", HIRA_SMALL_TSU, HIRA_SA)),  // っさ
            ("zza",  format!("{}{}", HIRA_SMALL_TSU, HIRA_ZA)),  // っざ
            ("sshi", format!("{}{}", HIRA_SMALL_TSU, HIRA_SHI)), // っし
            ("jji",  format!("{}{}", HIRA_SMALL_TSU, HIRA_JI)),  // っじ
            ("ssu",  format!("{}{}", HIRA_SMALL_TSU, HIRA_SU)),  // っす
            ("zzu",  format!("{}{}", HIRA_SMALL_TSU, HIRA_ZU)),  // っず
            ("sse",  format!("{}{}", HIRA_SMALL_TSU, HIRA_SE)),  // っせ
            ("zze",  format!("{}{}", HIRA_SMALL_TSU, HIRA_ZE)),  // っぜ
            ("sso",  format!("{}{}", HIRA_SMALL_TSU, HIRA_SO)),  // っそ
            ("zzo",  format!("{}{}", HIRA_SMALL_TSU, HIRA_ZO)),  // っぞ
            ("tta",  format!("{}{}", HIRA_SMALL_TSU, HIRA_TA)),  // った
            ("dda",  format!("{}{}", HIRA_SMALL_TSU, HIRA_DA)),  // っだ
            ("cchi", format!("{}{}", HIRA_SMALL_TSU, HIRA_CHI)), // っち
            ("ddji", format!("{}{}", HIRA_SMALL_TSU, HIRA_DJI)), // っぢ
            ("ttsu", format!("{}{}", HIRA_SMALL_TSU, HIRA_TSU)), // っつ
            ("ddzu", format!("{}{}", HIRA_SMALL_TSU, HIRA_DZU)), // っづ
            ("tte",  format!("{}{}", HIRA_SMALL_TSU, HIRA_TE)),  // って
            ("dde",  format!("{}{}", HIRA_SMALL_TSU, HIRA_DE)),  // っで
            ("tto",  format!("{}{}", HIRA_SMALL_TSU, HIRA_TO)),  // っと
            ("ddo",  format!("{}{}", HIRA_SMALL_TSU, HIRA_DO)),  // っど
            ("nna",  format!("{}{}", HIRA_N, HIRA_NA)),          // んな
            ("nni",  format!("{}{}", HIRA_N, HIRA_NI)),          // んに
            ("nnu",  format!("{}{}", HIRA_N, HIRA_NU)),          // んぬ
            ("nne",  format!("{}{}", HIRA_N, HIRA_NE)),          // んね
            ("nno",  format!("{}{}", HIRA_N, HIRA_NO)),          // んの
            ("hha",  format!("{}{}", HIRA_SMALL_TSU, HIRA_HA)),  // っは
            ("bba",  format!("{}{}", HIRA_SMALL_TSU, HIRA_BA)),  // っば
            ("ppa",  format!("{}{}", HIRA_SMALL_TSU, HIRA_PA)),  // っぱ
            ("hhi",  format!("{}{}", HIRA_SMALL_TSU, HIRA_HI)),  // っひ
            ("bbi",  format!("{}{}", HIRA_SMALL_TSU, HIRA_BI)),  // っび
            ("ppi",  format!("{}{}", HIRA_SMALL_TSU, HIRA_PI)),  // っぴ
            ("ffu",  format!("{}{}", HIRA_SMALL_TSU, HIRA_FU)),  // っふ
            ("bbu",  format!("{}{}", HIRA_SMALL_TSU, HIRA_BU)),  // っぶ
            ("ppu",  format!("{}{}", HIRA_SMALL_TSU, HIRA_PU)),  // っぷ
            ("hhe",  format!("{}{}", HIRA_SMALL_TSU, HIRA_HE)),  // っへ
            ("bbe",  format!("{}{}", HIRA_SMALL_TSU, HIRA_BE)),  // っべ
            ("ppe",  format!("{}{}", HIRA_SMALL_TSU, HIRA_PE)),  // っぺ
            ("hho",  format!("{}{}", HIRA_SMALL_TSU, HIRA_HO)),  // っほ
            ("bbo",  format!("{}{}", HIRA_SMALL_TSU, HIRA_BO)),  // っぼ
            ("ppo",  format!("{}{}", HIRA_SMALL_TSU, HIRA_PO)),  // っぽ
            ("mma",  format!("{}{}", HIRA_SMALL_TSU, HIRA_MA)),  // っま
            ("mmi",  format!("{}{}", HIRA_SMALL_TSU, HIRA_MI)),  // っみ
            ("mmu",  format!("{}{}", HIRA_SMALL_TSU, HIRA_MU)),  // っむ
            ("mme",  format!("{}{}", HIRA_SMALL_TSU, HIRA_ME)),  // っめ
            ("mmo",  format!("{}{}", HIRA_SMALL_TSU, HIRA_MO)),  // っも
            ("yya",  format!("{}{}", HIRA_SMALL_TSU, HIRA_YA)),  // っや
            ("yyu",  format!("{}{}", HIRA_SMALL_TSU, HIRA_YU)),  // っゆ
            ("yyo",  format!("{}{}", HIRA_SMALL_TSU, HIRA_YO)),  // っよ
            ("rra",  format!("{}{}", HIRA_SMALL_TSU, HIRA_RA)),  // っら
            ("rri",  format!("{}{}", HIRA_SMALL_TSU, HIRA_RI)),  // っり
            ("rru",  format!("{}{}", HIRA_SMALL_TSU, HIRA_RU)),  // っる
            ("rre",  format!("{}{}", HIRA_SMALL_TSU, HIRA_RE)),  // っれ
            ("rro",  format!("{}{}", HIRA_SMALL_TSU, HIRA_RO)),  // っろ
            ("vvu",  format!("{}{}", HIRA_SMALL_TSU, HIRA_VU)),  // っゔ

            // ## Digraph
            ("kkya", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_KI, HIRA_SMALL_YA)),  // っきゃ
            ("kkyu", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_KI, HIRA_SMALL_YU)),  // っきゅ
            ("kkyo", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_KI, HIRA_SMALL_YO)),  // っきょ
            ("ggya", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_GI, HIRA_SMALL_YA)),  // っぎゃ
            ("ggyu", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_GI, HIRA_SMALL_YU)),  // っぎゅ
            ("ggyo", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_GI, HIRA_SMALL_YO)),  // っぎょ
            ("ssha", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_SHI, HIRA_SMALL_YA)), // っしゃ
            ("sshu", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_SHI, HIRA_SMALL_YU)), // っしゅ
            ("ssho", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_SHI, HIRA_SMALL_YO)), // っしょ
            ("jja",  format!("{}{}{}", HIRA_SMALL_TSU, HIRA_JI, HIRA_SMALL_YA)),  // っじゃ
            ("jju",  format!("{}{}{}", HIRA_SMALL_TSU, HIRA_JI, HIRA_SMALL_YU)),  // っじゅ
            ("jjo",  format!("{}{}{}", HIRA_SMALL_TSU, HIRA_JI, HIRA_SMALL_YO)),  // っじょ
            ("ccha", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_CHI, HIRA_SMALL_YA)), // っちゃ
            ("cchu", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_CHI, HIRA_SMALL_YU)), // っちゅ
            ("ccho", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_CHI, HIRA_SMALL_YO)), // っちょ
            ("ddja", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_DJI, HIRA_SMALL_YA)), // っぢゃ
            ("ddju", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_DJI, HIRA_SMALL_YU)), // っぢゅ
            ("ddjo", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_DJI, HIRA_SMALL_YO)), // っぢょ
            ("hhya", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_HI, HIRA_SMALL_YA)),  // っひゃ
            ("hhyu", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_HI, HIRA_SMALL_YU)),  // っひゅ
            ("hhyo", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_HI, HIRA_SMALL_YO)),  // っひょ
            ("bbya", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_BI, HIRA_SMALL_YA)),  // っびゃ
            ("bbyu", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_BI, HIRA_SMALL_YU)),  // っびゅ
            ("bbyo", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_BI, HIRA_SMALL_YO)),  // っびょ
            ("ppya", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_PI, HIRA_SMALL_YA)),  // っぴゃ
            ("ppyu", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_PI, HIRA_SMALL_YU)),  // っぴゅ
            ("ppyo", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_PI, HIRA_SMALL_YO)),  // っぴょ
            ("nnya", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_NI, HIRA_SMALL_YA)),  // っにゃ
            ("nnyu", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_NI, HIRA_SMALL_YU)),  // っにゅ
            ("nnyo", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_NI, HIRA_SMALL_YO)),  // っにょ

            // # Katakana
            ("A",   format!("{}", KATA_A)),   // ア
            ("I",   format!("{}", KATA_I)),   // イ
            ("U",   format!("{}", KATA_U)),   // ウ
            ("E",   format!("{}", KATA_E)),   // エ
            ("O",   format!("{}", KATA_O)),   // オ
            ("KA",  format!("{}", KATA_KA)),  // カ
            ("GA",  format!("{}", KATA_GA)),  // ガ
            ("KI",  format!("{}", KATA_KI)),  // キ
            ("GI",  format!("{}", KATA_GI)),  // ギ
            ("KU",  format!("{}", KATA_KU)),  // ク
            ("GU",  format!("{}", KATA_GU)),  // グ
            ("KE",  format!("{}", KATA_KE)),  // ケ
            ("GE",  format!("{}", KATA_GE)),  // ゲ
            ("KO",  format!("{}", KATA_KO)),  // コ
            ("GO",  format!("{}", KATA_GO)),  // ゴ
            ("SA",  format!("{}", KATA_SA)),  // サ
            ("ZA",  format!("{}", KATA_ZA)),  // ザ
            ("SHI", format!("{}", KATA_SHI)), // シ
            ("JI",  format!("{}", KATA_JI)),  // ジ
            ("SU",  format!("{}", KATA_SU)),  // ス
            ("ZU",  format!("{}", KATA_ZU)),  // ズ
            ("SE",  format!("{}", KATA_SE)),  // セ
            ("ZE",  format!("{}", KATA_ZE)),  // ゼ
            ("SO",  format!("{}", KATA_SO)),  // ソ
            ("ZO",  format!("{}", KATA_ZO)),  // ゾ
            ("TA",  format!("{}", KATA_TA)),  // タ
            ("DA",  format!("{}", KATA_DA)),  // ダ
            ("CHI", format!("{}", KATA_CHI)), // チ
            ("DJI", format!("{}", KATA_DJI)), // ヂ
            ("TSU", format!("{}", KATA_TSU)), // ツ
            ("DZU", format!("{}", KATA_DZU)), // ヅ
            ("TE",  format!("{}", KATA_TE)),  // テ
            ("DE",  format!("{}", KATA_DE)),  // デ
            ("TO",  format!("{}", KATA_TO)),  // ト
            ("DO",  format!("{}", KATA_DO)),  // ド
            ("NA",  format!("{}", KATA_NA)),  // ナ
            ("NI",  format!("{}", KATA_NI)),  // ニ
            ("NU",  format!("{}", KATA_NU)),  // ヌ
            ("NE",  format!("{}", KATA_NE)),  // ネ
            ("NO",  format!("{}", KATA_NO)),  // ノ
            ("HA",  format!("{}", KATA_HA)),  // ハ
            ("BA",  format!("{}", KATA_BA)),  // バ
            ("PA",  format!("{}", KATA_PA)),  // パ
            ("HI",  format!("{}", KATA_HI)),  // ヒ
            ("BI",  format!("{}", KATA_BI)),  // ビ
            ("PI",  format!("{}", KATA_PI)),  // ピ
            ("FU",  format!("{}", KATA_FU)),  // フ
            ("BU",  format!("{}", KATA_BU)),  // ブ
            ("PU",  format!("{}", KATA_PU)),  // プ
            ("HE",  format!("{}", KATA_HE)),  // ヘ
            ("BE",  format!("{}", KATA_BE)),  // ベ
            ("PE",  format!("{}", KATA_PE)),  // ペ
            ("HO",  format!("{}", KATA_HO)),  // ホ
            ("BO",  format!("{}", KATA_BO)),  // ボ
            ("PO",  format!("{}", KATA_PO)),  // ポ
            ("MA",  format!("{}", KATA_MA)),  // マ
            ("MI",  format!("{}", KATA_MI)),  // ミ
            ("MU",  format!("{}", KATA_MU)),  // ム
            ("ME",  format!("{}", KATA_ME)),  // メ
            ("MO",  format!("{}", KATA_MO)),  // モ
            ("YA",  format!("{}", KATA_YA)),  // ヤ
            ("YU",  format!("{}", KATA_YU)),  // ユ
            ("YO",  format!("{}", KATA_YO)),  // ヨ
            ("RA",  format!("{}", KATA_RA)),  // ラ
            ("RI",  format!("{}", KATA_RI)),  // リ
            ("RU",  format!("{}", KATA_RU)),  // ル
            ("RE",  format!("{}", KATA_RE)),  // レ
            ("RO",  format!("{}", KATA_RO)),  // ロ
            ("WA",  format!("{}", KATA_WA)),  // ワ
            ("WI",  format!("{}", KATA_WI)),  // ヰ
            ("WE",  format!("{}", KATA_WE)),  // ヱ
            ("WO",  format!("{}", KATA_WO)),  // ヲ
            ("N",   format!("{}", KATA_N)),   // ン
            ("VU",  format!("{}", KATA_VU)),  // ヴ

            // ## Digraph
            ("KYA", format!("{}{}", KATA_KI, KATA_SMALL_YA)),  // きゃ
            ("KYU", format!("{}{}", KATA_KI, KATA_SMALL_YU)),  // きゅ
            ("KYO", format!("{}{}", KATA_KI, KATA_SMALL_YO)),  // きょ
            ("GYA", format!("{}{}", KATA_GI, KATA_SMALL_YA)),  // ぎゃ
            ("GYU", format!("{}{}", KATA_GI, KATA_SMALL_YU)),  // ぎゅ
            ("GYO", format!("{}{}", KATA_GI, KATA_SMALL_YO)),  // ぎょ
            ("SHA", format!("{}{}", KATA_SHI, KATA_SMALL_YA)), // しゃ
            ("SHU", format!("{}{}", KATA_SHI, KATA_SMALL_YU)), // しゅ
            ("SHO", format!("{}{}", KATA_SHI, KATA_SMALL_YO)), // しょ
            ("JA",  format!("{}{}", KATA_JI, KATA_SMALL_YA)),  // じゃ
            ("JU",  format!("{}{}", KATA_JI, KATA_SMALL_YU)),  // じゅ
            ("JO",  format!("{}{}", KATA_JI, KATA_SMALL_YO)),  // じょ
            ("CHA", format!("{}{}", KATA_CHI, KATA_SMALL_YA)), // ちゃ
            ("CHU", format!("{}{}", KATA_CHI, KATA_SMALL_YU)), // ちゅ
            ("CHO", format!("{}{}", KATA_CHI, KATA_SMALL_YO)), // ちょ
            ("DJA", format!("{}{}", KATA_DJI, KATA_SMALL_YA)), // ぢゃ
            ("DJU", format!("{}{}", KATA_DJI, KATA_SMALL_YU)), // ぢゅ
            ("DJO", format!("{}{}", KATA_DJI, KATA_SMALL_YO)), // ぢょ
            ("HYA", format!("{}{}", KATA_HI, KATA_SMALL_YA)),  // ひゃ
            ("HYU", format!("{}{}", KATA_HI, KATA_SMALL_YU)),  // ひゅ
            ("HYO", format!("{}{}", KATA_HI, KATA_SMALL_YO)),  // ひょ
            ("BYA", format!("{}{}", KATA_BI, KATA_SMALL_YA)),  // びゃ
            ("BYU", format!("{}{}", KATA_BI, KATA_SMALL_YU)),  // びゅ
            ("BYO", format!("{}{}", KATA_BI, KATA_SMALL_YO)),  // びょ
            ("PYA", format!("{}{}", KATA_PI, KATA_SMALL_YA)),  // ぴゃ
            ("PYU", format!("{}{}", KATA_PI, KATA_SMALL_YU)),  // ぴゅ
            ("PYO", format!("{}{}", KATA_PI, KATA_SMALL_YO)),  // ぴょ
            ("NYA", format!("{}{}", KATA_NI, KATA_SMALL_YA)),  // にゃ
            ("NYU", format!("{}{}", KATA_NI, KATA_SMALL_YU)),  // にゅ
            ("NYO", format!("{}{}", KATA_NI, KATA_SMALL_YO)),  // にょ

            // ## Pauses (small tsu)
            ("KKA",  format!("{}{}", KATA_SMALL_TSU, KATA_KA)),  // ッカ
            ("GGA",  format!("{}{}", KATA_SMALL_TSU, KATA_GA)),  // ッガ
            ("KKI",  format!("{}{}", KATA_SMALL_TSU, KATA_KI)),  // ッキ
            ("GGI",  format!("{}{}", KATA_SMALL_TSU, KATA_GI)),  // ッギ
            ("KKU",  format!("{}{}", KATA_SMALL_TSU, KATA_KU)),  // ック
            ("GGU",  format!("{}{}", KATA_SMALL_TSU, KATA_GU)),  // ッグ
            ("KKE",  format!("{}{}", KATA_SMALL_TSU, KATA_KE)),  // ッケ
            ("GGE",  format!("{}{}", KATA_SMALL_TSU, KATA_GE)),  // ッゲ
            ("KKO",  format!("{}{}", KATA_SMALL_TSU, KATA_KO)),  // ッコ
            ("GGO",  format!("{}{}", KATA_SMALL_TSU, KATA_GO)),  // ッゴ
            ("SSA",  format!("{}{}", KATA_SMALL_TSU, KATA_SA)),  // ッサ
            ("ZZA",  format!("{}{}", KATA_SMALL_TSU, KATA_ZA)),  // ッザ
            ("SSHI", format!("{}{}", KATA_SMALL_TSU, KATA_SHI)), // ッシ
            ("JJI",  format!("{}{}", KATA_SMALL_TSU, KATA_JI)),  // ッジ
            ("SSU",  format!("{}{}", KATA_SMALL_TSU, KATA_SU)),  // ッス
            ("ZZU",  format!("{}{}", KATA_SMALL_TSU, KATA_ZU)),  // ッズ
            ("SSE",  format!("{}{}", KATA_SMALL_TSU, KATA_SE)),  // ッセ
            ("ZZE",  format!("{}{}", KATA_SMALL_TSU, KATA_ZE)),  // ッゼ
            ("SSO",  format!("{}{}", KATA_SMALL_TSU, KATA_SO)),  // ッソ
            ("ZZO",  format!("{}{}", KATA_SMALL_TSU, KATA_ZO)),  // ッゾ
            ("TTA",  format!("{}{}", KATA_SMALL_TSU, KATA_TA)),  // ッタ
            ("DDA",  format!("{}{}", KATA_SMALL_TSU, KATA_DA)),  // ッダ
            ("CCHI", format!("{}{}", KATA_SMALL_TSU, KATA_CHI)), // ッチ
            ("DDJI", format!("{}{}", KATA_SMALL_TSU, KATA_DJI)), // ッヂ
            ("TTSU", format!("{}{}", KATA_SMALL_TSU, KATA_TSU)), // ッツ
            ("DDZU", format!("{}{}", KATA_SMALL_TSU, KATA_DZU)), // ッヅ
            ("TTE",  format!("{}{}", KATA_SMALL_TSU, KATA_TE)),  // ッテ
            ("DDE",  format!("{}{}", KATA_SMALL_TSU, KATA_DE)),  // ッデ
            ("TTO",  format!("{}{}", KATA_SMALL_TSU, KATA_TO)),  // ット
            ("DDO",  format!("{}{}", KATA_SMALL_TSU, KATA_DO)),  // ッド
            ("NNA",  format!("{}{}", KATA_N, KATA_NA)),          // ンナ
            ("NNI",  format!("{}{}", KATA_N, KATA_NI)),          // ンニ
            ("NNU",  format!("{}{}", KATA_N, KATA_NU)),          // ンヌ
            ("NNE",  format!("{}{}", KATA_N, KATA_NE)),          // ンネ
            ("NNO",  format!("{}{}", KATA_N, KATA_NO)),          // ンノ
            ("HHA",  format!("{}{}", KATA_SMALL_TSU, KATA_HA)),  // ッハ
            ("BBA",  format!("{}{}", KATA_SMALL_TSU, KATA_BA)),  // ッバ
            ("PPA",  format!("{}{}", KATA_SMALL_TSU, KATA_PA)),  // ッパ
            ("HHI",  format!("{}{}", KATA_SMALL_TSU, KATA_HI)),  // ッヒ
            ("BBI",  format!("{}{}", KATA_SMALL_TSU, KATA_BI)),  // ッビ
            ("PPI",  format!("{}{}", KATA_SMALL_TSU, KATA_PI)),  // ッピ
            ("FFU",  format!("{}{}", KATA_SMALL_TSU, KATA_FU)),  // ッフ
            ("BBU",  format!("{}{}", KATA_SMALL_TSU, KATA_BU)),  // ッブ
            ("PPU",  format!("{}{}", KATA_SMALL_TSU, KATA_PU)),  // ップ
            ("HHE",  format!("{}{}", KATA_SMALL_TSU, KATA_HE)),  // ッヘ
            ("BBE",  format!("{}{}", KATA_SMALL_TSU, KATA_BE)),  // ッベ
            ("PPE",  format!("{}{}", KATA_SMALL_TSU, KATA_PE)),  // ッペ
            ("HHO",  format!("{}{}", KATA_SMALL_TSU, KATA_HO)),  // ッホ
            ("BBO",  format!("{}{}", KATA_SMALL_TSU, KATA_BO)),  // ッボ
            ("PPO",  format!("{}{}", KATA_SMALL_TSU, KATA_PO)),  // ッポ
            ("MMA",  format!("{}{}", KATA_SMALL_TSU, KATA_MA)),  // ッマ
            ("MMI",  format!("{}{}", KATA_SMALL_TSU, KATA_MI)),  // ッミ
            ("MMU",  format!("{}{}", KATA_SMALL_TSU, KATA_MU)),  // ッム
            ("MME",  format!("{}{}", KATA_SMALL_TSU, KATA_ME)),  // ッメ
            ("MMO",  format!("{}{}", KATA_SMALL_TSU, KATA_MO)),  // ッモ
            ("YYA",  format!("{}{}", KATA_SMALL_TSU, KATA_YA)),  // ッヤ
            ("YYU",  format!("{}{}", KATA_SMALL_TSU, KATA_YU)),  // ッユ
            ("YYO",  format!("{}{}", KATA_SMALL_TSU, KATA_YO)),  // ッヨ
            ("RRA",  format!("{}{}", KATA_SMALL_TSU, KATA_RA)),  // ッラ
            ("RRI",  format!("{}{}", KATA_SMALL_TSU, KATA_RI)),  // ッリ
            ("RRU",  format!("{}{}", KATA_SMALL_TSU, KATA_RU)),  // ッル
            ("RRE",  format!("{}{}", KATA_SMALL_TSU, KATA_RE)),  // ッレ
            ("RRO",  format!("{}{}", KATA_SMALL_TSU, KATA_RO)),  // ッロ
            ("VVU",  format!("{}{}", KATA_SMALL_TSU, KATA_VU)),  // ッヴ

            // lazy ass aproach to long vowels in Katakana
            // Maybe do a :substitute passage instead
            // https://doc.rust-lang.org/regex/regex/struct.Regex.html
            // https://doc.rust-lang.org/regex/regex/index.html
            ("AA", format!("{}{}", KATA_A, PUNCT_CHOONPU)), // アー
            ("II", format!("{}{}", KATA_I, PUNCT_CHOONPU)), // イー
            ("UU", format!("{}{}", KATA_U, PUNCT_CHOONPU)), // ウー
            ("EE", format!("{}{}", KATA_E, PUNCT_CHOONPU)), // エー
            ("OO", format!("{}{}", KATA_O, PUNCT_CHOONPU)), // オー

            // Punctuation
            (".", format!("{}", PUNCT_PERIOD)),  // 。
            (",", format!("{}", PUNCT_COMMA)),   // 、
            ("!", format!("{}", PUNCT_EMARK)),   // ！
            ("?", format!("{}", PUNCT_QMARK)),   // ？
            ("-", format!("{}", PUNCT_CHOONPU)), // ー
        ].into_iter()
            .map(|p| (p.0.to_string(), p.1))
            .collect();

        let mut ret = KanaTable {
            hmap: hmap,
            maxlen: 0,
        };

        ret.calc_maxlen();

        ret
    }

    /// Calculate the kana corresponding to given syllable,
    /// or return back the same string if there is none.
    pub fn convert_syllable(&self, s: &String) -> String {
        match self.hmap.get(s) {
            Some(v) => v,
            None => s,
        }.clone()
    }

    /// Separate a string into syllables.
    pub fn syllables(&self, o: &String) -> Vec<String> {
        let mut ret: Vec<String> = Vec::new();

        for word in o.split_whitespace() {
            let mut s: String = word.to_string();

            while !s.is_empty() {
                let mut n: usize = 1;
                let maxlen = ::std::cmp::min(s.chars().count(), self.maxlen()) + 1;
                // [1, 2, .., maxlen-1].rev()
                for len in (1..maxlen).rev() {
                    let syl: String = s.chars().take(len).collect();

                    if self.is_syllable(&syl) {
                        n = len;
                        break;
                    }
                }

                //-- what i did
                //let (syl, rest) = (s.chars().take(n).collect(), s.chars().skip(n).collect());
                //-- char_indices()
                //let mid = s.char_indices().nth(5).map_or(s.len(), |(i, _)| i);
                //-- chars iterator
                //let tupple: (String, String) = {
                //    let mut chars = s.chars();
                //    let a = chars.by_ref().take(5).collect();
                //    let b = chars.collect();
                //    (a, b)
                //};

                let (syl, rest): (String, String) = {
                    let mut chars = s.chars();
                    let syl: String = chars.by_ref().take(n).collect();
                    let rest: String = chars.collect();
                    (syl, rest)
                };

                ret.push(syl);
                s = rest;
            }
        }

        ret
    }

    /// Check if a string is a valid romaji syllable.
    pub fn is_syllable(&self, syl: &String) -> bool {
        self.hmap.contains_key(syl)
    }

    /// Convert a string to kana, after normalizing and preprocessing.
    pub fn convert(&self, ct: ConvType<&String>) -> String {
        self.gojira(&ct.normalize().preprocess().unwrap())
    }

    /// Convert a string to kana, with auto-detection.
    pub fn to_kana(&self, s: &String) -> String {
        self.convert(Auto(s))
    }

    /// Convert a string to hiragana.
    pub fn to_hira(&self, s: &String) -> String {
        self.convert(Hira(s))
    }

    /// Convert a string to katakana.
    pub fn to_kata(&self, s: &String) -> String {
        self.convert(Kata(s))
    }
}
