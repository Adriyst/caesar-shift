use std::collections::HashMap;
use std::fs;

use clap::{Arg, ArgAction, Command};
use whatlang::detect;

const DEFAULT_ALPHABET: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZÆØÅabcdefghijklmnopqrstuvwxyzæøå";

fn caesar_cipher(text: &str, shift: i32, decrypt: bool, alphabet: &str) -> String {
    let shift = if decrypt { -shift } else { shift };
    let alphabet_chars: Vec<char> = alphabet.chars().collect();
    let alphabet_chars_map: HashMap<char, usize> = alphabet
        .chars()
        .enumerate()
        .map(|(idx, c)| (c, idx))
        .collect();
    let len = alphabet_chars.len() as i32;
    text.chars()
        .map(|c| {
            alphabet_chars_map
                .get(&c)
                .map(|pos| (*pos as i32 + shift).rem_euclid(len))
                .map(|new_pos| {
                    alphabet_chars
                        .get(new_pos as usize)
                        .map(|new_c| new_c.to_owned())
                        .unwrap_or(c.clone())
                })
                .unwrap_or(c.clone())
        })
        .collect()
}

fn get_alphabet(input: &String) -> String {
    String::from(match input.as_str() {
        "eng" => "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz",
        "rus" => "АБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯабвгдеёжзийклмнопрстуфхцчшщъыьэюя",
        "deu" => "ABCDEFGHIJKLMNOPQRSTUVWXYZÄÖÜabcdefghijklmnopqrstuvwxyzäöüß",
        _ => DEFAULT_ALPHABET, // Default to Norwegian
    })
}

fn main() {
    let matches = Command::new("caesar_cipher")
        .arg(
            Arg::new("shift")
                .required(true)
                .value_parser(clap::value_parser!(i32))
                .help("How many characters to shift"),
        )
        .arg(
            Arg::new("filename")
                .required(true)
                .help("Name of the file that is to be processed"),
        )
        .arg(
            Arg::new("decrypt")
                .short('d')
                .action(ArgAction::SetTrue)
                .conflicts_with("detect")
                .help("If it should decrypt, i.e. shift leftwise instead of rightwise"),
        )
        .arg(
            Arg::new("alphabet")
                .short('a')
                .conflicts_with("detect")
                .help("ISO 639-3 code for an alphabet. Cannot be used in conjunction with language detection (-e)"),
        )
        .arg(
            Arg::new("detect")
                .short('e')
                .action(ArgAction::SetTrue)
                .conflicts_with("alphabet")
                .conflicts_with("decrypt")
                .help("Detect which language, and as a consequence which alphabet to use to decipher. Only available for ciphering. Cannot be used in conjunction with the alphabet argument (-a)")
        )
        .get_matches();

    let shift: i32 = *matches.get_one::<i32>("shift").unwrap();
    let filename = matches.get_one::<String>("filename").unwrap();
    let decrypt = matches.get_flag("decrypt");
    let text = fs::read_to_string(filename).expect("Failed to read file");

    let alphabet: Option<String>;
    if matches.get_flag("detect") {
        alphabet = detect(&text)
            .filter(|i| i.is_reliable())
            .map(|i| String::from(i.lang().code()))
            .map(|l: std::string::String| get_alphabet(&l));
    } else {
        alphabet = matches.get_one::<String>("alphabet").map(get_alphabet);
    }

    let result = caesar_cipher(
        &text,
        shift,
        decrypt,
        alphabet.unwrap_or(String::from(DEFAULT_ALPHABET)).as_str(),
    );
    println!("{}", result);
}
