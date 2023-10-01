// Copyright (c) Team CharLS.
// SPDX-License-Identifier: BSD-3-Clause

//mod jpeg_marker_code;

use std::io::Read;

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
    pub fn new(r: R) -> JpegStreamReader<R> {
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

    pub fn read_next_marker_code(&mut self) -> Result<JpegMarkerCode, DecodingError> {
        let mut value = self.read_u8()?;
        if value != 255 {
            return Err(DecodingError::StartOfImageMarkerNotFound);
        }

        // Read all preceding 0xFF fill values until a non 0xFF value has been found. (see ISO/IEC 10918-1, B.1.1.2)
        while value == 255 {
            value = self.read_u8()?;
        }

        let r = JpegMarkerCode::try_from(value);
        if r.is_err() {
            return Err(DecodingError::StartOfImageMarkerNotFound);
        }

        return Ok(r.unwrap())
    }

    pub fn read_header(&mut self) -> Result<(), DecodingError> {
        if self.state == ReaderState::BeforeStartOfImage {
            if self.read_next_marker_code()? != JpegMarkerCode::StartOfImage {
                return Err(DecodingError::StartOfImageMarkerNotFound);
            }

            self.state = ReaderState::HeaderSection;
        }

        Ok(())
    }

    fn read_u8(&mut self) -> Result<u8, DecodingError> {
        let mut buf = [0; 1];
        let result = self.reader.read_exact(&mut buf);
        if result.is_err() {
            return Err(DecodingError::UnknownError);
        }

        Ok(buf[0])
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use super::*;

    #[test]
    fn read_header_from_too_small_input_buffer_fails() {
        let mut buffer = Vec::new();
        buffer.write_all(&[1]).unwrap();

        let mut reader = JpegStreamReader::new(buffer.as_slice());
        assert!(reader.read_header().is_err());
    }
}
