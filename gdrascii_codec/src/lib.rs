// SPDX-FileCopyrightText: 2022 Jonah Brüchert <jbb@kaidan.im>
//
// SPDX-License-Identifier: EUPL-1.2

#![no_std]

extern crate alloc;
use alloc::{string::String, vec::Vec};

// UTF-8 to GDR ASCII
const fn utf8_to_gdr_ascii(c: char) -> Option<&'static [u8]> {
    Some(match c {
        // control characters
        '\x08' => b"\x72", // backspace
        '\t' => b"\x79",
        '\n' => b"\x77",
        '\r' => b"\x78",

        // punctuation
        ' ' => b"\x71",
        '!' => b"\x42",
        '"' => b"\x43",
        '#' => b"\x41",
        '$' => b"\x48",
        '%' => b"\x04",
        '&' => b"\x02",
        '\'' => b"\x17",
        '(' => b"\x1D",
        ')' => b"\x1F",
        '*' => b"\x1B",
        '+' => b"\x25",
        ',' => b"\x64",
        '-' => b"\x62",
        '.' => b"\x63",
        '/' => b"\x40",

        // digits
        '0' => b"\x0D",
        '1' => b"\x11",
        '2' => b"\x10",
        '3' => b"\x0F",
        '4' => b"\x0E",
        '5' => b"\x0C",
        '6' => b"\x0B",
        '7' => b"\x0A",
        '8' => b"\x09",
        '9' => b"\x08",

        // more punctuation
        ':' => b"\x13",
        ';' => b"\x3B",
        '=' => b"\x2E",
        '?' => b"\x35",

        // upper case letters
        'A' => b"\x30",
        'B' => b"\x18",
        'C' => b"\x20",
        'D' => b"\x14",
        'E' => b"\x34",
        'F' => b"\x3E",
        'G' => b"\x1C",
        'H' => b"\x12",
        'I' => b"\x21",
        'J' => b"\x32",
        'K' => b"\x24",
        'L' => b"\x2C",
        'M' => b"\x16",
        'N' => b"\x2A",
        'O' => b"\x1E",
        'P' => b"\x2F",
        'Q' => b"\x1A",
        'R' => b"\x36",
        'S' => b"\x33",
        'T' => b"\x37",
        'U' => b"\x28",
        'V' => b"\x22",
        'W' => b"\x2D",
        'X' => b"\x26",
        'Y' => b"\x31",
        'Z' => b"\x38",

        // punctuation
        '^' => b"\x19\x71",
        '_' => b"\x01",
        '`' => b"\x2B\x71",

        // lower case letters
        'a' => b"\x61",
        'b' => b"\x4E",
        'c' => b"\x57",
        'd' => b"\x53",
        'e' => b"\x5A",
        'f' => b"\x49",
        'g' => b"\x60",
        'h' => b"\x55",
        'i' => b"\x05",
        'j' => b"\x4B",
        'k' => b"\x50",
        'l' => b"\x4D",
        'm' => b"\x4A",
        'n' => b"\x5C",
        'o' => b"\x5E",
        'p' => b"\x5B",
        'q' => b"\x52",
        'r' => b"\x59",
        's' => b"\x58",
        't' => b"\x56",
        'u' => b"\x5D",
        'v' => b"\x4F",
        'w' => b"\x4C",
        'x' => b"\x5F",
        'y' => b"\x51",
        'z' => b"\x54",

        // special chars
        '|' => b"\x27",
        '£' => b"\x06",
        '§' => b"\x3D",
        '¨' => b"\x03\x71",
        '°' => b"\x39",
        '²' => b"\x15",
        '³' => b"\x23",

        // umlauts, accents
        'Ä' => b"\x3F",
        'Ö' => b"\x3C",
        'Ü' => b"\x3A",
        'ß' => b"\x47",
        'ä' => b"\x65",
        'ç' => b"\x45",
        'è' => b"\x46",
        'é' => b"\x44",
        'ö' => b"\x66",
        'ü' => b"\x67",
        '´' => b"\x29\x71",
        'μ' => b"\x07",

        // combined chars
        '€' => b"\x20\x72\x2E",

        _ => return None,
    })
}

