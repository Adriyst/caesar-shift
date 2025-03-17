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
        .arg(Arg::new("shift").required(true))
        .arg(Arg::new("filename").required(true))
        .arg(Arg::new("decrypt").short('d').action(ArgAction::SetTrue))
        .arg(Arg::new("alphabet").short('a'))
        .arg(Arg::new("detect").short('e').action(ArgAction::SetTrue))
        .get_matches();

    let shift: i32 = matches
        .get_one::<String>("shift")
        .unwrap()
        .parse()
        .expect("Shift must be a number");
    let filename = matches.get_one::<String>("filename").unwrap();
    let decrypt = matches.get_flag("decrypt");
    let text = fs::read_to_string(filename).expect("Failed to read file");

    if matches.get_one::<String>("alphabet").is_some() && matches.get_flag("detect") {
        panic!("Cannot detect language if one is provided");
    }

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
