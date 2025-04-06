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

use hollow_knight_save_converter::*;

const TEST_FILE: &[u8] = include_bytes!("./user1.dat");

#[test]
fn test_decode_encode() {
    let decoded = decode(TEST_FILE);
    println!(
        "decoded keys: {:?}",
        decoded.keys().collect::<Vec<&String>>()
    );
    assert_eq!(
        &decoded,
        &serde_json::from_slice(serde_json::to_string(&decoded).unwrap().as_bytes()).unwrap()
    );
    let encoded = encode(serde_json::to_string(&decoded).unwrap().into_bytes());
    assert_eq!(&encoded, TEST_FILE);
}
