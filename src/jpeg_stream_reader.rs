// Copyright (c) Team CharLS.
// SPDX-License-Identifier: BSD-3-Clause

//mod jpeg_marker_code;

use std::io::Read;

use crate::jpeg_marker_code::JpegMarkerCode;
use crate::decoding_error::DecodingError;

#[derive(Clone, Debug)]
pub struct FrameInfo {
    width: u32,
    height: u32,
    bits_per_sample: u8,
    component_count: u8,
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
    AfterEndOfImage,
}


#[derive(Debug)]
pub struct JpegStreamReader<R: Read> {
    reader: R,
    frame_info: FrameInfo,
    state: ReaderState,
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
                component_count,
            },
            state: ReaderState::BeforeStartOfImage,
        }
    }

    pub fn read_next_marker_code(&mut self) -> Result<JpegMarkerCode, DecodingError> {
        let mut value = self.read_u8()?;
        if value != 255 {
            return Err(DecodingError::JpegMarkerStartByteNotFound);
        }

        // Read all preceding 0xFF fill values until a non 0xFF value has been found. (see ISO/IEC 10918-1, B.1.1.2)
        while value == 255 {
            value = self.read_u8()?;
        }

        let r = JpegMarkerCode::try_from(value);
        if r.is_err() {
            return Err(DecodingError::StartOfImageMarkerNotFound);
        }

        return Ok(r.unwrap());
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
        let extra_start_byte = 0xFFu8;
        let mut buffer = Vec::new();

        write_byte(&mut buffer, extra_start_byte);
        write_start_of_image(&mut buffer);

        write_byte(&mut buffer, extra_start_byte);
        write_start_of_frame_segment(&mut buffer, 1, 1, 2, 1);

        write_byte(&mut buffer, extra_start_byte);
        write_start_of_scan_segment(&mut buffer, 0, 1, 1, 0);

        let mut reader = JpegStreamReader::new(buffer.as_slice());
        assert!(reader.read_header().is_ok());
    }

    #[test]
    fn read_header_from_buffer_not_starting_with_ff_throws() {
        let mut buffer = Vec::new();
        buffer.write_all(&[0x0F, 0xFF, 0xD8, 0xFF, 0xFF, 0xDA]).unwrap();

        let mut reader = JpegStreamReader::new(buffer.as_slice());

        let x = reader.read_header().unwrap_err();
        assert_eq!(x, DecodingError::JpegMarkerStartByteNotFound);

        //
        // assert_expect_exception(jpegls_errc::jpeg_marker_start_byte_not_found, [&reader] { reader.read_header(); });
    }

    #[test]
    fn read_header_with_application_data() {
        for i in 0..16 {
            //read_header_with_application_data(i);
        }
    }

    fn read_header_with_application_data2(data_number: u8) {
        // jpeg_test_stream_writer writer;
        // writer.write_start_of_image();
        //
        // writer.write_byte(byte{0xFF});
        // writer.write_byte(static_cast<byte>(0xE0 + data_number));
        // writer.write_byte(byte{0x00});
        // writer.write_byte(byte{0x02});
        //
        // writer.write_start_of_frame_segment(1, 1, 2, 1);
        // writer.write_start_of_scan_segment(0, 1, 1, interleave_mode::none);
        //
        // jpeg_stream_reader reader;
        // reader.source({writer.buffer.data(), writer.buffer.size()});
        //
        // reader.read_header(); // if it doesn't throw test is passed.
    }

    fn write_byte(buffer: &mut Vec<u8>, value: u8) {
        buffer.write_all(&[value]).unwrap();
    }

    fn write_u16(buffer: &mut Vec<u8>, value: u16) {
        buffer.write_all(&value.to_be_bytes()).unwrap();
    }

    fn write_marker(buffer: &mut Vec<u8>, marker_code: JpegMarkerCode)
    {
        write_byte(buffer, 0xFF);
        write_byte(buffer, marker_code as u8);
    }

    fn write_start_of_image(buffer: &mut Vec<u8>) {
        buffer.write_all(&[0xFF, 0xD8]).unwrap();
    }

    fn write_start_of_frame_segment(buffer: &mut Vec<u8>, width: u16, height: u16, bits_per_sample: u8,
                                    component_count: u16) {
        // Create a Frame Header as defined in T.87, C.2.2 and T.81, B.2.2
        let mut segment = Vec::new();

        write_byte(&mut segment, bits_per_sample); // P = Sample precision
        write_u16(&mut segment, height); // Y = Number of lines
        write_u16(&mut segment, width); // X = Number of samples per line

        // Components
        write_byte(&mut segment, component_count as u8); // Nf = Number of image components in frame

        for component_id in 0..component_count as u8 {
            // Component Specification parameters
            write_byte(&mut segment, component_id); // Ci = Component identifier
            write_byte(&mut segment, 0x11); // Hi + Vi = Horizontal sampling factor + Vertical sampling factor
            write_byte(&mut segment, 0); // Tqi = Quantization table destination selector (reserved for JPEG-LS, should be set to 0)
        }

        write_segment(buffer, JpegMarkerCode::StartOfFrameJpegls, &segment);
    }

    fn write_start_of_scan_segment(buffer: &mut Vec<u8>, component_id: u8, component_count: u8, near_lossless: u8,
                                   interleave_mode: u8) {
        // Create a Scan Header as defined in T.87, C.2.3 and T.81, B.2.3
        let mut segment = Vec::new();

        write_byte(&mut segment, component_count);
        for i in 0..component_count {
            write_byte(&mut segment, component_id + i);
            write_byte(&mut segment, 0); // Mapping table selector (0 = no table)
        }

        write_byte(&mut segment, near_lossless); // NEAR parameter
        write_byte(&mut segment, interleave_mode); // ILV parameter
        write_byte(&mut segment, 0); // transformation

        write_segment(buffer, JpegMarkerCode::StartOfScan, &segment);
    }

    fn write_segment(buffer: &mut Vec<u8>, marker_code: JpegMarkerCode, segment_data: &Vec<u8>)
    {
        buffer.write_all(&[0xFF, 0xD8]).unwrap();

        write_marker(buffer, marker_code);
        write_u16(buffer, (segment_data.len() + 2) as u16);
        buffer.write_all(segment_data).unwrap();
    }
}
