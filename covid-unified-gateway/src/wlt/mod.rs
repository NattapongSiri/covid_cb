use serde::{ Deserialize, Serialize };
use std::fmt::{ Debug };
use super::utils::{ post_json, CurlErr, RawResponse };

#[derive(Serialize)]
pub struct WLTTranslationRequest<'a> {
    #[serde(skip)]
    pub endpoint: String,
    #[serde(skip)]
    pub api_key: &'a str,
    pub text: &'a[&'a str],
    pub source: &'a str,
    pub target: &'a str
}

#[derive(Deserialize, Serialize)]
pub struct Translation {
    pub translation: String
}

#[derive(Deserialize, Serialize)]
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
        WLTTranslationRequest {
            endpoint: format!("{}/v3/translate?version={}", endpoint, version),
            api_key: api_key,
            text: text,
            source: source,
            target: target
        }
    }

    pub async fn send(&self) -> Result<RawResponse<WLTTranslationResponse>, WLTErr> {
        match post_json(&self.endpoint, &self.api_key, Some(self)) as Result<RawResponse<WLTTranslationResponse>, CurlErr> {
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

    // pub async fn send_with(&self, client: &reqwest::Client) -> Result<WLTTranslationResponse, WLTErr> {
    //     let wlt_request = client.post(&self.endpoint)
    //                                     .basic_auth("apikey", Some(&self.api_key))
    //                                     .query(&[("version", "2018-05-01")])
    //                                     .json(&self)
    //                                     .build();
    //     if let Ok(req) = wlt_request {
    //         if let Ok(res) = client.execute(req).await {
    //             if let Ok(body) = res.json::<WLTTranslationResponse>().await {
    //                 if body.translations.len() > 0 {
    //                     Ok(body)
    //                 } else {
    //                     Err(WLTErr::NoTranslationErr)
    //                 }
    //             } else {
    //                 Err(WLTErr::DecodeResultErr)
    //             }
    //         } else {
    //             Err(WLTErr::SendRequestErr)
    //         }
    //     } else {
    //         Err(WLTErr::BuildRequestErr)
    //     }
    // }
}

// pub async fn translate() {
//     let client = reqwest::Client::new();
//     let wlt_request = client.post(&translation_endpoint)
//                                     .basic_auth("apikey", Some(api_key))
//                                     .query(&[("version", "2018-05-01")])
//                                     .json(&WLTTranslationRequest {
//                                         text: &[&params.message],
//                                         source: &params.source_lang,
//                                         target: &params.target_lang
//                                     }).build().expect("Fail to build request");
//     let result = client.execute(wlt_request).await.expect("Fail to send request");
//     let wlt_result = result.json::<WLTTranslationResponse>().await.expect("Fail to convert response to json");
//     if wlt_result.translations.len() > 0 {
//         params.message = wlt_result.translations[0].translation.to_owned();
//     }
//     println!("{}", serde_json::to_string(&params).unwrap());
// }