use std::collections::HashMap;

pub type Dict = HashMap<String, String>;

//  - [X] Punctuation
const PUNCTUATION_PERIOD: &str = "\u{3002}"; // 。
const PUNCTUATION_COMMA: &str = "\u{3001}"; // 、
const PUNCTUATION_EMARK: &str = "\u{ff01}"; // ！
const PUNCTUATION_QMARK: &str = "\u{ff1f}"; // ？
const PUNCTUATION_CHOONPU: &str = "\u{30fc}"; // ー TODO

//  - [X] Hiragana
//      - [X] Without diacritics
//      - [X] With diacritics
const HIRAGANA_A: &str = "\u{3042}"; // あ
const HIRAGANA_SMALL_A: &str = "\u{3042}"; // ぁ
const HIRAGANA_I: &str = "\u{3044}"; // い
const HIRAGANA_SMALL_I: &str = "\u{3044}"; // ぃ
const HIRAGANA_U: &str = "\u{3046}"; // う
const HIRAGANA_SMALL_U: &str = "\u{3046}"; // ぅ
const HIRAGANA_E: &str = "\u{3048}"; // え
const HIRAGANA_SMALL_E: &str = "\u{3048}"; // ぇ
const HIRAGANA_O: &str = "\u{304a}"; // お
const HIRAGANA_SMALL_O: &str = "\u{304a}"; // ぉ
const HIRAGANA_KA: &str = "\u{304b}"; // か
const HIRAGANA_GA: &str = "\u{304c}"; // が
const HIRAGANA_KI: &str = "\u{304d}"; // き
const HIRAGANA_GI: &str = "\u{304e}"; // ぎ
const HIRAGANA_KU: &str = "\u{304f}"; // く
const HIRAGANA_GU: &str = "\u{3050}"; // ぐ
const HIRAGANA_KE: &str = "\u{3051}"; // け
const HIRAGANA_GE: &str = "\u{3052}"; // げ
const HIRAGANA_KO: &str = "\u{3053}"; // こ
const HIRAGANA_GO: &str = "\u{3054}"; // ご
const HIRAGANA_SA: &str = "\u{3055}"; // さ
const HIRAGANA_ZA: &str = "\u{3056}"; // ざ
const HIRAGANA_SHI: &str = "\u{3057}"; // し
const HIRAGANA_JI: &str = "\u{3058}"; // じ
const HIRAGANA_SU: &str = "\u{3059}"; // す
const HIRAGANA_ZU: &str = "\u{305a}"; // ず
const HIRAGANA_SE: &str = "\u{305b}"; // せ
const HIRAGANA_ZE: &str = "\u{305c}"; // ぜ
const HIRAGANA_SO: &str = "\u{305d}"; // そ
const HIRAGANA_ZO: &str = "\u{305e}"; // ぞ
const HIRAGANA_TA: &str = "\u{305f}"; // た
const HIRAGANA_DA: &str = "\u{3060}"; // だ
const HIRAGANA_CHI: &str = "\u{3061}"; // ち
const HIRAGANA_DJI: &str = "\u{3062}"; // ぢ
const HIRAGANA_SMALL_TSU: &str = "\u{3063}"; // っ
const HIRAGANA_TSU: &str = "\u{3064}"; // つ
const HIRAGANA_DZU: &str = "\u{3065}"; // づ
const HIRAGANA_TE: &str = "\u{3066}"; // て
const HIRAGANA_DE: &str = "\u{3067}"; // で
const HIRAGANA_TO: &str = "\u{3068}"; // と
const HIRAGANA_DO: &str = "\u{3069}"; // ど
const HIRAGANA_NA: &str = "\u{306a}"; // な
const HIRAGANA_NI: &str = "\u{306b}"; // に
const HIRAGANA_NU: &str = "\u{306c}"; // ぬ
const HIRAGANA_NE: &str = "\u{306d}"; // ね
const HIRAGANA_NO: &str = "\u{306d}"; // の
const HIRAGANA_HA: &str = "\u{306f}"; // は
const HIRAGANA_BA: &str = "\u{3070}"; // ば
const HIRAGANA_PA: &str = "\u{3071}"; // ぱ
const HIRAGANA_HI: &str = "\u{3072}"; // ひ
const HIRAGANA_BI: &str = "\u{3073}"; // び
const HIRAGANA_PI: &str = "\u{3074}"; // ぴ
const HIRAGANA_FU: &str = "\u{3075}"; // ふ
const HIRAGANA_BU: &str = "\u{3076}"; // ぶ
const HIRAGANA_PU: &str = "\u{3077}"; // ぷ
const HIRAGANA_HE: &str = "\u{3078}"; // へ
const HIRAGANA_BE: &str = "\u{3079}"; // べ
const HIRAGANA_PE: &str = "\u{307a}"; // ぺ
const HIRAGANA_HO: &str = "\u{307b}"; // ほ
const HIRAGANA_BO: &str = "\u{307c}"; // ぼ
const HIRAGANA_PO: &str = "\u{307d}"; // ぽ
const HIRAGANA_MA: &str = "\u{307e}"; // ま
const HIRAGANA_MI: &str = "\u{307f}"; // み
const HIRAGANA_MU: &str = "\u{3080}"; // む
const HIRAGANA_ME: &str = "\u{3081}"; // め
const HIRAGANA_MO: &str = "\u{3082}"; // も
const HIRAGANA_SMALL_YA: &str = "\u{3083}"; // ゃ
const HIRAGANA_YA: &str = "\u{3084}"; // や
const HIRAGANA_SMALL_YO: &str = "\u{3087}"; // ょ
const HIRAGANA_YO: &str = "\u{3088}"; // よ
const HIRAGANA_SMALL_YU: &str = "\u{3085}"; // ゅ
const HIRAGANA_YU: &str = "\u{3086}"; // ゆ
const HIRAGANA_RA: &str = "\u{3089}"; // ら
const HIRAGANA_RI: &str = "\u{308a}"; // り
const HIRAGANA_RU: &str = "\u{308b}"; // る
const HIRAGANA_RE: &str = "\u{308c}"; // れ
const HIRAGANA_RO: &str = "\u{308d}"; // ろ
const HIRAGANA_WA: &str = "\u{308f}"; // わ
const HIRAGANA_WI: &str = "\u{3090}"; // ゐ
const HIRAGANA_WE: &str = "\u{3091}"; // ゑ
const HIRAGANA_WO: &str = "\u{3092}"; // を
const HIRAGANA_N: &str = "\u{3093}"; // ん
const HIRAGANA_VU: &str = "\u{3094}"; // ゔ

