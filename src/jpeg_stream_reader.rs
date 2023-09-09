// Copyright (c) Team CharLS.
// SPDX-License-Identifier: BSD-3-Clause

//mod jpeg_marker_code;

use std::io::{Read, self};

use crate::jpeg_marker_code::JpegMarkerCode;
use crate::decoding_error::DecodingError;

#[derive(Clone, Debug)]
pub struct FrameInfo {
    width:           u32,
    height:          u32,
    bits_per_sample: u8,
    component_count: u8
}


#[derive(Debug, Eq, PartialEq)]
enum ReaderState
{
    BeforeStartOfImage,
    HeaderSection,
    SpiffHeaderSection,
    ImageSection,
    FrameSection,
    ScanSection,
    BitStreamSection,
    AfterEndOfImage
}


#[derive(Debug)]
pub struct JpegStreamReader<R: Read> {
    reader: R,
    frame_info: FrameInfo,
    state: ReaderState
}


impl<R: Read> JpegStreamReader<R> {
    pub fn new(mut r: R) -> JpegStreamReader<R> {
        let width = 0;
        let height = 0;
        let bits_per_sample = 0;
        let component_count = 0;

        JpegStreamReader {
            reader: r,
            frame_info: FrameInfo {
                width,
                height,
                bits_per_sample,
                component_count
            },
            state: ReaderState::BeforeStartOfImage
        }
    }

    pub fn read_next_marker_code(&mut self) -> JpegMarkerCode {
        JpegMarkerCode::StartOfImage
    }

    pub fn read_header(&mut self) -> Result<(), DecodingError> {
        if self.state == ReaderState::BeforeStartOfImage {
            if self.read_next_marker_code() != JpegMarkerCode::StartOfImage {
                return Err(DecodingError::StartOfImageMarkerNotFound);
            }

            self.state = ReaderState::HeaderSection;
        }

        Ok(())
    }
}

