// Copyright 2022 Canvas02.
// SPDX-License-Identifier: MIT

pub fn is_url(url: &str) -> bool {
    if url.contains("http://") || url.contains("https://") {
        true
    } else {
        false
    }
}

pub fn is_paste_rs_url(url: &str) -> bool {
    if url.contains("paste.rs") {
        true
    } else {
        false
    }
}
