// Copyright 2022 Canvas02 <Canvas02@protonmail.com>.
// SPDX-License-Identifier: MIT

#[derive(Clone, Copy, Debug)]
pub enum PasteError {
    InvalidUrl,
    InvalidArguments,
}

impl std::error::Error for PasteError {}

impl std::fmt::Display for PasteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            PasteError::InvalidArguments => write!(f, "Invalid Arguments"),
            PasteError::InvalidUrl => write!(f, "Invalid Url"),
        }
    }
}
