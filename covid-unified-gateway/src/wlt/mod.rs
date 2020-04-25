//! The module to send translation request to WLT
//! Construct [WLTTranslationRequest](struct.WLTTranslationRequest.html)
//! then call async [send method](struct.WLTTranslationRequest.html#method.send)
//! to get future result.

use dotenv::dotenv;
use serde::{ Deserialize, Serialize };
use std::fmt::{ Debug };
use std::env;
use super::utils::{ post_json, CurlErr };

#[derive(Serialize)]
pub struct WLTTranslationRequest<'a> {
    #[serde(skip)]
    pub endpoint: String,
    #[serde(skip)]
    pub api_key: &'a str,
    pub model_id: String,
    pub text: &'a[&'a str]
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Translation {
    pub translation: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WLTTranslationResponse {
    pub word_count: u32,
    pub character_count: u32,
    pub translations: Vec<Translation>
}

#[derive(Debug)]
pub enum WLTErr {
    BuildRequestErr,
    SendRequestErr,
    DecodeResultErr,
    NoTranslationErr
}

impl<'a> WLTTranslationRequest<'a> {
    pub fn new(endpoint: &'a str, api_key: &'a str, text: &'a [&'a str], source: &'a str, target: &'a str, version: &'a str) -> WLTTranslationRequest<'a> {
        dotenv().ok();
        let model_id = env::var(format!("{}_{}", source, target)).unwrap_or(format!("{}-{}", source, target));
        
        WLTTranslationRequest {
            endpoint: format!("{}/v3/translate?version={}", endpoint, version),
            api_key,
            model_id,
            text
        }
    }

    pub async fn send(&self) -> Result<WLTTranslationResponse, WLTErr> {
        match post_json(&self.endpoint, &self.api_key, Some(self)) as Result<WLTTranslationResponse, CurlErr> {
            Ok(result) => {
                if result.translations.len() > 0 {
                    Ok(result)
                } else {
                    Err(WLTErr::NoTranslationErr)
                }
            },
            Err(e) => {
                match e {
                    CurlErr::InvalidUrl => {
                        println!("The endpoing url may be invalid");
                        Err(WLTErr::BuildRequestErr)
                    },
                    CurlErr::InvalidInputData => {
                        println!("The input data may be invalid");
                        Err(WLTErr::BuildRequestErr)
                    },
                    CurlErr::UnexpectedOutputData => {
                        println!("The Curl engine doesn't support output data");
                        Err(WLTErr::DecodeResultErr)
                    },
                    CurlErr::RequestFail => {
                        println!("Fail to deliver translation request to server");
                        Err(WLTErr::SendRequestErr)
                    },
                    CurlErr::IncompatibleResultData => {
                        println!("The return data cannot be parsed into given struct");
                        Err(WLTErr::DecodeResultErr)
                    }
                }
            }
        }
    }
}