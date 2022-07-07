use std::io::{self, BufRead};
use std::convert::TryFrom;
use std::{str::Bytes};

fn main() {
    
    let stdin = io::stdin();
    let mut c = 0;
    let mut secret = String::from("");
    for line in stdin.lock().lines() {
        let current = line.unwrap();
        //println!("data: {}", current);

        if c == 0 {
            secret =  String::from(current.clone());

        }
        if c >= 1 {
            decode_word( String::from(current.clone()), secret.clone());
        }
        c +=1;
    }
}

fn decode_word(encoded:String, secret: String)  {
    let alphabet: Bytes = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".bytes();
    let alp_count: i32 = i32::try_from(alphabet.len()).unwrap();

    let mut output: String = "".to_owned();
    let mut last_position = 0;
    let sum =  sum_word(secret);

    for b in  encoded.bytes() {

        let character = b as char;
        let find_char = alphabet.clone().position(|c| c == b);
        
        if find_char.is_none() {
            output.push(character);
            continue;
        }

        let position = find_char.unwrap() as i32;
        let counter = sum + last_position;
        let dec_position = get_position(position, counter, alp_count);
        last_position = position;
        
        let mut z = 0;

        for j in  alphabet.clone().into_iter() {

            if dec_position == z {
                output.push(j as char)
            }
            
            z += 1;
        }
    }

    println!("FINAL OUTPUT: {}", output);

}

fn sum_word(secret: String) -> i32 {
    let sum: i32 = secret
    .as_bytes()
    .iter()
    .map(|&b| b as i32)
    .sum();

    sum
}

fn get_position(mut position: i32, mut amount: i32, dict_count: i32) -> i32 {
    while amount > 0 {
        if position == 0 {
            position = dict_count - 1;
        } else {
            position -= 1; 
        }

        amount -= 1;

    }

    position
}

