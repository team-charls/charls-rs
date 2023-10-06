// Copyright (c) Team CharLS.
// SPDX-License-Identifier: BSD-3-Clause

#[derive(Debug, PartialEq)]
pub enum DecodingError {
    /// An error in IO of the underlying reader.
    IoError,
    JpegMarkerStartByteNotFound,
    StartOfImageMarkerNotFound,
    UnknownError
}
