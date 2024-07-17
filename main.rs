use std::str;
use std::env;

static EQUALS : u8 = 255;

fn ascii_to_base64(ascii : &u8) -> Option<u8> {
    match ascii {
        97..=122 => Some(ascii - 71),
        65..=90 => Some(ascii - 65),
        48..=57 => Some(ascii + 4),
        43 => Some(62),
        47 => Some(63),
        61 => Some(EQUALS),
         _ => None
    }
}

fn main() {

    let args : Vec<String> = env::args().collect();

    assert!(args.len() >= 2, "Program requires input string");

    let input_string = &args[1];
    assert!(input_string.len() % 4 == 0);

    let input_byte_array : Vec<u8> = input_string.as_bytes().iter().map(|x| ascii_to_base64(x).unwrap()).collect();
    assert!(input_byte_array.len() % 4 == 0);
    assert!(input_byte_array.len() == input_string.len());

    let mut result : Vec<u8> = vec![];

    for chunk in input_byte_array.chunks(4) {
        result.push(chunk[0] << 2 | (chunk[1] >> 4) & 0x03);

        if chunk[2] == EQUALS {
            continue;
        }

        result.push(chunk[1] << 4 | (chunk[2] >> 2) & 0x0F);

        if chunk[3] == EQUALS {
            continue;
        }

        result.push((chunk[2] << 6) | chunk[3] & 0x3F);
    }

    match str::from_utf8(&result) {
        Ok(v) => println!("{}", v),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e)
    }
}