//  - [X] Katakana
//      - [X] Without diacritics
//      - [X] With diacritics
const KATAKANA_A : &str = "\u{30a2}"; // ア
const KATAKANA_SMALL_A : &str = "\u{30a1}"; // ァ
const KATAKANA_I : &str = "\u{30a4}"; // イ
const KATAKANA_SMALL_I : &str = "\u{30a3}"; // ィ
const KATAKANA_U : &str = "\u{30a6}"; // ウ
const KATAKANA_SMALL_U : &str = "\u{30a5}"; // ゥ
const KATAKANA_E : &str = "\u{30a8}"; // エ
const KATAKANA_SMALL_E : &str = "\u{30a7}"; // ェ
const KATAKANA_O : &str = "\u{30aa}"; // オ
const KATAKANA_SMALL_O : &str = "\u{30a9}"; // ォ
const KATAKANA_KA : &str = "\u{30ab}"; // カ
const KATAKANA_GA : &str = "\u{30ac}"; // ガ
const KATAKANA_KI : &str = "\u{30ad}"; // キ
const KATAKANA_GI : &str = "\u{30ae}"; // ギ
const KATAKANA_KU : &str = "\u{30af}"; // ク
const KATAKANA_GU : &str = "\u{30b0}"; // グ
const KATAKANA_KE : &str = "\u{30b1}"; // ケ
const KATAKANA_GE : &str = "\u{30b2}"; // ゲ
const KATAKANA_KO : &str = "\u{30b3}"; // コ
const KATAKANA_GO : &str = "\u{30b4}"; // ゴ
const KATAKANA_SA : &str = "\u{30b5}"; // サ
const KATAKANA_ZA : &str = "\u{30b6}"; // ザ
const KATAKANA_SHI : &str = "\u{30b7}"; // シ
const KATAKANA_JI : &str = "\u{30b8}"; // ジ
const KATAKANA_SU : &str = "\u{30b9}"; // ス
const KATAKANA_ZU : &str = "\u{30ba}"; // ズ
const KATAKANA_SE : &str = "\u{30bb}"; // セ
const KATAKANA_ZE : &str = "\u{30bc}"; // ゼ
const KATAKANA_SO : &str = "\u{30bd}"; // ソ
const KATAKANA_ZO : &str = "\u{30be}"; // ゾ
const KATAKANA_TA : &str = "\u{30bf}"; // タ
const KATAKANA_DA : &str = "\u{30c0}"; // ダ
const KATAKANA_CHI : &str = "\u{30c1}"; // チ
const KATAKANA_DJI : &str = "\u{30c2}"; // ヂ
const KATAKANA_SMALL_TSU : &str = "\u{30c3}"; // ッ
const KATAKANA_TSU : &str = "\u{30c4}"; // ツ
const KATAKANA_DZU : &str = "\u{30c5}"; // ヅ
const KATAKANA_TE : &str = "\u{30c6}"; // テ
const KATAKANA_DE : &str = "\u{30c7}"; // デ
const KATAKANA_TO : &str = "\u{30c8}"; // ト
const KATAKANA_DO : &str = "\u{30c9}"; // ド
const KATAKANA_NA : &str = "\u{30ca}"; // ナ
const KATAKANA_NI : &str = "\u{30cb}"; // ニ
const KATAKANA_NU : &str = "\u{30cc}"; // ヌ
const KATAKANA_NE : &str = "\u{30cd}"; // ネ
const KATAKANA_NO : &str = "\u{30ce}"; // ノ
const KATAKANA_HA : &str = "\u{30cf}"; // ハ
const KATAKANA_BA : &str = "\u{30d0}"; // バ
const KATAKANA_PA : &str = "\u{30d1}"; // パ
const KATAKANA_HI : &str = "\u{30d2}"; // ヒ
const KATAKANA_BI : &str = "\u{30d3}"; // ビ
const KATAKANA_PI : &str = "\u{30d4}"; // ピ
const KATAKANA_FU : &str = "\u{30d5}"; // フ
const KATAKANA_BU : &str = "\u{30d6}"; // ブ
const KATAKANA_PU : &str = "\u{30d7}"; // プ
const KATAKANA_HE : &str = "\u{30d8}"; // ヘ
const KATAKANA_BE : &str = "\u{30d9}"; // ベ
const KATAKANA_PE : &str = "\u{30da}"; // ペ
const KATAKANA_HO : &str = "\u{30db}"; // ホ
const KATAKANA_BO : &str = "\u{30dc}"; // ボ
const KATAKANA_PO : &str = "\u{30dd}"; // ポ
const KATAKANA_MA : &str = "\u{30de}"; // マ
const KATAKANA_MI : &str = "\u{30df}"; // ミ
const KATAKANA_MU : &str = "\u{30e0}"; // ム
const KATAKANA_ME : &str = "\u{30e1}"; // メ
const KATAKANA_MO : &str = "\u{30e2}"; // モ
const KATAKANA_SMALL_YA : &str = "\u{30e3}"; // ャ
const KATAKANA_YA : &str = "\u{30e4}"; // ヤ
const KATAKANA_SMALL_YU : &str = "\u{30e5}"; // ュ
const KATAKANA_YU : &str = "\u{30e6}"; // ユ
const KATAKANA_SMALL_YO : &str = "\u{30e7}"; // ョ
const KATAKANA_YO : &str = "\u{30e8}"; // ヨ
const KATAKANA_RA : &str = "\u{30e9}"; // ラ
const KATAKANA_RI : &str = "\u{30ea}"; // リ
const KATAKANA_RU : &str = "\u{30eb}"; // ル
const KATAKANA_RE : &str = "\u{30ec}"; // レ
const KATAKANA_RO : &str = "\u{30ed}"; // ロ
const KATAKANA_WA : &str = "\u{30ef}"; // ワ
const KATAKANA_WI : &str = "\u{30f0}"; // ヰ
const KATAKANA_WE : &str = "\u{30f1}"; // ヱ
const KATAKANA_WO : &str = "\u{30f2}"; // ヲ
const KATAKANA_N : &str = "\u{30f3}"; // ン
const KATAKANA_VU : &str = "\u{30f4}"; // ヴ

