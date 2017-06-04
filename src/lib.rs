//! # Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! r2k = { version = "*", git = "https://github.com/siiky/r2k" }
//! ```
//!
//! and this to your crate root:
//!
//! ```
//! extern crate r2k;
//! ```
//!
//! # Example
//!
//! ```
//! use r2k::kana_table::KanaTable;
//!
//! let kt: KanaTable = KanaTable::new();
//! let input: String = "jagAimo".to_string();
//!
//! let auto: String = kt.to_kana(&input);
//! let hira: String = kt.to_hira(&input);
//! let kata: String = kt.to_kata(&input);
//!
//! //assert_eq!(auto, "ジャガイモ");
//! assert_eq!(hira, "じゃがいも");
//! //assert_eq!(kata, "ジャガイモ");
//! ```
pub mod conv_type;
pub mod kana_table;
pub mod normalize;
pub mod preprocess;