const fn gdr_ascii_to_utf8(bytes: &[u8]) -> Option<char> {
    Some(match bytes {
        // control characters
        b"\x72" => '\x08',
        b"\x79" => '\t',
        b"\x77" => '\n',
        b"\x78" => '\r',

        // punctuation
        b"\x71" => ' ',
        b"\x42" => '!',
        b"\x43" => '"',
        b"\x41" => '#',
        b"\x48" => '$',
        b"\x04" => '%',
        b"\x02" => '&',
        b"\x17" => '\'',
        b"\x1D" => '(',
        b"\x1F" => ')',
        b"\x1B" => '*',
        b"\x25" => '+',
        b"\x64" => ',',
        b"\x62" => '-',
        b"\x63" => '.',
        b"\x40" => '/',

        // digits
        b"\x0D" => '0',
        b"\x11" => '1',
        b"\x10" => '2',
        b"\x0F" => '3',
        b"\x0E" => '4',
        b"\x0C" => '5',
        b"\x0B" => '6',
        b"\x0A" => '7',
        b"\x09" => '8',
        b"\x08" => '9',

        // more punctuation
        b"\x13" => ':',
        b"\x3B" => ';',
        b"\x2E" => '=',
        b"\x35" => '?',

        // upper case letters
        b"\x30" => 'A',
        b"\x18" => 'B',
        b"\x20" => 'C',
        b"\x14" => 'D',
        b"\x34" => 'E',
        b"\x3E" => 'F',
        b"\x1C" => 'G',
        b"\x12" => 'H',
        b"\x21" => 'I',
        b"\x32" => 'J',
        b"\x24" => 'K',
        b"\x2C" => 'L',
        b"\x16" => 'M',
        b"\x2A" => 'N',
        b"\x1E" => 'O',
        b"\x2F" => 'P',
        b"\x1A" => 'Q',
        b"\x36" => 'R',
        b"\x33" => 'S',
        b"\x37" => 'T',
        b"\x28" => 'U',
        b"\x22" => 'V',
        b"\x2D" => 'W',
        b"\x26" => 'X',
        b"\x31" => 'Y',
        b"\x38" => 'Z',

        // punctuation
        b"\x19\x71" => '^',
        b"\x01" => '_',
        b"\x2B\x71" => '`',
        b"\x29" => '´', // TODO check
        b"\x2B" => '`',  // TODO check

        // lower case letters
        b"\x61" => 'a',
        b"\x4E" => 'b',
        b"\x57" => 'c',
        b"\x53" => 'd',
        b"\x5A" => 'e',
        b"\x49" => 'f',
        b"\x60" => 'g',
        b"\x55" => 'h',
        b"\x05" => 'i',
        b"\x4B" => 'j',
        b"\x50" => 'k',
        b"\x4D" => 'l',
        b"\x4A" => 'm',
        b"\x5C" => 'n',
        b"\x5E" => 'o',
        b"\x5B" => 'p',
        b"\x52" => 'q',
        b"\x59" => 'r',
        b"\x58" => 's',
        b"\x56" => 't',
        b"\x5D" => 'u',
        b"\x4F" => 'v',
        b"\x4C" => 'w',
        b"\x5F" => 'x',
        b"\x51" => 'y',
        b"\x54" => 'z',

        // special chars
        b"\x27" => '|',
        b"\x06" => '£',
        b"\x3D" => '§',
        b"\x03x71" => '¨',
        b"\x39" => '°',
        b"\x15" => '²',
        b"\x23" => '³',

        // umlauts, accents
        b"\x3F" => 'Ä',
        b"\x3C" => 'Ö',
        b"\x3A" => 'Ü',
        b"\x47" => 'ß',
        b"\x65" => 'ä',
        b"\x45" => 'ç',
        b"\x46" => 'è',
        b"\x44" => 'é',
        b"\x66" => 'ö',
        b"\x67" => 'ü',
        b"\x29\x71" => '´',
        b"\x07" => 'μ',

        // combined chars
        b"\x20\x72\x2E" => '€',

        _ => return None,
    })
}

/// Errors that can happen while encoding or decoding
#[derive(Debug)]
pub enum EncodingError {
    /// The character is not representable in the target codec
    UnrepresentableCharacter,
    /// The input is not valid data encoded in the source codec and can't be decoded.
    InvalidInput,
}

/// Pre-configured Result used by failable functions in this crate.
pub type EncodingResult<T> = Result<T, EncodingError>;

