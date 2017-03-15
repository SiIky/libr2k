use std::collections::HashMap;

pub type Dict = HashMap<String, String>;

//  - [X] Punctuation
const PUNCTUATION_PERIOD: &str = "\u{3002}"; // 。
const PUNCTUATION_COMMA: &str = "\u{3001}"; // 、
const PUNCTUATION_EMARK: &str = "\u{ff01}"; // ！
const PUNCTUATION_QMARK: &str = "\u{ff1f}"; // ？
const PUNCTUATION_DASH: &str = "\u{30fc}"; // ー TODO

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
                              ("a", "\u{3042}"), // あ
                              ("i", "\u{3044}"), // い
                              ("u", "\u{3046}"), // う
                              ("e", "\u{3048}"), // え
                              ("o", "\u{304a}"), // お
                              ("ka", "\u{304b}"), // か
                              ("ga", "\u{304c}"), // が
                              ("ki", "\u{304d}"), // き
                              ("gi", "\u{304e}"), // ぎ
                              ("ku", "\u{304f}"), // く
                              ("gu", "\u{3050}"), // ぐ
                              ("ke", "\u{3051}"), // け
                              ("ge", "\u{3052}"), // げ
                              ("ko", "\u{3053}"), // こ
                              ("go", "\u{3054}"), // ご
                              ("sa", "\u{3055}"), // さ
                              ("za", "\u{3056}"), // ざ
                              ("shi", "\u{3057}"), // し
                              ("ji", "\u{3058}"), // じ
                              ("su", "\u{3059}"), // す
                              ("zu", "\u{305a}"), // ず
                              ("se", "\u{305b}"), // せ
                              ("ze", "\u{305c}"), // ぜ
                              ("so", "\u{305d}"), // そ
                              ("zo", "\u{305e}"), // ぞ
                              ("ta", "\u{305f}"), // た
                              ("da", "\u{3060}"), // だ
                              ("chi", "\u{3061}"), // ち
                              ("dji", "\u{3062}"), // ぢ
                              ("tsu", "\u{3064}"), // つ
                              ("dzu", "\u{3065}"), // づ
                              ("te", "\u{3066}"), // て
                              ("de", "\u{3067}"), // で
                              ("to", "\u{3068}"), // と
                              ("do", "\u{3069}"), // ど
                              ("na", "\u{306a}"), // な
                              ("ni", "\u{306b}"), // に
                              ("nu", "\u{306c}"), // ぬ
                              ("ne", "\u{306d}"), // ね
                              ("no", "\u{306d}"), // の
                              ("ha", "\u{306f}"), // は
                              ("ba", "\u{3070}"), // ば
                              ("pa", "\u{3071}"), // ぱ
                              ("hi", "\u{3072}"), // ひ
                              ("bi", "\u{3073}"), // び
                              ("pi", "\u{3074}"), // ぴ
                              ("fu", "\u{3075}"), // ふ
                              ("bu", "\u{3076}"), // ぶ
                              ("pu", "\u{3077}"), // ぷ
                              ("he", "\u{3078}"), // へ
                              ("be", "\u{3079}"), // べ
                              ("pe", "\u{307a}"), // ぺ
                              ("ho", "\u{307b}"), // ほ
                              ("bo", "\u{307c}"), // ぼ
                              ("po", "\u{307d}"), // ぽ
                              ("ma", "\u{307e}"), // ま
                              ("mi", "\u{307f}"), // み
                              ("mu", "\u{3080}"), // む
                              ("me", "\u{3081}"), // め
                              ("mo", "\u{3082}"), // も
                              ("ya", "\u{3084}"), // や
                              ("yu", "\u{3086}"), // ゆ
                              ("yo", "\u{3088}"), // よ
                              ("ra", "\u{3089}"), // ら
                              ("ri", "\u{308a}"), // り
                              ("ru", "\u{308b}"), // る
                              ("re", "\u{308c}"), // れ
                              ("ro", "\u{308d}"), // ろ
                              ("wa", "\u{308f}"), // わ
                              ("kya", "\u{304d}\u{3083}"), // き
                              ("kyu", "\u{304d}\u{3085}"), // き
                              ("kyo", "\u{304d}\u{3087}"), // き
                              ("gya", "\u{304e}\u{3083}"), //
                              ("gyu", "\u{304e}\u{3085}"), //
                              ("gyo", "\u{304e}\u{3087}"), //
                              ("sha", "\u{3057}\u{3083}"), //
                              ("shu", "\u{3057}\u{3085}"), //
                              ("sho", "\u{3057}\u{3087}"), //
                              ("ja", "\u{3058}\u{3083}"), //
                              ("ju", "\u{3058}\u{3085}"), //
                              ("jo", "\u{3058}\u{3087}"), //
                              ("cha", "\u{3061}\u{3083}"), //
                              ("chu", "\u{3061}\u{3085}"), //
                              ("cho", "\u{3061}\u{3087}"), //
                              ("dja", "\u{3062}\u{3083}"), //
                              ("dju", "\u{3062}\u{3085}"), //
                              ("djo", "\u{3062}\u{3087}"), //
                              ("hya", "\u{3072}\u{3083}"), //
                              ("hyu", "\u{3072}\u{3085}"), //
                              ("hyo", "\u{3072}\u{3087}"), //
                              ("bya", "\u{3073}\u{3083}"), //
                              ("byu", "\u{3073}\u{3085}"), //
                              ("byo", "\u{3073}\u{3087}"), //
                              ("pya", "\u{3074}\u{3083}"), //
                              ("pyu", "\u{3074}\u{3085}"), //
                              ("pyo", "\u{3074}\u{3087}"), //
                              ("nya", "\u{306b}\u{3083}"), //
                              ("nyu", "\u{306b}\u{3085}"), //
                              ("nyo", "\u{306b}\u{3087}"), //
                              ("wi", "\u{3090}"), // ゐ
                              ("we", "\u{3091}"), // ゑ
                              ("wo", "\u{3092}"), // を
                              ("n", "\u{3093}"), // ん
                              ("vu", "\u{3094}"), // ゔ
                              // lazy ass aproach to pauses (small tsu) goes here
                              //
                              // Katakana
                              ("A", "\u{30a2}"), // ア
                              ("I", "\u{30a4}"), // イ
                              ("U", "\u{30a6}"), // ウ
                              ("E", "\u{30a8}"), // エ
                              ("O", "\u{30aa}"), // オ
                              ("KA", "\u{30ab}"), // カ
                              ("GA", "\u{30ac}"), // ガ
                              ("KI", "\u{30ad}"), // キ
                              ("GI", "\u{30ae}"), // ギ
                              ("KU", "\u{30af}"), // ク
                              ("GU", "\u{30b0}"), // グ
                              ("KE", "\u{30b1}"), // ケ
                              ("GE", "\u{30b2}"), // ゲ
                              ("KO", "\u{30b3}"), // コ
                              ("GO", "\u{30b4}"), // ゴ
                              ("SA", "\u{30b5}"), // サ
                              ("ZA", "\u{30b6}"), // ザ
                              ("SHI", "\u{30b7}"), // シ
                              ("JI", "\u{30b8}"), // ジ
                              ("SU", "\u{30b9}"), // ス
                              ("ZU", "\u{30ba}"), // ズ
                              ("SE", "\u{30bb}"), // セ
                              ("ZE", "\u{30bc}"), // ゼ
                              ("SO", "\u{30bd}"), // ソ
                              ("ZO", "\u{30be}"), // ゾ
                              ("TA", "\u{30bf}"), // タ
                              ("DA", "\u{30c0}"), // ダ
                              ("CHI", "\u{30c1}"), // チ
                              ("DJI", "\u{30c2}"), // ヂ
                              ("TSU", "\u{30c4}"), // ツ
                              ("DZU", "\u{30c5}"), // ヅ
                              ("TE", "\u{30c6}"), // テ
                              ("DE", "\u{30c7}"), // デ
                              ("TO", "\u{30c8}"), // ト
                              ("DO", "\u{30c9}"), // ド
                              ("NA", "\u{30ca}"), // ナ
                              ("NI", "\u{30cb}"), // ニ
                              ("NU", "\u{30cc}"), // ヌ
                              ("NE", "\u{30cd}"), // ネ
                              ("NO", "\u{30ce}"), // ノ
                              ("HA", "\u{30cf}"), // ハ
                              ("BA", "\u{30d0}"), // バ
                              ("PA", "\u{30d1}"), // パ
                              ("HI", "\u{30d2}"), // ヒ
                              ("BI", "\u{30d3}"), // ビ
                              ("PI", "\u{30d4}"), // ピ
                              ("FU", "\u{30d5}"), // フ
                              ("BU", "\u{30d6}"), // ブ
                              ("PU", "\u{30d7}"), // プ
                              ("HE", "\u{30d8}"), // ヘ
                              ("BE", "\u{30d9}"), // ベ
                              ("PE", "\u{30da}"), // ペ
                              ("HO", "\u{30db}"), // ホ
                              ("BO", "\u{30dc}"), // ボ
                              ("PO", "\u{30dd}"), // ポ
                              ("MA", "\u{30de}"), // マ
                              ("MI", "\u{30df}"), // ミ
                              ("MU", "\u{30e0}"), // ム
                              ("ME", "\u{30e1}"), // メ
                              ("MO", "\u{30e2}"), // モ
                              ("YA", "\u{30e4}"), // ヤ
                              ("YU", "\u{30e6}"), // ユ
                              ("YO", "\u{30e8}"), // ヨ
                              ("RA", "\u{30e9}"), // ラ
                              ("RI", "\u{30ea}"), // リ
                              ("RU", "\u{30eb}"), // ル
                              ("RE", "\u{30ec}"), // レ
                              ("RO", "\u{30ed}"), // ロ
                              ("WA", "\u{30ef}"), // ワ
                              ("KYA", "\u{30ad}\u{30e3}"), //
                              ("KYU", "\u{30ad}\u{30e5}"), //
                              ("KYO", "\u{30ad}\u{30e7}"), //
                              ("GYA", "\u{30ae}\u{30e3}"), //
                              ("GYU", "\u{30ae}\u{30e5}"), //
                              ("GYO", "\u{30ae}\u{30e7}"), //
                              ("SHA", "\u{30b7}\u{30e3}"), //
                              ("SHU", "\u{30b7}\u{30e5}"), //
                              ("SHO", "\u{30b7}\u{30e7}"), //
                              ("JA", "\u{30b8}\u{30e3}"), //
                              ("JU", "\u{30b8}\u{30e5}"), //
                              ("JO", "\u{30b8}\u{30e7}"), //
                              ("CHA", "\u{30c1}\u{30e3}"), //
                              ("CHU", "\u{30c1}\u{30e5}"), //
                              ("CHO", "\u{30c1}\u{30e7}"), //
                              ("DJA", "\u{30c2}\u{30e3}"), //
                              ("DJU", "\u{30c2}\u{30e5}"), //
                              ("DJO", "\u{30c2}\u{30e7}"), //
                              ("HYA", "\u{30d2}\u{30e3}"), //
                              ("HYU", "\u{30d2}\u{30e5}"), //
                              ("HYO", "\u{30d2}\u{30e7}"), //
                              ("BYA", "\u{30d3}\u{30e3}"), //
                              ("BYU", "\u{30d3}\u{30e5}"), //
                              ("BYO", "\u{30d3}\u{30e7}"), //
                              ("PYA", "\u{30d4}\u{30e3}"), //
                              ("PYU", "\u{30d4}\u{30e5}"), //
                              ("PYO", "\u{30d4}\u{30e7}"), //
                              ("NYA", "\u{30cb}\u{30e3}"), //
                              ("NYU", "\u{30cb}\u{30e5}"), //
                              ("NYO", "\u{30cb}\u{30e7}"), //
                              ("WI", "\u{30f0}"), // ヰ
                              ("WE", "\u{30f1}"), // ヱ
                              ("WO", "\u{30f2}"), // ヲ
                              ("N", "\u{30f3}"), // ン
                              ("VU", "\u{30f4}"), // ヴ
                              // lazy ass aproach to long vowels in Katakana
                              ("AA", "\u{30a2}\u{30fc}"), // アー
                              ("II", "\u{30a4}\u{30fc}"), // イー
                              ("UU", "\u{30a6}\u{30fc}"), // ウー
                              ("EE", "\u{30a8}\u{30fc}"), // エー
                              ("OO", "\u{30aa}\u{30fc}"), // オー
                              // lazy ass aproach to pauses (small tsu) goes here
                              //
                              // Punctuation
                              (".", "\u{3002}"), // 。
                              (",", "\u{3001}"), // 、
                              ("!", "\u{ff01}"), // ！
                              ("?", "\u{ff1f}"), // ？
                              ("-", "\u{30fc}") /* ー */])
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect();
        ret
    }
}
