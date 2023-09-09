// Copyright (c) Team CharLS.
// SPDX-License-Identifier: BSD-3-Clause

use std::io;

#[derive(Debug)]
pub enum DecodingError {
    /// An error in IO of the underlying reader.
    IoError(io::Error),

    StartOfImageMarkerNotFound,
    UnknownError
}
