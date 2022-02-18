// Copyright 2022 Canvas02 <Canvas02@protonmail.com>.
// SPDX-License-Identifier: MIT

const PASTE_RS_URL: &str = "https://paste.rs/";

use crate::error::PasteError;

#[derive(Debug)]
pub struct Paste(String);

// Use get_id() and get_url() methods
/*
impl fmt::Display for Paste {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Paste(id) = self;
        write!(f, "{}", id)
    }
}
*/

impl Paste {
    /// Make a new paste struct from a string(Url, incomplete url, id)
    ///
    /// # Example
    /// ```
    /// let paste = Paste::from("osx").unwrap();
    /// let paste = Paste::from("https://paste.rs/osx").unwrap();
    /// let paste = Paste::from("paste.rs/osx").unwrap();
    /// ```
    pub fn from(val: &str) -> Result<Self, PasteError> {
        if is_url(val) && is_paste_rs_url(val) {
            Ok(Paste(extract_paste_id(&val.to_string())?))
        } else if !is_url(val) && is_paste_rs_url(val) {
            let full_url = format!("https://{}", val);
            Ok(Paste(extract_paste_id(&full_url.to_string())?))
        } else if val.len() == 3 {
            Ok(Paste(val.to_string()))
        } else if is_url(val) && !is_paste_rs_url(val) {
            // bail!("Invalid URL")
            Err(PasteError::InvalidUrl)
        } else {
            // bail!("Invalid argument")
            Err(PasteError::InvalidArguments)
        }
    }

    /// Make a new paste
    ///
    /// # Example
    /// ```
    ///  let res = Paste::new("Hello world!".to_string())
    ///        .await
    ///        .unwrap()
    ///        .get_url();
    ///
    /// dbg!(res);
    /// ```
    ///
    pub async fn new(data: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let res = client
            .post(PASTE_RS_URL)
            .body(data)
            .header("Content-Type", "text/plain")
            .send()
            .await?
            .text()
            .await?;

        Ok(Paste(res))
    }

    /// Get a paste's content
    ///
    /// # Example
    /// ```
    /// let paste = Paste::from("osx").unwrap();
    ///
    /// let paste_content = paste.get().unwrap();
    ///
    /// dbg!(paste_content);
    /// ```
    ///
    pub async fn get(&self) -> Result<String, Box<dyn std::error::Error>> {
        let res = reqwest::get(self.get_url()).await?.text().await?;
        Ok(res)
    }

    /* ! Unused code
    /// Get the id of a Paste
    pub fn get_id(&self) -> String {
        let Paste(id) = self;
        id.to_owned()
    }
    */

    /// Get the url of a Paste
    pub fn get_url(&self) -> String {
        let Paste(id) = self;
        format!("{}/{}", PASTE_RS_URL, id)
    }
}

// ######################## Util functions ########################

/// Is the string a url?
fn is_url(url: &str) -> bool {
    if url.contains("http://") || url.contains("https://") {
        true
    } else {
        false
    }
}

// is the string a paste.rs url?
fn is_paste_rs_url(url: &str) -> bool {
    if url.contains("paste.rs") {
        true
    } else {
        false
    }
}

fn extract_paste_id(url: &String) -> Result<String, PasteError> {
    // let url = url.to_owned();
    // url.replace_range(0..PASTE_RS_URL.len(), "");
    if url.contains(PASTE_RS_URL) {
        Ok(url.replace(PASTE_RS_URL, ""))
    } else {
        // bail!("Url is not a Paste.rs url")
        Err(PasteError::InvalidUrl)
    }
}
