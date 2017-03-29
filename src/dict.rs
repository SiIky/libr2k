#![allow(dead_code)] // Temporary
use std::collections::HashMap;

pub type Dict = HashMap<String, String>;

//  - [X] Punctuation
const PUNCT_PERIOD: &str = "\u{3002}"; // 。
const PUNCT_COMMA: &str = "\u{3001}"; // 、
const PUNCT_EMARK: &str = "\u{ff01}"; // ！
const PUNCT_QMARK: &str = "\u{ff1f}"; // ？
const PUNCT_CHOONPU: &str = "\u{30fc}"; // ー

//  - [X] Hiragana
//      - [X] Without diacritics
//      - [X] With diacritics
const HIRA_A: &str = "\u{3042}"; // あ
const HIRA_SMALL_A: &str = "\u{3042}"; // ぁ
const HIRA_I: &str = "\u{3044}"; // い
const HIRA_SMALL_I: &str = "\u{3044}"; // ぃ
const HIRA_U: &str = "\u{3046}"; // う
const HIRA_SMALL_U: &str = "\u{3046}"; // ぅ
const HIRA_E: &str = "\u{3048}"; // え
const HIRA_SMALL_E: &str = "\u{3048}"; // ぇ
const HIRA_O: &str = "\u{304a}"; // お
const HIRA_SMALL_O: &str = "\u{304a}"; // ぉ
const HIRA_KA: &str = "\u{304b}"; // か
const HIRA_GA: &str = "\u{304c}"; // が
const HIRA_KI: &str = "\u{304d}"; // き
const HIRA_GI: &str = "\u{304e}"; // ぎ
const HIRA_KU: &str = "\u{304f}"; // く
const HIRA_GU: &str = "\u{3050}"; // ぐ
const HIRA_KE: &str = "\u{3051}"; // け
const HIRA_GE: &str = "\u{3052}"; // げ
const HIRA_KO: &str = "\u{3053}"; // こ
const HIRA_GO: &str = "\u{3054}"; // ご
const HIRA_SA: &str = "\u{3055}"; // さ
const HIRA_ZA: &str = "\u{3056}"; // ざ
const HIRA_SHI: &str = "\u{3057}"; // し
const HIRA_JI: &str = "\u{3058}"; // じ
const HIRA_SU: &str = "\u{3059}"; // す
const HIRA_ZU: &str = "\u{305a}"; // ず
const HIRA_SE: &str = "\u{305b}"; // せ
const HIRA_ZE: &str = "\u{305c}"; // ぜ
const HIRA_SO: &str = "\u{305d}"; // そ
const HIRA_ZO: &str = "\u{305e}"; // ぞ
const HIRA_TA: &str = "\u{305f}"; // た
const HIRA_DA: &str = "\u{3060}"; // だ
const HIRA_CHI: &str = "\u{3061}"; // ち
const HIRA_DJI: &str = "\u{3062}"; // ぢ
const HIRA_SMALL_TSU: &str = "\u{3063}"; // っ
const HIRA_TSU: &str = "\u{3064}"; // つ
const HIRA_DZU: &str = "\u{3065}"; // づ
const HIRA_TE: &str = "\u{3066}"; // て
const HIRA_DE: &str = "\u{3067}"; // で
const HIRA_TO: &str = "\u{3068}"; // と
const HIRA_DO: &str = "\u{3069}"; // ど
const HIRA_NA: &str = "\u{306a}"; // な
const HIRA_NI: &str = "\u{306b}"; // に
const HIRA_NU: &str = "\u{306c}"; // ぬ
const HIRA_NE: &str = "\u{306d}"; // ね
const HIRA_NO: &str = "\u{306d}"; // の
const HIRA_HA: &str = "\u{306f}"; // は
const HIRA_BA: &str = "\u{3070}"; // ば
const HIRA_PA: &str = "\u{3071}"; // ぱ
const HIRA_HI: &str = "\u{3072}"; // ひ
const HIRA_BI: &str = "\u{3073}"; // び
const HIRA_PI: &str = "\u{3074}"; // ぴ
const HIRA_FU: &str = "\u{3075}"; // ふ
const HIRA_BU: &str = "\u{3076}"; // ぶ
const HIRA_PU: &str = "\u{3077}"; // ぷ
const HIRA_HE: &str = "\u{3078}"; // へ
const HIRA_BE: &str = "\u{3079}"; // べ
const HIRA_PE: &str = "\u{307a}"; // ぺ
const HIRA_HO: &str = "\u{307b}"; // ほ
const HIRA_BO: &str = "\u{307c}"; // ぼ
const HIRA_PO: &str = "\u{307d}"; // ぽ
const HIRA_MA: &str = "\u{307e}"; // ま
const HIRA_MI: &str = "\u{307f}"; // み
const HIRA_MU: &str = "\u{3080}"; // む
const HIRA_ME: &str = "\u{3081}"; // め
const HIRA_MO: &str = "\u{3082}"; // も
const HIRA_SMALL_YA: &str = "\u{3083}"; // ゃ
const HIRA_YA: &str = "\u{3084}"; // や
const HIRA_SMALL_YO: &str = "\u{3087}"; // ょ
const HIRA_YO: &str = "\u{3088}"; // よ
const HIRA_SMALL_YU: &str = "\u{3085}"; // ゅ
const HIRA_YU: &str = "\u{3086}"; // ゆ
const HIRA_RA: &str = "\u{3089}"; // ら
const HIRA_RI: &str = "\u{308a}"; // り
const HIRA_RU: &str = "\u{308b}"; // る
const HIRA_RE: &str = "\u{308c}"; // れ
const HIRA_RO: &str = "\u{308d}"; // ろ
const HIRA_WA: &str = "\u{308f}"; // わ
const HIRA_WI: &str = "\u{3090}"; // ゐ
const HIRA_WE: &str = "\u{3091}"; // ゑ
const HIRA_WO: &str = "\u{3092}"; // を
const HIRA_N: &str = "\u{3093}"; // ん
const HIRA_VU: &str = "\u{3094}"; // ゔ

