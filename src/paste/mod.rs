// Copyright 2022 Canvas02.
// SPDX-License-Identifier: MIT

// ! Used in development never turn on for a production build
// #![allow(dead_code)]
// #![allow(unused_variables)]
// ! enable for production build
#![deny(dead_code)]
#![deny(unused_variables)]

use crate::util::*;
use reqwest::Client;

const PASTE_RS_URL: &str = "https://paste.rs/";

pub async fn new_paste(data: String) -> Result<Url, reqwest::Error> {
    let client = Client::new();
    let res = client
        .post(PASTE_RS_URL)
        .body(data)
        .header("Content-Type", "text/plain")
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    Ok(Url::new(&*res).unwrap())

    // !Deprecated
    // // match res.error_for_status() {
    // //     Ok(res) => {
    // //         let res_text = res.text().await?;
    // //         Ok(Url::new(&*res_text).unwrap())
    // //     }
    // //     Err(err) => Err(err),
    // // }
}

pub struct Url(String);

impl std::fmt::Display for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let url = self.get_url();
        write!(f, "{}", url)
    }
}

impl Url {
    pub fn new(val: &str) -> Result<Url, String> {
        // cheching if the argument is an url
        if is_url(val) && is_paste_rs_url(val) {
            Ok(Url(val.to_string()))
        } else if is_url(val) && !is_paste_rs_url(val) {
            Err("The url is not a https://paste.rs/ url".to_string())
        } else if !is_url(val) && is_paste_rs_url(val) {
            Ok(Url(format!("https://{}", val)))
        } else if val.len() == 3 {
            Ok(Url(format!("{}{}", PASTE_RS_URL, val)))
        } else {
            Err("Invalid argument".to_string())
        }
    }

    pub fn get_url(&self) -> &String {
        let Url(url) = self;
        url
    }

    pub fn get_id(&mut self) -> String {
        let Url(url) = self;
        url.replace_range(0..PASTE_RS_URL.len(), "");
        url.clone()
    }

    pub async fn make_request(&self) -> Result<String, reqwest::Error> {
        let res = reqwest::get(self.get_url())
            .await?
            .error_for_status()?
            .text()
            .await;
        res
    }
}
