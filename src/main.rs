#[macro_use]
extern crate clap;
use clap::{App, Arg, ArgMatches, Values};

extern crate r2k;
use r2k::kana_table::KanaTable;

fn main() {
    fn aux(vals: Values) -> Vec<String> {
        vals.into_iter().map(|x| x.to_string()).collect()
    }

    let kana: KanaTable = KanaTable::new();
    let matches: ArgMatches = clap();

    let do_work = |s| {
        let tmp: Option<Values> = matches.values_of(s);
        let tmp: Option<Vec<String>> = tmp.map(|x| aux(x));
        let tmp: Option<String> = tmp.map(|x| x.iter().map(|v| kana.to_kana(&v)).collect());
        if let Some(v) = tmp {
            println!("{}", v);
        };
    };

    do_work("romaji");
    do_work("hiragana");
    do_work("katakana");
}

///
/// Usage: (This comment will be used to describe the
/// expected behavior and the program must fit this
/// description, not the other way around)
///
/// - [X] `-r`: Autodetect and convert words according to case;
/// - [X] `-h`: Don't autodetect, convert everything to hiragana;
/// - [X] `-k`: Don't autodetect, convert everything to katakana;
///     NOTE: At least one of these must be used. If more than one is used:
///         - [X] **Process every option;** (Current behavior, makes more sense out of the two)
///         - [ ] ~~Check options in order and process only the first one;~~
///
fn clap() -> ArgMatches<'static> {
    App::new("Japanese Command-line Dictionary")
        .author(crate_authors!())
        .version(crate_version!())
        .about("Romaji to Kana converter")
        .help_short("H")
        .args(&[Arg::with_name("romaji")
                    .long("romaji")
                    .short("r")
                    .takes_value(true)
                    .multiple(true)
                    .help("Convert romaji to kana."),
                Arg::with_name("hiragana")
                    .long("hiragana")
                    .short("h")
                    .takes_value(true)
                    .multiple(true)
                    .help("Convert romaji to hiragana."),
                Arg::with_name("katakana")
                    .long("katakana")
                    .short("k")
                    .takes_value(true)
                    .multiple(true)
                    .help("Convert romaji to katakana.")])
        .get_matches()
}
