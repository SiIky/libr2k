use std::io::{stdin, BufRead};

#[macro_use]
extern crate clap;
use clap::{App, Arg, ArgMatches};

extern crate r2k;
use r2k::kana_table::KanaTable;
use r2k::conv_type::ConvType;

fn main() {
    fn choose_conv_type(m: &ArgMatches) -> ConvType<()> {
        match (
            m.is_present("hiragana"),
            m.is_present("katakana"),
        ) {
            (true, _) => ConvType::Hira(()),
            (_, true) => ConvType::Kata(()),
            _ => ConvType::Auto(()),
        }
    };

    let kana: KanaTable = KanaTable::new();
    let matches: ArgMatches = clap();
    let ct = choose_conv_type(&matches);

    let convert2str = |txt| {
        let ct = ct.map(|_| &txt);
        let res = kana.convert(ct);
        format!("{}", res)
    };

    if matches.is_present("TEXT") {
        let v: Vec<String> = matches
            .values_of("TEXT")
            .unwrap()
            .into_iter()
            .map(|x| x.to_string())
            .collect();

        for s in v {
            print!("{}", convert2str(s));
        }

        println!();
    } else {
        let stdin = stdin();

        for line in stdin.lock().lines() {
            match line {
                Ok(line) => print!("{}\n", convert2str(line)),
                Err(e) => println!("{}", e),
            }
        }
    }
}

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
fn clap() -> ArgMatches<'static> {
    App::new("Japanese Command-line Dictionary")
        .author(crate_authors!())
        .version(crate_version!())
        .about("Romaji to Kana converter")
        .help_short("H")
        .args(&[
            Arg::with_name("hiragana")
                .conflicts_with("katakana")
                .help("Convert romaji to hiragana.")
                .long("hiragana")
                .short("h"),
            Arg::with_name("katakana")
                .conflicts_with("hiragana")
                .help("Convert romaji to katakana.")
                .long("katakana")
                .short("k"),
            Arg::with_name("TEXT").multiple(true),
        ])
        .get_matches()
}