/// Encode a single char
pub const fn encode_char(character: char) -> Option<&'static [u8]> {
    utf8_to_gdr_ascii(character)
}

/// Encode a string.
/// Returns an Ok Result if all characters could be represented in the text coded, otherwise returns an Err.
pub fn try_encode(text: &str) -> EncodingResult<Vec<u8>> {
    // This is only an approximation, GDR encoding is sometimes longer
    let mut out = Vec::<u8>::with_capacity(text.len());

    for c in text.chars() {
        match utf8_to_gdr_ascii(c) {
            Some(encoded) => out.extend_from_slice(encoded),
            None => return Err(EncodingError::UnrepresentableCharacter),
        }
    }

    Ok(out)
}

/// Encodes a string.
/// If characters could not be encoded, a questionmark character is written instead.
/// This function always succeeds.
pub fn encode(text: &str) -> Vec<u8> {
    // This is only an approximation, GDR encoding is sometimes longer
    let mut out = Vec::<u8>::with_capacity(text.len());

    for c in text.chars() {
        let encoded = match utf8_to_gdr_ascii(c) {
            Some(encoded_char) => encoded_char,
            None => utf8_to_gdr_ascii('?').expect("? is always part of the codec"),
        };

        out.extend_from_slice(encoded);
    }

    out
}

/// Decode a single character.
/// This can handle multi-byte characters, but the sequence always needs to represent just one single character.
pub fn decode_char(character: &[u8]) -> EncodingResult<char> {
    gdr_ascii_to_utf8(character).ok_or(EncodingError::InvalidInput)
}

/// Decode bytes into a string.
/// This function never fails if the input is valid, that means only contains sequences defined in the decoding.
/// If that is not the case, it returns an Err.
pub fn decode(text: &[u8]) -> EncodingResult<String> {
    // Approximation of the expected required size
    let mut out = String::with_capacity((text.len() as f32 * 0.95) as usize);

    let mut i = 0;
    let input_len = text.len();
    while i < input_len {
        if i + 3 <= input_len {
            if let Ok(character) = decode_char(&text[i..i + 3]) {
                out.push(character);
                i += 3;
                continue;
            }
        }
        if i + 2 <= input_len {
            if let Ok(character) = decode_char(&text[i..i + 2]) {
                out.push(character);
                i += 2;
                continue;
            }
        }

        let c = decode_char(&text[i..i + 1])?;
        out.push(c);
        i += 1;
    }

    Ok(out)
}

mod test {
    #[test]
    fn encode_hello_world() -> crate::EncodingResult<()> {
        use crate::try_encode;

        let encoded = try_encode("Hello World")?;
        assert_eq!(encoded, b"\x12\x5A\x4D\x4D\x5E\x71\x2D\x5E\x59\x4D\x53");

        Ok(())
    }

    #[test]
    fn unrepresentable_character() {
        use crate::{try_encode, EncodingError};

        let result = try_encode("@");
        assert!(matches!(
            result,
            Err(EncodingError::UnrepresentableCharacter)
        ));
    }

    #[test]
    fn ignore_errors() {
        use crate::encode;

        let result = encode("@");
        assert_eq!(result, b"\x35");
    }

    #[test]
    fn decode_test() -> crate::EncodingResult<()> {
        use crate::decode;

        let decoded =
            decode(b"\x12\x5A\x4D\x4D\x5E\x71\x20\x72\x2E\x71\x2D\x5E\x59\x4D\x53\x29\x71")?;
        assert_eq!(decoded, "Hello € World´");
        Ok(())
    }

    #[test]
    fn decode_invalid() {
        use crate::{decode, EncodingError};

        let decoded = decode(b"asd89wesad");
        assert!(matches!(decoded, Err(EncodingError::InvalidInput)));
    }

