//
// hollow-knight-save-converter
//
// Copyright 2025 Manos Pitsidianakis
//
// This file is part of hollow-knight-save-converter.
//
// hollow-knight-save-converter is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// hollow-knight-save-converter is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with hollow-knight-save-converter. If not, see <http://www.gnu.org/licenses/>.
//
// SPDX-License-Identifier: EUPL-1.2 OR GPL-3.0-or-later

use base64::prelude::*;
use serde_json::{Map, Value};

const C_SHARP_HEADER: &[u8] = &[
    0, 1, 0, 0, 0, 255, 255, 255, 255, 1, 0, 0, 0, 0, 0, 0, 0, 6, 1, 0, 0, 0,
];
const AES_KEY: &[u8] = b"UKu52ePUBwetZ9wNX88o54dnfKRu0T1l";

fn remove_header(bytes: &[u8]) -> &[u8] {
    // remove fixed CSharp header, plus the ending byte 11.
    let header_bytes = &bytes[C_SHARP_HEADER.len()..][..(bytes.len() - 1 - C_SHARP_HEADER.len())];

    // remove LengthPrefixedString header
    let mut pos = 1;
    while pos < 5 {
        if (header_bytes[pos - 1] & 0x80) == 0 {
            break;
        }
        pos += 1;
    }
    &header_bytes[pos..]
}

fn aes_decrypt(mut bytes: Vec<u8>) -> Vec<u8> {
    use aes::cipher::{BlockDecryptMut, KeyInit, block_padding::Pkcs7};
    type Aes128EcbDec = ecb::Decryptor<aes::Aes256>;

    let pt = Aes128EcbDec::new(AES_KEY.into())
        .decrypt_padded_mut::<Pkcs7>(&mut bytes)
        .unwrap();
    pt.to_vec()
}

/// Decodes a file into a JSON object ([`Map<String, Value>`]).
///
/// # Panics
///
/// This function panics if the input is in the wrong format.
pub fn decode(mut bytes: &[u8]) -> Map<String, Value> {
    bytes = remove_header(bytes);
    let decoded_encrypted = BASE64_STANDARD.decode(bytes).unwrap();
    let decoded_aes = aes_decrypt(decoded_encrypted);
    serde_json::from_slice(&decoded_aes).unwrap()
}

fn generate_length_prefixed_string(length: usize) -> Vec<u8> {
    let mut length = length.min(0x7FFFFFFF); // maximum value
    let mut bytes = Vec::with_capacity(8);
    for _ in 0..4 {
        if (length >> 7) != 0 {
            bytes.push(((length & 0x7F) | 0x80) as u8);
            length >>= 7;
        } else {
            bytes.push((length & 0x7F) as u8);
            length >>= 7;
            break;
        }
    }
    if length != 0 {
        bytes.push(length.try_into().unwrap())
    }

    bytes
}

fn add_header(bytes: Vec<u8>) -> Vec<u8> {
    let length_data = generate_length_prefixed_string(bytes.len());
    let mut new_bytes = vec![0; bytes.len() + C_SHARP_HEADER.len() + length_data.len() + 1];
    {
        // fixed header
        let (new_bytes, _) = new_bytes.split_at_mut(C_SHARP_HEADER.len());
        new_bytes.copy_from_slice(C_SHARP_HEADER);
    }
    {
        // variable LengthPrefixedString header
        let (_, new_bytes) = new_bytes.split_at_mut(C_SHARP_HEADER.len());
        new_bytes[..length_data.len()].copy_from_slice(&length_data);
    }
    {
        // the actual data
        let (_, new_bytes) = new_bytes.split_at_mut(length_data.len() + C_SHARP_HEADER.len());
        new_bytes[..bytes.len()].copy_from_slice(&bytes);
    }
    {
        // fixed header (11)
        let (_, new_bytes) =
            new_bytes.split_at_mut(length_data.len() + C_SHARP_HEADER.len() + bytes.len());
        new_bytes[..1].copy_from_slice(&[11]);
    }
    new_bytes
}

fn aes_encrypt(bytes: Vec<u8>) -> Vec<u8> {
    use aes::cipher::{BlockEncryptMut, KeyInit, block_padding::Pkcs7};
    type Aes128EcbEnc = ecb::Encryptor<aes::Aes256>;

    let plaintext_length = bytes.len();
    let pad_value: u8 = 16 - (bytes.len() % 16) as u8;
    let mut padded = vec![pad_value; bytes.len() + usize::from(pad_value)];
    {
        let (padded, _) = padded.split_at_mut(bytes.len());
        padded.copy_from_slice(&bytes);
    }
    let pt = Aes128EcbEnc::new(AES_KEY.into())
        .encrypt_padded_mut::<Pkcs7>(&mut padded, plaintext_length)
        .unwrap();
    pt.to_vec()
}

/// Encodes a JSON byte slice into a save file.
///
/// # Panics
///
/// The function panics if input is not valid JSON.
pub fn encode(bytes: Vec<u8>) -> Vec<u8> {
    // Deserialize and serialize into bytes again to remove any extraneous whitespace/formatting.
    let json_map: Map<String, Value> =
        serde_json::from_slice(&bytes).expect("Could not deserialize JSON from input bytes");
    let bytes = serde_json::to_string(&json_map)
        .expect("Failed to serialize input JSON")
        .into_bytes();
    let encrypted_bytes = aes_encrypt(bytes);
    let encrypted_base64 = BASE64_STANDARD.encode(encrypted_bytes).into_bytes();
    add_header(encrypted_base64)
}
