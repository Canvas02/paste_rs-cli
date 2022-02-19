// Copyright 2022 Canvas02 <Canvas02@protonmail.com>.
// SPDX-License-Identifier: MIT

pub type PasteResult<T> = core::result::Result<T, PasteError>;

#[derive(Debug)]
pub enum PasteError {
    InvalidUrl,
    InvalidArguments,
    ReqwestError(reqwest::Error),
}

impl std::convert::From<reqwest::Error> for PasteError {
    fn from(err: reqwest::Error) -> Self {
        PasteError::ReqwestError(err)
    }
}

impl std::error::Error for PasteError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            PasteError::ReqwestError(err) => Some(err),
            _ => None,
        }
    }
}

impl std::fmt::Display for PasteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            PasteError::InvalidArguments => write!(f, "Invalid Arguments"),
            PasteError::InvalidUrl => write!(f, "Invalid Url"),
            PasteError::ReqwestError(req_err) => write!(f, "{}", req_err),
        }
    }
}
