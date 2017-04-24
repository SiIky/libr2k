use std::collections::HashMap;

use super::conv_type::ConvType;
use super::conv_type::ConvType::*;

mod list;
use self::list::*;

use super::preprocess::PreProcess;

pub struct KanaTable {
    map: HashMap<String, String>,
    maxlen: usize,
}

// Private methods
impl KanaTable {
    fn calc_maxlen(&mut self) {
        self.maxlen = 0;

        for k in self.map.keys() {
            // this works because the keys are ASCII
            let klen: usize = k.len();
            self.maxlen = ::std::cmp::max(self.maxlen, klen);
        }
    }

    fn maxlen(&self) -> usize {
        self.maxlen
    }

    fn gojira(&self, s: String) -> String {
        self.syllables(&s)
            .iter()
            .map(|x| self.convert_syllable(x))
            .collect()
    }
}

// Public methods
impl KanaTable {
    pub fn new() -> KanaTable {
        let map: HashMap<String, String> =
            vec![// # Hiragana
                 // ## Monograph
                 ("a", format!("{}", HIRA_A)), // あ
                 ("i", format!("{}", HIRA_I)), // い
                 ("u", format!("{}", HIRA_U)), // う
                 ("e", format!("{}", HIRA_E)), // え
                 ("o", format!("{}", HIRA_O)), // お
                 ("ka", format!("{}", HIRA_KA)), // か
                 ("ga", format!("{}", HIRA_GA)), // が
                 ("ki", format!("{}", HIRA_KI)), // き
                 ("gi", format!("{}", HIRA_GI)), // ぎ
                 ("ku", format!("{}", HIRA_KU)), // く
                 ("gu", format!("{}", HIRA_GU)), // ぐ
                 ("ke", format!("{}", HIRA_KE)), // け
                 ("ge", format!("{}", HIRA_GE)), // げ
                 ("ko", format!("{}", HIRA_KO)), // こ
                 ("go", format!("{}", HIRA_GO)), // ご
                 ("sa", format!("{}", HIRA_SA)), // さ
                 ("za", format!("{}", HIRA_ZA)), // ざ
                 ("shi", format!("{}", HIRA_SHI)), // し
                 ("ji", format!("{}", HIRA_JI)), // じ
                 ("su", format!("{}", HIRA_SU)), // す
                 ("zu", format!("{}", HIRA_ZU)), // ず
                 ("se", format!("{}", HIRA_SE)), // せ
                 ("ze", format!("{}", HIRA_ZE)), // ぜ
                 ("so", format!("{}", HIRA_SO)), // そ
                 ("zo", format!("{}", HIRA_ZO)), // ぞ
                 ("ta", format!("{}", HIRA_TA)), // た
                 ("da", format!("{}", HIRA_DA)), // だ
                 ("chi", format!("{}", HIRA_CHI)), // ち
                 ("dji", format!("{}", HIRA_DJI)), // ぢ
                 ("tsu", format!("{}", HIRA_TSU)), // つ
                 ("dzu", format!("{}", HIRA_DZU)), // づ
                 ("te", format!("{}", HIRA_TE)), // て
                 ("de", format!("{}", HIRA_DE)), // で
                 ("to", format!("{}", HIRA_TO)), // と
                 ("do", format!("{}", HIRA_DO)), // ど
                 ("na", format!("{}", HIRA_NA)), // な
                 ("ni", format!("{}", HIRA_NI)), // に
                 ("nu", format!("{}", HIRA_NU)), // ぬ
                 ("ne", format!("{}", HIRA_NE)), // ね
                 ("no", format!("{}", HIRA_NO)), // の
                 ("ha", format!("{}", HIRA_HA)), // は
                 ("ba", format!("{}", HIRA_BA)), // ば
                 ("pa", format!("{}", HIRA_PA)), // ぱ
                 ("hi", format!("{}", HIRA_HI)), // ひ
                 ("bi", format!("{}", HIRA_BI)), // び
                 ("pi", format!("{}", HIRA_PI)), // ぴ
                 ("fu", format!("{}", HIRA_FU)), // ふ
                 ("bu", format!("{}", HIRA_BU)), // ぶ
                 ("pu", format!("{}", HIRA_PU)), // ぷ
                 ("he", format!("{}", HIRA_HE)), // へ
                 ("be", format!("{}", HIRA_BE)), // べ
                 ("pe", format!("{}", HIRA_PE)), // ぺ
                 ("ho", format!("{}", HIRA_HO)), // ほ
                 ("bo", format!("{}", HIRA_BO)), // ぼ
                 ("po", format!("{}", HIRA_PO)), // ぽ
                 ("ma", format!("{}", HIRA_MA)), // ま
                 ("mi", format!("{}", HIRA_MI)), // み
                 ("mu", format!("{}", HIRA_MU)), // む
                 ("me", format!("{}", HIRA_ME)), // め
                 ("mo", format!("{}", HIRA_MO)), // も
                 ("ya", format!("{}", HIRA_YA)), // や
                 ("yu", format!("{}", HIRA_YU)), // ゆ
                 ("yo", format!("{}", HIRA_YO)), // よ
                 ("ra", format!("{}", HIRA_RA)), // ら
                 ("ri", format!("{}", HIRA_RI)), // り
                 ("ru", format!("{}", HIRA_RU)), // る
                 ("re", format!("{}", HIRA_RE)), // れ
                 ("ro", format!("{}", HIRA_RO)), // ろ
                 ("wa", format!("{}", HIRA_WA)), // わ
                 ("wi", format!("{}", HIRA_WI)), // ゐ
                 ("we", format!("{}", HIRA_WE)), // ゑ
                 ("wo", format!("{}", HIRA_WO)), // を
                 ("n", format!("{}", HIRA_N)), // ん
                 ("vu", format!("{}", HIRA_VU)), // ゔ
                 // ## Digraph
                 ("kya", format!("{}{}", HIRA_KI, HIRA_SMALL_YA)), // きゃ
                 ("kyu", format!("{}{}", HIRA_KI, HIRA_SMALL_YU)), // きゅ
                 ("kyo", format!("{}{}", HIRA_KI, HIRA_SMALL_YO)), // きょ
                 ("gya", format!("{}{}", HIRA_GI, HIRA_SMALL_YA)), // ぎゃ
                 ("gyu", format!("{}{}", HIRA_GI, HIRA_SMALL_YU)), // ぎゅ
                 ("gyo", format!("{}{}", HIRA_GI, HIRA_SMALL_YO)), // ぎょ
                 ("sha", format!("{}{}", HIRA_SHI, HIRA_SMALL_YA)), // しゃ
                 ("shu", format!("{}{}", HIRA_SHI, HIRA_SMALL_YU)), // しゅ
                 ("sho", format!("{}{}", HIRA_SHI, HIRA_SMALL_YO)), // しょ
                 ("ja", format!("{}{}", HIRA_JI, HIRA_SMALL_YA)), // じゃ
                 ("ju", format!("{}{}", HIRA_JI, HIRA_SMALL_YU)), // じゅ
                 ("jo", format!("{}{}", HIRA_JI, HIRA_SMALL_YO)), // じょ
                 ("cha", format!("{}{}", HIRA_CHI, HIRA_SMALL_YA)), // ちゃ
                 ("chu", format!("{}{}", HIRA_CHI, HIRA_SMALL_YU)), // ちゅ
                 ("cho", format!("{}{}", HIRA_CHI, HIRA_SMALL_YO)), // ちょ
                 ("dja", format!("{}{}", HIRA_DJI, HIRA_SMALL_YA)), // ぢゃ
                 ("dju", format!("{}{}", HIRA_DJI, HIRA_SMALL_YU)), // ぢゅ
                 ("djo", format!("{}{}", HIRA_DJI, HIRA_SMALL_YO)), // ぢょ
                 ("hya", format!("{}{}", HIRA_HI, HIRA_SMALL_YA)), // ひゃ
                 ("hyu", format!("{}{}", HIRA_HI, HIRA_SMALL_YU)), // ひゅ
                 ("hyo", format!("{}{}", HIRA_HI, HIRA_SMALL_YO)), // ひょ
                 ("bya", format!("{}{}", HIRA_BI, HIRA_SMALL_YA)), // びゃ
                 ("byu", format!("{}{}", HIRA_BI, HIRA_SMALL_YU)), // びゅ
                 ("byo", format!("{}{}", HIRA_BI, HIRA_SMALL_YO)), // びょ
                 ("pya", format!("{}{}", HIRA_PI, HIRA_SMALL_YA)), // ぴゃ
                 ("pyu", format!("{}{}", HIRA_PI, HIRA_SMALL_YU)), // ぴゅ
                 ("pyo", format!("{}{}", HIRA_PI, HIRA_SMALL_YO)), // ぴょ
                 ("nya", format!("{}{}", HIRA_NI, HIRA_SMALL_YA)), // にゃ
                 ("nyu", format!("{}{}", HIRA_NI, HIRA_SMALL_YU)), // にゅ
                 ("nyo", format!("{}{}", HIRA_NI, HIRA_SMALL_YO)), // にょ
                 // ## Pauses (small tsu)
                 ("kka", format!("{}{}", HIRA_SMALL_TSU, HIRA_KA)), // か
                 ("gga", format!("{}{}", HIRA_SMALL_TSU, HIRA_GA)), // が
                 ("kki", format!("{}{}", HIRA_SMALL_TSU, HIRA_KI)), // き
                 ("ggi", format!("{}{}", HIRA_SMALL_TSU, HIRA_GI)), // ぎ
                 ("kku", format!("{}{}", HIRA_SMALL_TSU, HIRA_KU)), // く
                 ("ggu", format!("{}{}", HIRA_SMALL_TSU, HIRA_GU)), // ぐ
                 ("kke", format!("{}{}", HIRA_SMALL_TSU, HIRA_KE)), // け
                 ("gge", format!("{}{}", HIRA_SMALL_TSU, HIRA_GE)), // げ
                 ("kko", format!("{}{}", HIRA_SMALL_TSU, HIRA_KO)), // こ
                 ("ggo", format!("{}{}", HIRA_SMALL_TSU, HIRA_GO)), // ご
                 ("ssa", format!("{}{}", HIRA_SMALL_TSU, HIRA_SA)), // さ
                 ("zza", format!("{}{}", HIRA_SMALL_TSU, HIRA_ZA)), // ざ
                 ("sshi", format!("{}{}", HIRA_SMALL_TSU, HIRA_SHI)), // し
                 ("jji", format!("{}{}", HIRA_SMALL_TSU, HIRA_JI)), // じ
                 ("ssu", format!("{}{}", HIRA_SMALL_TSU, HIRA_SU)), // す
                 ("zzu", format!("{}{}", HIRA_SMALL_TSU, HIRA_ZU)), // ず
                 ("sse", format!("{}{}", HIRA_SMALL_TSU, HIRA_SE)), // せ
                 ("zze", format!("{}{}", HIRA_SMALL_TSU, HIRA_ZE)), // ぜ
                 ("sso", format!("{}{}", HIRA_SMALL_TSU, HIRA_SO)), // そ
                 ("zzo", format!("{}{}", HIRA_SMALL_TSU, HIRA_ZO)), // ぞ
                 ("tta", format!("{}{}", HIRA_SMALL_TSU, HIRA_TA)), // た
                 ("dda", format!("{}{}", HIRA_SMALL_TSU, HIRA_DA)), // だ
                 ("cchi", format!("{}{}", HIRA_SMALL_TSU, HIRA_CHI)), // ち
                 ("ddji", format!("{}{}", HIRA_SMALL_TSU, HIRA_DJI)), // ぢ
                 ("ttsu", format!("{}{}", HIRA_SMALL_TSU, HIRA_TSU)), // つ
                 ("ddzu", format!("{}{}", HIRA_SMALL_TSU, HIRA_DZU)), // づ
                 ("tte", format!("{}{}", HIRA_SMALL_TSU, HIRA_TE)), // て
                 ("dde", format!("{}{}", HIRA_SMALL_TSU, HIRA_DE)), // で
                 ("tto", format!("{}{}", HIRA_SMALL_TSU, HIRA_TO)), // と
                 ("ddo", format!("{}{}", HIRA_SMALL_TSU, HIRA_DO)), // ど
                 ("nna", format!("{}{}", HIRA_SMALL_TSU, HIRA_NA)), // な
                 ("nni", format!("{}{}", HIRA_SMALL_TSU, HIRA_NI)), // に
                 ("nnu", format!("{}{}", HIRA_SMALL_TSU, HIRA_NU)), // ぬ
                 ("nne", format!("{}{}", HIRA_SMALL_TSU, HIRA_NE)), // ね
                 ("nno", format!("{}{}", HIRA_SMALL_TSU, HIRA_NO)), // の
                 ("hha", format!("{}{}", HIRA_SMALL_TSU, HIRA_HA)), // は
                 ("bba", format!("{}{}", HIRA_SMALL_TSU, HIRA_BA)), // ば
                 ("ppa", format!("{}{}", HIRA_SMALL_TSU, HIRA_PA)), // ぱ
                 ("hhi", format!("{}{}", HIRA_SMALL_TSU, HIRA_HI)), // ひ
                 ("bbi", format!("{}{}", HIRA_SMALL_TSU, HIRA_BI)), // び
                 ("ppi", format!("{}{}", HIRA_SMALL_TSU, HIRA_PI)), // ぴ
                 ("ffu", format!("{}{}", HIRA_SMALL_TSU, HIRA_FU)), // ふ
                 ("bbu", format!("{}{}", HIRA_SMALL_TSU, HIRA_BU)), // ぶ
                 ("ppu", format!("{}{}", HIRA_SMALL_TSU, HIRA_PU)), // ぷ
                 ("hhe", format!("{}{}", HIRA_SMALL_TSU, HIRA_HE)), // へ
                 ("bbe", format!("{}{}", HIRA_SMALL_TSU, HIRA_BE)), // べ
                 ("ppe", format!("{}{}", HIRA_SMALL_TSU, HIRA_PE)), // ぺ
                 ("hho", format!("{}{}", HIRA_SMALL_TSU, HIRA_HO)), // ほ
                 ("bbo", format!("{}{}", HIRA_SMALL_TSU, HIRA_BO)), // ぼ
                 ("ppo", format!("{}{}", HIRA_SMALL_TSU, HIRA_PO)), // ぽ
                 ("mma", format!("{}{}", HIRA_SMALL_TSU, HIRA_MA)), // ま
                 ("mmi", format!("{}{}", HIRA_SMALL_TSU, HIRA_MI)), // み
                 ("mmu", format!("{}{}", HIRA_SMALL_TSU, HIRA_MU)), // む
                 ("mme", format!("{}{}", HIRA_SMALL_TSU, HIRA_ME)), // め
                 ("mmo", format!("{}{}", HIRA_SMALL_TSU, HIRA_MO)), // も
                 ("yya", format!("{}{}", HIRA_SMALL_TSU, HIRA_YA)), // や
                 ("yyu", format!("{}{}", HIRA_SMALL_TSU, HIRA_YU)), // ゆ
                 ("yyo", format!("{}{}", HIRA_SMALL_TSU, HIRA_YO)), // よ
                 ("rra", format!("{}{}", HIRA_SMALL_TSU, HIRA_RA)), // ら
                 ("rri", format!("{}{}", HIRA_SMALL_TSU, HIRA_RI)), // り
                 ("rru", format!("{}{}", HIRA_SMALL_TSU, HIRA_RU)), // る
                 ("rre", format!("{}{}", HIRA_SMALL_TSU, HIRA_RE)), // れ
                 ("rro", format!("{}{}", HIRA_SMALL_TSU, HIRA_RO)), // ろ
                 ("vvu", format!("{}{}", HIRA_SMALL_TSU, HIRA_VU)), // ゔ
                 //  ## Digraph
                 ("kkya", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_KI, HIRA_SMALL_YA)), // きゃ
                 ("kkyu", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_KI, HIRA_SMALL_YU)), // きゅ
                 ("kkyo", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_KI, HIRA_SMALL_YO)), // きょ
                 ("ggya", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_GI, HIRA_SMALL_YA)), // ぎゃ
                 ("ggyu", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_GI, HIRA_SMALL_YU)), // ぎゅ
                 ("ggyo", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_GI, HIRA_SMALL_YO)), // ぎょ
                 ("ssha", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_SHI, HIRA_SMALL_YA)), // しゃ
                 ("sshu", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_SHI, HIRA_SMALL_YU)), // しゅ
                 ("ssho", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_SHI, HIRA_SMALL_YO)), // しょ
                 ("jja", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_JI, HIRA_SMALL_YA)), // じゃ
                 ("jju", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_JI, HIRA_SMALL_YU)), // じゅ
                 ("jjo", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_JI, HIRA_SMALL_YO)), // じょ
                 ("ccha", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_CHI, HIRA_SMALL_YA)), // ちゃ
                 ("cchu", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_CHI, HIRA_SMALL_YU)), // ちゅ
                 ("ccho", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_CHI, HIRA_SMALL_YO)), // ちょ
                 ("ddja", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_DJI, HIRA_SMALL_YA)), // ぢゃ
                 ("ddju", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_DJI, HIRA_SMALL_YU)), // ぢゅ
                 ("ddjo", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_DJI, HIRA_SMALL_YO)), // ぢょ
                 ("hhya", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_HI, HIRA_SMALL_YA)), // ひゃ
                 ("hhyu", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_HI, HIRA_SMALL_YU)), // ひゅ
                 ("hhyo", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_HI, HIRA_SMALL_YO)), // ひょ
                 ("bbya", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_BI, HIRA_SMALL_YA)), // びゃ
                 ("bbyu", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_BI, HIRA_SMALL_YU)), // びゅ
                 ("bbyo", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_BI, HIRA_SMALL_YO)), // びょ
                 ("ppya", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_PI, HIRA_SMALL_YA)), // ぴゃ
                 ("ppyu", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_PI, HIRA_SMALL_YU)), // ぴゅ
                 ("ppyo", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_PI, HIRA_SMALL_YO)), // ぴょ
                 ("nnya", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_NI, HIRA_SMALL_YA)), // にゃ
                 ("nnyu", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_NI, HIRA_SMALL_YU)), // にゅ
                 ("nnyo", format!("{}{}{}", HIRA_SMALL_TSU, HIRA_NI, HIRA_SMALL_YO)), // にょ
                 // # Katakana
                 ("A", format!("{}", KATA_A)), // ア
                 ("I", format!("{}", KATA_I)), // イ
                 ("U", format!("{}", KATA_U)), // ウ
                 ("E", format!("{}", KATA_E)), // エ
                 ("O", format!("{}", KATA_O)), // オ
                 ("KA", format!("{}", KATA_KA)), // カ
                 ("GA", format!("{}", KATA_GA)), // ガ
                 ("KI", format!("{}", KATA_KI)), // キ
                 ("GI", format!("{}", KATA_GI)), // ギ
                 ("KU", format!("{}", KATA_KU)), // ク
                 ("GU", format!("{}", KATA_GU)), // グ
                 ("KE", format!("{}", KATA_KE)), // ケ
                 ("GE", format!("{}", KATA_GE)), // ゲ
                 ("KO", format!("{}", KATA_KO)), // コ
                 ("GO", format!("{}", KATA_GO)), // ゴ
                 ("SA", format!("{}", KATA_SA)), // サ
                 ("ZA", format!("{}", KATA_ZA)), // ザ
                 ("SHI", format!("{}", KATA_SHI)), // シ
                 ("JI", format!("{}", KATA_JI)), // ジ
                 ("SU", format!("{}", KATA_SU)), // ス
                 ("ZU", format!("{}", KATA_ZU)), // ズ
                 ("SE", format!("{}", KATA_SE)), // セ
                 ("ZE", format!("{}", KATA_ZE)), // ゼ
                 ("SO", format!("{}", KATA_SO)), // ソ
                 ("ZO", format!("{}", KATA_ZO)), // ゾ
                 ("TA", format!("{}", KATA_TA)), // タ
                 ("DA", format!("{}", KATA_DA)), // ダ
                 ("CHI", format!("{}", KATA_CHI)), // チ
                 ("DJI", format!("{}", KATA_DJI)), // ヂ
                 ("TSU", format!("{}", KATA_TSU)), // ツ
                 ("DZU", format!("{}", KATA_DZU)), // ヅ
                 ("TE", format!("{}", KATA_TE)), // テ
                 ("DE", format!("{}", KATA_DE)), // デ
                 ("TO", format!("{}", KATA_TO)), // ト
                 ("DO", format!("{}", KATA_DO)), // ド
                 ("NA", format!("{}", KATA_NA)), // ナ
                 ("NI", format!("{}", KATA_NI)), // ニ
                 ("NU", format!("{}", KATA_NU)), // ヌ
                 ("NE", format!("{}", KATA_NE)), // ネ
                 ("NO", format!("{}", KATA_NO)), // ノ
                 ("HA", format!("{}", KATA_HA)), // ハ
                 ("BA", format!("{}", KATA_BA)), // バ
                 ("PA", format!("{}", KATA_PA)), // パ
                 ("HI", format!("{}", KATA_HI)), // ヒ
                 ("BI", format!("{}", KATA_BI)), // ビ
                 ("PI", format!("{}", KATA_PI)), // ピ
                 ("FU", format!("{}", KATA_FU)), // フ
                 ("BU", format!("{}", KATA_BU)), // ブ
                 ("PU", format!("{}", KATA_PU)), // プ
                 ("HE", format!("{}", KATA_HE)), // ヘ
                 ("BE", format!("{}", KATA_BE)), // ベ
                 ("PE", format!("{}", KATA_PE)), // ペ
                 ("HO", format!("{}", KATA_HO)), // ホ
                 ("BO", format!("{}", KATA_BO)), // ボ
                 ("PO", format!("{}", KATA_PO)), // ポ
                 ("MA", format!("{}", KATA_MA)), // マ
                 ("MI", format!("{}", KATA_MI)), // ミ
                 ("MU", format!("{}", KATA_MU)), // ム
                 ("ME", format!("{}", KATA_ME)), // メ
                 ("MO", format!("{}", KATA_MO)), // モ
                 ("YA", format!("{}", KATA_YA)), // ヤ
                 ("YU", format!("{}", KATA_YU)), // ユ
                 ("YO", format!("{}", KATA_YO)), // ヨ
                 ("RA", format!("{}", KATA_RA)), // ラ
                 ("RI", format!("{}", KATA_RI)), // リ
                 ("RU", format!("{}", KATA_RU)), // ル
                 ("RE", format!("{}", KATA_RE)), // レ
                 ("RO", format!("{}", KATA_RO)), // ロ
                 ("WA", format!("{}", KATA_WA)), // ワ
                 ("WI", format!("{}", KATA_WI)), // ヰ
                 ("WE", format!("{}", KATA_WE)), // ヱ
                 ("WO", format!("{}", KATA_WO)), // ヲ
                 ("N", format!("{}", KATA_N)), // ン
                 ("VU", format!("{}", KATA_VU)), // ヴ
                 ("KYA", format!("{}", KATA_A)), // きゃ
                 ("KYU", format!("{}", KATA_A)), // きゅ
                 ("KYO", format!("{}", KATA_A)), // きょ
                 ("GYA", format!("{}", KATA_A)), // ぎゃ
                 ("GYU", format!("{}", KATA_A)), // ぎゅ
                 ("GYO", format!("{}", KATA_A)), // ぎょ
                 ("SHA", format!("{}", KATA_A)), // しゃ
                 ("SHU", format!("{}", KATA_A)), // しゅ
                 ("SHO", format!("{}", KATA_A)), // しょ
                 ("JA", format!("{}", KATA_A)), // じゃ
                 ("JU", format!("{}", KATA_A)), // じゅ
                 ("JO", format!("{}", KATA_A)), // じょ
                 ("CHA", format!("{}", KATA_A)), // ちゃ
                 ("CHU", format!("{}", KATA_A)), // ちゅ
                 ("CHO", format!("{}", KATA_A)), // ちょ
                 ("DJA", format!("{}", KATA_A)), // ぢゃ
                 ("DJU", format!("{}", KATA_A)), // ぢゅ
                 ("DJO", format!("{}", KATA_A)), // ぢょ
                 ("HYA", format!("{}", KATA_A)), // ひゃ
                 ("HYU", format!("{}", KATA_A)), // ひゅ
                 ("HYO", format!("{}", KATA_A)), // ひょ
                 ("BYA", format!("{}", KATA_A)), // びゃ
                 ("BYU", format!("{}", KATA_A)), // びゅ
                 ("BYO", format!("{}", KATA_A)), // びょ
                 ("PYA", format!("{}", KATA_A)), // ぴゃ
                 ("PYU", format!("{}", KATA_A)), // ぴゅ
                 ("PYO", format!("{}", KATA_A)), // ぴょ
                 ("NYA", format!("{}", KATA_A)), // にゃ
                 ("NYU", format!("{}", KATA_A)), // にゅ
                 ("NYO", format!("{}", KATA_A)), // にょ
                 // lazy ass aproach to long vowels in Katakana
                 // Maybe do a :substitute passage instead
                 // https://doc.rust-lang.org/regex/regex/struct.Regex.html
                 // https://doc.rust-lang.org/regex/regex/index.html
                 ("AA", format!("{}{}", KATA_A, PUNCT_CHOONPU)), // アー
                 ("II", format!("{}{}", KATA_I, PUNCT_CHOONPU)), // イー
                 ("UU", format!("{}{}", KATA_U, PUNCT_CHOONPU)), // ウー
                 ("EE", format!("{}{}", KATA_E, PUNCT_CHOONPU)), // エー
                 ("OO", format!("{}{}", KATA_O, PUNCT_CHOONPU)), // オー
                 // ## Pauses (small tsu)
                 // Punctuation
                 (".", format!("{}", PUNCT_PERIOD)), // 。
                 (",", format!("{}", PUNCT_COMMA)), // 、
                 ("!", format!("{}", PUNCT_EMARK)), // ！
                 ("?", format!("{}", PUNCT_QMARK)), // ？
                 ("-", format!("{}", PUNCT_CHOONPU))] // ー
                    .into_iter()
                    .map(|p| (p.0.to_string(), p.1))
                    .collect();

        let mut ret = KanaTable {
            map: map,
            maxlen: 0,
        };

        ret.calc_maxlen();
        let ret = ret;

        ret
    }

    pub fn convert_syllable(&self, s: &String) -> String {
        match self.map.get(s) {
            Some(v) => v.clone(),
            None => s.clone(),
        }
    }

    pub fn syllables(&self, s: &String) -> Vec<String> {
        let mut ret: Vec<String> = Vec::new();
        let mut s = s.clone();

        while !s.is_empty() {
            let mut n: usize = 1;
            let maxlen = ::std::cmp::min(s.chars().count(), self.maxlen()) + 1;
            // [1, 2, .., maxlen-1].rev()
            for len in (1..maxlen).rev() {
                let tmp: String = s.chars().take(len).collect();

                if self.is_syllable(&tmp) {
                    n = len;
                    break;
                }
            }

            /*
               -- what i did
               let (syl, rest) = (s.chars().take(n).collect(), s.chars().skip(n).collect());

               -- char_indices()
               let mid = s.char_indices().nth(5).map_or(s.len(), |(i, _)| i);

               -- chars iterator
               let mut chars = s.chars();
               let tupple: (String, String) = {
               let a = chars.by_ref().take(5).collect();
               let b = chars.collect();
               (a, b)
               };
             */
            let (syl, rest): (String, String) = {
                let mut chars = s.chars();
                let syl: String = chars.by_ref().take(n).collect();
                let rest: String = chars.collect();
                (syl, rest)
            };

            ret.push(syl);
            s = rest;
        }

        ret
    }

    pub fn is_syllable(&self, syl: &String) -> bool {
        self.map.contains_key(syl)
    }

    pub fn convert(&self, ct: ConvType<&String>) -> String {
        self.gojira(ct.preprocess().unwrap())
    }

    pub fn to_kana(&self, s: &String) -> String {
        self.convert(Auto(s))
    }

    pub fn to_hira(&self, s: &String) -> String {
        self.convert(Hira(s))
    }

    pub fn to_kata(&self, s: &String) -> String {
        self.convert(Kata(s))
    }
}