//  - [X] Katakana
//      - [X] Without diacritics
//      - [X] With diacritics
const KATA_A: &str = "\u{30a2}"; // ア
const KATA_SMALL_A: &str = "\u{30a1}"; // ァ
const KATA_I: &str = "\u{30a4}"; // イ
const KATA_SMALL_I: &str = "\u{30a3}"; // ィ
const KATA_U: &str = "\u{30a6}"; // ウ
const KATA_SMALL_U: &str = "\u{30a5}"; // ゥ
const KATA_E: &str = "\u{30a8}"; // エ
const KATA_SMALL_E: &str = "\u{30a7}"; // ェ
const KATA_O: &str = "\u{30aa}"; // オ
const KATA_SMALL_O: &str = "\u{30a9}"; // ォ
const KATA_KA: &str = "\u{30ab}"; // カ
const KATA_GA: &str = "\u{30ac}"; // ガ
const KATA_KI: &str = "\u{30ad}"; // キ
const KATA_GI: &str = "\u{30ae}"; // ギ
const KATA_KU: &str = "\u{30af}"; // ク
const KATA_GU: &str = "\u{30b0}"; // グ
const KATA_KE: &str = "\u{30b1}"; // ケ
const KATA_GE: &str = "\u{30b2}"; // ゲ
const KATA_KO: &str = "\u{30b3}"; // コ
const KATA_GO: &str = "\u{30b4}"; // ゴ
const KATA_SA: &str = "\u{30b5}"; // サ
const KATA_ZA: &str = "\u{30b6}"; // ザ
const KATA_SHI: &str = "\u{30b7}"; // シ
const KATA_JI: &str = "\u{30b8}"; // ジ
const KATA_SU: &str = "\u{30b9}"; // ス
const KATA_ZU: &str = "\u{30ba}"; // ズ
const KATA_SE: &str = "\u{30bb}"; // セ
const KATA_ZE: &str = "\u{30bc}"; // ゼ
const KATA_SO: &str = "\u{30bd}"; // ソ
const KATA_ZO: &str = "\u{30be}"; // ゾ
const KATA_TA: &str = "\u{30bf}"; // タ
const KATA_DA: &str = "\u{30c0}"; // ダ
const KATA_CHI: &str = "\u{30c1}"; // チ
const KATA_DJI: &str = "\u{30c2}"; // ヂ
const KATA_SMALL_TSU: &str = "\u{30c3}"; // ッ
const KATA_TSU: &str = "\u{30c4}"; // ツ
const KATA_DZU: &str = "\u{30c5}"; // ヅ
const KATA_TE: &str = "\u{30c6}"; // テ
const KATA_DE: &str = "\u{30c7}"; // デ
const KATA_TO: &str = "\u{30c8}"; // ト
const KATA_DO: &str = "\u{30c9}"; // ド
const KATA_NA: &str = "\u{30ca}"; // ナ
const KATA_NI: &str = "\u{30cb}"; // ニ
const KATA_NU: &str = "\u{30cc}"; // ヌ
const KATA_NE: &str = "\u{30cd}"; // ネ
const KATA_NO: &str = "\u{30ce}"; // ノ
const KATA_HA: &str = "\u{30cf}"; // ハ
const KATA_BA: &str = "\u{30d0}"; // バ
const KATA_PA: &str = "\u{30d1}"; // パ
const KATA_HI: &str = "\u{30d2}"; // ヒ
const KATA_BI: &str = "\u{30d3}"; // ビ
const KATA_PI: &str = "\u{30d4}"; // ピ
const KATA_FU: &str = "\u{30d5}"; // フ
const KATA_BU: &str = "\u{30d6}"; // ブ
const KATA_PU: &str = "\u{30d7}"; // プ
const KATA_HE: &str = "\u{30d8}"; // ヘ
const KATA_BE: &str = "\u{30d9}"; // ベ
const KATA_PE: &str = "\u{30da}"; // ペ
const KATA_HO: &str = "\u{30db}"; // ホ
const KATA_BO: &str = "\u{30dc}"; // ボ
const KATA_PO: &str = "\u{30dd}"; // ポ
const KATA_MA: &str = "\u{30de}"; // マ
const KATA_MI: &str = "\u{30df}"; // ミ
const KATA_MU: &str = "\u{30e0}"; // ム
const KATA_ME: &str = "\u{30e1}"; // メ
const KATA_MO: &str = "\u{30e2}"; // モ
const KATA_SMALL_YA: &str = "\u{30e3}"; // ャ
const KATA_YA: &str = "\u{30e4}"; // ヤ
const KATA_SMALL_YU: &str = "\u{30e5}"; // ュ
const KATA_YU: &str = "\u{30e6}"; // ユ
const KATA_SMALL_YO: &str = "\u{30e7}"; // ョ
const KATA_YO: &str = "\u{30e8}"; // ヨ
const KATA_RA: &str = "\u{30e9}"; // ラ
const KATA_RI: &str = "\u{30ea}"; // リ
const KATA_RU: &str = "\u{30eb}"; // ル
const KATA_RE: &str = "\u{30ec}"; // レ
const KATA_RO: &str = "\u{30ed}"; // ロ
const KATA_WA: &str = "\u{30ef}"; // ワ
const KATA_WI: &str = "\u{30f0}"; // ヰ
const KATA_WE: &str = "\u{30f1}"; // ヱ
const KATA_WO: &str = "\u{30f2}"; // ヲ
const KATA_N: &str = "\u{30f3}"; // ン
const KATA_VU: &str = "\u{30f4}"; // ヴ

pub trait KanaConversionTable {
    fn dnew() -> Dict;
}

impl KanaConversionTable for Dict {
    fn dnew() -> Dict {
        let ret: Dict =
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
                 ("-", format!("{}", PUNCT_CHOONPU)) /* ー */]
                    .into_iter()
                    .map(|(k, v)| (k.to_string(), v))
                    .collect();
        ret
    }
}
