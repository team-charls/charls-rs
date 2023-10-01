// Copyright (c) Team CharLS.
// SPDX-License-Identifier: BSD-3-Clause

use std::io::{Read};

#[warn(unused_variables)]




#[derive(Debug)]
pub struct Decoder<R: Read> {
    reader: R,
    width:           u32,
    height:          u32,
    bits_per_sample: u8,
    component_count: u8
}


impl<R: Read> Decoder<R> {
    pub fn new(r: R) -> Decoder<R> {
        let width = 0;
        let height = 0;
        let bits_per_sample = 0;
        let component_count = 0;

        Decoder {
            reader: r,
            width: width,
            height: height,
            bits_per_sample: bits_per_sample,
            component_count: component_count
        }
    }
}