pub trait KanaConversionTable {
    fn dnew() -> Dict;
}

impl KanaConversionTable for Dict {
    fn dnew() -> Dict {
        let ret: Dict = (vec![// Hiragana
                              ("a", format!("{}", HIRAGANA_A)), // あ
                              ("i", format!("{}", HIRAGANA_I)), // い
                              ("u", format!("{}", HIRAGANA_U)), // う
                              ("e", format!("{}", HIRAGANA_E)), // え
                              ("o", format!("{}", HIRAGANA_O)), // お
                              ("ka", format!("{}", HIRAGANA_KA)), // か
                              ("ga", format!("{}", HIRAGANA_GA)), // が
                              ("ki", format!("{}", HIRAGANA_KI)), // き
                              ("gi", format!("{}", HIRAGANA_GI)), // ぎ
                              ("ku", format!("{}", HIRAGANA_KU)), // く
                              ("gu", format!("{}", HIRAGANA_GU)), // ぐ
                              ("ke", format!("{}", HIRAGANA_KE)), // け
                              ("ge", format!("{}", HIRAGANA_GE)), // げ
                              ("ko", format!("{}", HIRAGANA_KO)), // こ
                              ("go", format!("{}", HIRAGANA_GO)), // ご
                              ("sa", format!("{}", HIRAGANA_SA)), // さ
                              ("za", format!("{}", HIRAGANA_ZA)), // ざ
                              ("shi", format!("{}", HIRAGANA_SHI)), // し
                              ("ji", format!("{}", HIRAGANA_JI)), // じ
                              ("su", format!("{}", HIRAGANA_SU)), // す
                              ("zu", format!("{}", HIRAGANA_ZU)), // ず
                              ("se", format!("{}", HIRAGANA_SE)), // せ
                              ("ze", format!("{}", HIRAGANA_ZE)), // ぜ
                              ("so", format!("{}", HIRAGANA_SO)), // そ
                              ("zo", format!("{}", HIRAGANA_ZO)), // ぞ
                              ("ta", format!("{}", HIRAGANA_TA)), // た
                              ("da", format!("{}", HIRAGANA_DA)), // だ
                              ("chi", format!("{}", HIRAGANA_CHI)), // ち
                              ("dji", format!("{}", HIRAGANA_DJI)), // ぢ
                              ("tsu", format!("{}", HIRAGANA_TSU)), // つ
                              ("dzu", format!("{}", HIRAGANA_DZU)), // づ
                              ("te", format!("{}", HIRAGANA_TE)), // て
                              ("de", format!("{}", HIRAGANA_DE)), // で
                              ("to", format!("{}", HIRAGANA_TO)), // と
                              ("do", format!("{}", HIRAGANA_DO)), // ど
                              ("na", format!("{}", HIRAGANA_NA)), // な
                              ("ni", format!("{}", HIRAGANA_NI)), // に
                              ("nu", format!("{}", HIRAGANA_NU)), // ぬ
                              ("ne", format!("{}", HIRAGANA_NE)), // ね
                              ("no", format!("{}", HIRAGANA_NO)), // の
                              ("ha", format!("{}", HIRAGANA_HA)), // は
                              ("ba", format!("{}", HIRAGANA_BA)), // ば
                              ("pa", format!("{}", HIRAGANA_PA)), // ぱ
                              ("hi", format!("{}", HIRAGANA_HI)), // ひ
                              ("bi", format!("{}", HIRAGANA_BI)), // び
                              ("pi", format!("{}", HIRAGANA_PI)), // ぴ
                              ("fu", format!("{}", HIRAGANA_FU)), // ふ
                              ("bu", format!("{}", HIRAGANA_BU)), // ぶ
                              ("pu", format!("{}", HIRAGANA_PU)), // ぷ
                              ("he", format!("{}", HIRAGANA_HE)), // へ
                              ("be", format!("{}", HIRAGANA_BE)), // べ
                              ("pe", format!("{}", HIRAGANA_PE)), // ぺ
                              ("ho", format!("{}", HIRAGANA_HO)), // ほ
                              ("bo", format!("{}", HIRAGANA_BO)), // ぼ
                              ("po", format!("{}", HIRAGANA_PO)), // ぽ
                              ("ma", format!("{}", HIRAGANA_MA)), // ま
                              ("mi", format!("{}", HIRAGANA_MI)), // み
                              ("mu", format!("{}", HIRAGANA_MU)), // む
                              ("me", format!("{}", HIRAGANA_ME)), // め
                              ("mo", format!("{}", HIRAGANA_MO)), // も
                              ("ya", format!("{}", HIRAGANA_YA)), // や
                              ("yu", format!("{}", HIRAGANA_YU)), // ゆ
                              ("yo", format!("{}", HIRAGANA_YO)), // よ
                              ("ra", format!("{}", HIRAGANA_RA)), // ら
                              ("ri", format!("{}", HIRAGANA_RI)), // り
                              ("ru", format!("{}", HIRAGANA_RU)), // る
                              ("re", format!("{}", HIRAGANA_RE)), // れ
                              ("ro", format!("{}", HIRAGANA_RO)), // ろ
                              ("wa", format!("{}", HIRAGANA_WA)), // わ
                              ("kya", format!("{}", HIRAGANA_A)), // きゃ
                              ("kyu", format!("{}", HIRAGANA_A)), // きゅ
                              ("kyo", format!("{}", HIRAGANA_A)), // きょ
                              ("gya", format!("{}", HIRAGANA_A)), // ぎゃ
                              ("gyu", format!("{}", HIRAGANA_A)), // ぎゅ
                              ("gyo", format!("{}", HIRAGANA_A)), // ぎょ
                              ("sha", format!("{}", HIRAGANA_A)), // しゃ
                              ("shu", format!("{}", HIRAGANA_A)), // しゅ
                              ("sho", format!("{}", HIRAGANA_A)), // しょ
                              ("ja", format!("{}", HIRAGANA_A)), // じゃ
                              ("ju", format!("{}", HIRAGANA_A)), // じゅ
                              ("jo", format!("{}", HIRAGANA_A)), // じょ
                              ("cha", format!("{}", HIRAGANA_A)), // ちゃ
                              ("chu", format!("{}", HIRAGANA_A)), // ちゅ
                              ("cho", format!("{}", HIRAGANA_A)), // ちょ
                              ("dja", format!("{}", HIRAGANA_A)), // ぢゃ
                              ("dju", format!("{}", HIRAGANA_A)), // ぢゅ
                              ("djo", format!("{}", HIRAGANA_A)), // ぢょ
                              ("hya", format!("{}", HIRAGANA_A)), // ひゃ
                              ("hyu", format!("{}", HIRAGANA_A)), // ひゅ
                              ("hyo", format!("{}", HIRAGANA_A)), // ひょ
                              ("bya", format!("{}", HIRAGANA_A)), // びゃ
                              ("byu", format!("{}", HIRAGANA_A)), // びゅ
                              ("byo", format!("{}", HIRAGANA_A)), // びょ
                              ("pya", format!("{}", HIRAGANA_A)), // ぴゃ
                              ("pyu", format!("{}", HIRAGANA_A)), // ぴゅ
                              ("pyo", format!("{}", HIRAGANA_A)), // ぴょ
                              ("nya", format!("{}", HIRAGANA_A)), // にゃ
                              ("nyu", format!("{}", HIRAGANA_A)), // にゅ
                              ("nyo", format!("{}", HIRAGANA_A)), // にょ
                              ("wi", format!("{}", HIRAGANA_WI)), // ゐ
                              ("we", format!("{}", HIRAGANA_WE)), // ゑ
                              ("wo", format!("{}", HIRAGANA_WO)), // を
                              ("n", format!("{}", HIRAGANA_N)), // ん
                              ("vu", format!("{}", HIRAGANA_VU)), // ゔ
                              // lazy ass aproach to pauses (small tsu) goes here
                              //
                              // Katakana
                              ("A", format!("{}", KATAKANA_A)), // ア
                              ("I", format!("{}", KATAKANA_I)), // イ
                              ("U", format!("{}", KATAKANA_U)), // ウ
                              ("E", format!("{}", KATAKANA_E)), // エ
                              ("O", format!("{}", KATAKANA_O)), // オ
                              ("KA", format!("{}", KATAKANA_KA)), // カ
                              ("GA", format!("{}", KATAKANA_GA)), // ガ
                              ("KI", format!("{}", KATAKANA_KI)), // キ
                              ("GI", format!("{}", KATAKANA_GI)), // ギ
                              ("KU", format!("{}", KATAKANA_KU)), // ク
                              ("GU", format!("{}", KATAKANA_GU)), // グ
                              ("KE", format!("{}", KATAKANA_KE)), // ケ
                              ("GE", format!("{}", KATAKANA_GE)), // ゲ
                              ("KO", format!("{}", KATAKANA_KO)), // コ
                              ("GO", format!("{}", KATAKANA_GO)), // ゴ
                              ("SA", format!("{}", KATAKANA_SA)), // サ
                              ("ZA", format!("{}", KATAKANA_ZA)), // ザ
                              ("SHI", format!("{}", KATAKANA_SHI)), // シ
                              ("JI", format!("{}", KATAKANA_JI)), // ジ
                              ("SU", format!("{}", KATAKANA_SU)), // ス
                              ("ZU", format!("{}", KATAKANA_ZU)), // ズ
                              ("SE", format!("{}", KATAKANA_SE)), // セ
                              ("ZE", format!("{}", KATAKANA_ZE)), // ゼ
                              ("SO", format!("{}", KATAKANA_SO)), // ソ
                              ("ZO", format!("{}", KATAKANA_ZO)), // ゾ
                              ("TA", format!("{}", KATAKANA_TA)), // タ
                              ("DA", format!("{}", KATAKANA_DA)), // ダ
                              ("CHI", format!("{}", KATAKANA_CHI)), // チ
                              ("DJI", format!("{}", KATAKANA_DJI)), // ヂ
                              ("TSU", format!("{}", KATAKANA_TSU)), // ツ
                              ("DZU", format!("{}", KATAKANA_DZU)), // ヅ
                              ("TE", format!("{}", KATAKANA_TE)), // テ
                              ("DE", format!("{}", KATAKANA_DE)), // デ
                              ("TO", format!("{}", KATAKANA_TO)), // ト
                              ("DO", format!("{}", KATAKANA_DO)), // ド
                              ("NA", format!("{}", KATAKANA_NA)), // ナ
                              ("NI", format!("{}", KATAKANA_NI)), // ニ
                              ("NU", format!("{}", KATAKANA_NU)), // ヌ
                              ("NE", format!("{}", KATAKANA_NE)), // ネ
                              ("NO", format!("{}", KATAKANA_NO)), // ノ
                              ("HA", format!("{}", KATAKANA_HA)), // ハ
                              ("BA", format!("{}", KATAKANA_BA)), // バ
                              ("PA", format!("{}", KATAKANA_PA)), // パ
                              ("HI", format!("{}", KATAKANA_HI)), // ヒ
                              ("BI", format!("{}", KATAKANA_BI)), // ビ
                              ("PI", format!("{}", KATAKANA_PI)), // ピ
                              ("FU", format!("{}", KATAKANA_FU)), // フ
                              ("BU", format!("{}", KATAKANA_BU)), // ブ
                              ("PU", format!("{}", KATAKANA_PU)), // プ
                              ("HE", format!("{}", KATAKANA_HE)), // ヘ
                              ("BE", format!("{}", KATAKANA_BE)), // ベ
                              ("PE", format!("{}", KATAKANA_PE)), // ペ
                              ("HO", format!("{}", KATAKANA_HO)), // ホ
                              ("BO", format!("{}", KATAKANA_BO)), // ボ
                              ("PO", format!("{}", KATAKANA_PO)), // ポ
                              ("MA", format!("{}", KATAKANA_MA)), // マ
                              ("MI", format!("{}", KATAKANA_MI)), // ミ
                              ("MU", format!("{}", KATAKANA_MU)), // ム
                              ("ME", format!("{}", KATAKANA_ME)), // メ
                              ("MO", format!("{}", KATAKANA_MO)), // モ
                              ("YA", format!("{}", KATAKANA_YA)), // ヤ
                              ("YU", format!("{}", KATAKANA_YU)), // ユ
                              ("YO", format!("{}", KATAKANA_YO)), // ヨ
                              ("RA", format!("{}", KATAKANA_RA)), // ラ
                              ("RI", format!("{}", KATAKANA_RI)), // リ
                              ("RU", format!("{}", KATAKANA_RU)), // ル
                              ("RE", format!("{}", KATAKANA_RE)), // レ
                              ("RO", format!("{}", KATAKANA_RO)), // ロ
                              ("WA", format!("{}", KATAKANA_WA)), // ワ
                              ("KYA", format!("{}", KATAKANA_A)), // きゃ
                              ("KYU", format!("{}", KATAKANA_A)), // きゅ
                              ("KYO", format!("{}", KATAKANA_A)), // きょ
                              ("GYA", format!("{}", KATAKANA_A)), // ぎゃ
                              ("GYU", format!("{}", KATAKANA_A)), // ぎゅ
                              ("GYO", format!("{}", KATAKANA_A)), // ぎょ
                              ("SHA", format!("{}", KATAKANA_A)), // しゃ
                              ("SHU", format!("{}", KATAKANA_A)), // しゅ
                              ("SHO", format!("{}", KATAKANA_A)), // しょ
                              ("JA", format!("{}", KATAKANA_A)), // じゃ
                              ("JU", format!("{}", KATAKANA_A)), // じゅ
                              ("JO", format!("{}", KATAKANA_A)), // じょ
                              ("CHA", format!("{}", KATAKANA_A)), // ちゃ
                              ("CHU", format!("{}", KATAKANA_A)), // ちゅ
                              ("CHO", format!("{}", KATAKANA_A)), // ちょ
                              ("DJA", format!("{}", KATAKANA_A)), // ぢゃ
                              ("DJU", format!("{}", KATAKANA_A)), // ぢゅ
                              ("DJO", format!("{}", KATAKANA_A)), // ぢょ
                              ("HYA", format!("{}", KATAKANA_A)), // ひゃ
                              ("HYU", format!("{}", KATAKANA_A)), // ひゅ
                              ("HYO", format!("{}", KATAKANA_A)), // ひょ
                              ("BYA", format!("{}", KATAKANA_A)), // びゃ
                              ("BYU", format!("{}", KATAKANA_A)), // びゅ
                              ("BYO", format!("{}", KATAKANA_A)), // びょ
                              ("PYA", format!("{}", KATAKANA_A)), // ぴゃ
                              ("PYU", format!("{}", KATAKANA_A)), // ぴゅ
                              ("PYO", format!("{}", KATAKANA_A)), // ぴょ
                              ("NYA", format!("{}", KATAKANA_A)), // にゃ
                              ("NYU", format!("{}", KATAKANA_A)), // にゅ
                              ("NYO", format!("{}", KATAKANA_A)), // にょ
                              ("WI", format!("{}", KATAKANA_WI)), // ヰ
                              ("WE", format!("{}", KATAKANA_WE)), // ヱ
                              ("WO", format!("{}", KATAKANA_WO)), // ヲ
                              ("N", format!("{}", KATAKANA_N)), // ン
                              ("VU", format!("{}", KATAKANA_VU)), // ヴ
                              // lazy ass aproach to long vowels in Katakana
                              ("AA", format!("{}{}", KATAKANA_A, PUNCTUATION_CHOONPU)), // アー
                              ("II", format!("{}{}", KATAKANA_I, PUNCTUATION_CHOONPU)), // イー
                              ("UU", format!("{}{}", KATAKANA_U, PUNCTUATION_CHOONPU)), // ウー
                              ("EE", format!("{}{}", KATAKANA_E, PUNCTUATION_CHOONPU)), // エー
                              ("OO", format!("{}{}", KATAKANA_O, PUNCTUATION_CHOONPU)), // オー
                              // lazy ass aproach to pauses (small tsu) goes here
                              //
                              // Punctuation
                              (".", format!("{}", PUNCTUATION_PERIOD)), // 。
                              (",", format!("{}", PUNCTUATION_COMMA)), // 、
                              ("!", format!("{}", PUNCTUATION_EMARK)), // ！
                              ("?", format!("{}", PUNCTUATION_QMARK)), // ？
                              ("-", format!("{}", PUNCTUATION_CHOONPU)) /* ー */])
                .into_iter()
                .map(|(k, v)| (k.to_string(), v))
                .collect();
        ret
    }
}
