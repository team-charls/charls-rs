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

    #[test]
    fn read_header_from_buffer_preceded_with_fill_bytes() {
        let mut buffer = Vec::new();

        write_byte(&mut buffer, 0xFF);
        write_start_of_image(&mut buffer);

        // writer.write_byte(extra_start_byte);
        // writer.write_start_of_frame_segment(1, 1, 2, 1);
        //
        // writer.write_byte(extra_start_byte);
        // writer.write_start_of_scan_segment(0, 1, 1, interleave_mode::none);

        let mut reader = JpegStreamReader::new(buffer.as_slice());
        assert!(reader.read_header().is_ok());
    }

    fn write_byte(buffer: &mut Vec<u8>, value: u8) {
        buffer.write_all(&[value]).unwrap();
    }

    fn write_start_of_image(buffer: &mut Vec<u8>) {
        buffer.write_all(&[0xFF, 0xD8]).unwrap();
    }

    fn write_start_of_frame_segment(buffer: &mut Vec<u8>, width: u16, height: u16, bits_per_sample: u8,
     component_count: u16) {
        // Create a Frame Header as defined in T.87, C.2.2 and T.81, B.2.2
        let mut segment = Vec::new();

        write_byte(&mut segment, bits_per_sample); // P = Sample precision
        // push_back(segment, static_cast<uint16_t>(height));          // Y = Number of lines
        // push_back(segment, static_cast<uint16_t>(width));           // X = Number of samples per line
        //
        // // Components
        // segment.push_back(static_cast<std::byte>(component_count)); // Nf = Number of image components in frame
        // for (int component_id{}; component_id < component_count; ++component_id)
        // {
        //     // Component Specification parameters
        //     if (componentIdOverride == 0)
        //     {
        //         segment.push_back(static_cast<std::byte>(component_id)); // Ci = Component identifier
        //     }
        //     else
        //     {
        //         segment.push_back(static_cast<std::byte>(componentIdOverride)); // Ci = Component identifier
        //     }
        //     segment.push_back(std::byte{0x11}); // Hi + Vi = Horizontal sampling factor + Vertical sampling factor
        //     segment.push_back(
        //         std::byte{0}); // Tqi = Quantization table destination selector (reserved for JPEG-LS, should be set to 0)
        // }
        //
        // write_segment(jpeg_marker_code::start_of_frame_jpegls, segment.data(), segment.size());
    }
}