    #[test]
    fn encode_decode_lorem_ipsum() -> crate::EncodingResult<()> {
        use crate::{decode, encode};

        // This is a normal Lorem ipsum text with some multi-byte characters hidden in it,
        // To make sure the decoder never gets confused by them.
        let example_text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Vel pharetra vel turpis nunc. Consequat interdum varius sit amet mattis vulputate enim. €Vel orci porta non pulvinar neque. Lobortis scelerisque fermentum dui faucibus. Porta nibh venenatis cras sed felis eget velit. Fusce ut placerat orci nulla pellentesque dignissim. Interdum consectetur libero id faucibus nisl. Vitae elementum curabitur vitae nunc. Justo donec enim diam vulputate. Maecenas volutpat blandit aliquam etiam erat velit scelerisque in dictum. ** Nec sagittis aliquam malesuada bibendum arcu. Dictum non consectetur a erat. Vivamus arcu felis bibendum ut. Blandit aliquam etiam erat velit scelerisque in. Pharetra et ultrices neque ornare aenean.

Convallis convallis tellus id interdum velit. Ante metus dictum at tempor commodo ullamcorper a lacus. Suspendisse interdum consectetur libero id faucibus nisl tincidunt. Suspendisse in est ante in nibh mauris cursus mattis. Massa sed elementum tempus egestas sed. Velit egestas dui id ornare arcu odio. Nam at lectus urna duis. Nisl ^suscipit adipiscing bibendum est ultricies integer quis auctor elit. Sagittis id consectetur purus ut faucibus pulvinar elementum integer. Egestas egestas fringilla phasellus faucibus. Cursus turpis massa tincidunt dui ut ornare. Arcu non odio euismod lacinia at. Amet nisl purus in mollis nunc sed.

Feugiat in ante metus dictum at. Urna et pharetra pharetra massa massa ultricies. Tellus in metus vulputate eu scelerisque felis imperdiet. Ut sem viverra aliquet eget sit amet tellus cras. Nunc sed augue lacus viverra vitae congue. Volutpat sed cras ornare arcu. Ut tristique et egestas quis ipsum suspendisse. Sit amet massa vitae tortor condimentum lacinia quis. Massa vitae tortor condimentum lacinia quis vel eros. Volutpat diam ut venenatis tellus in metus. Lobortis mattis aliquam faucibus purus in.

Vel eros donec ac odio tempor orci dapibus ultrices in. Nunc sed blandit libero volutpat sed cras ornare arcu. Fusce id velit ut tortor. Elit ut aliquam purus sit amet luctus. Sed blandit libero volutpat sed. Tortor id aliquet lectus proin nibh. Viverra justo nec ultrices dui sapien eget. Dignissim diam quis enim lobortis scelerisque fermentum dui faucibus. Diam vulputate ut pharetra sit amet. Amet consectetur adipiscing *elit pellentesque habitant morbi tristique senectus et. Augue eget arcu dictum varius duis at consectetur lorem donec. Sed turpis tincidunt id aliquet risus feugiat in ante. Vitae elementum curabitur vitae nunc sed velit. Adipiscing elit pellentesque habitant morbi tristique senectus et. Feugiat vivamus at augue eget arcu dictum varius duis. Erat imperdiet sed euismod nisi porta. Tempor id eu nisl nunc mi ipsum faucibus. Nisi scelerisque eu ultrices vitae auctor eu. Nibh cras pulvinar mattis nunc sed blandit libero volutpat. Ac tortor dignissim convallis aenean et tortor at risus.

Massa vitae tortor condimentum lacinia quis vel. Ullamcorper malesuada proin libero nunc consequat interdum varius sit. Dui faucibus in ornare quam viverra. Egestas diam in arcu cursus euismod quis viverra. Hac habitasse platea dictumst quisque sagittis. Integer enim neque volutpat ac tincidunt vitae semper quis. Eleifend donec pretium vulputate sapien nec sagittis. Risus pretium quam vulputate dignissim suspendisse in est. Scelerisque fermentum dui faucibus in. Facilisis volutpat est velit egestas dui. Aliquam eleifend mi in nulla posuere sollicitudin aliquam ultrices. Faucibus in ornare quam viverra orci sagittis eu volutpat. Proin sagittis nisl rhoncus mattis rhoncus urna neque viverra.";

        let encoded = encode(example_text);
        let decoded = decode(&encoded)?;
        assert_eq!(decoded, example_text);
        Ok(())
    }

    #[test]
    fn decode_char() -> crate::EncodingResult<()> {
        use crate::decode_char;

        let c = decode_char(b"\x20\x72\x2E")?;
        assert_eq!(c, '€');

        let c = decode_char(b"\x254");
        assert!(matches!(c, Err(crate::EncodingError::InvalidInput)));

        let c = decode_char(b"");
        assert!(matches!(c, Err(crate::EncodingError::InvalidInput)));

        Ok(())
    }
}
