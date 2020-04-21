use dotenv::dotenv;
use std::env;
use serde::{Deserialize, Serialize};
use serde_json::{json};
use utils::{RawResponse};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Params {
    #[serde(skip_serializing_if="Option::is_none")]
    context: Option<wa::UnknownType>,
    #[serde(skip_serializing_if="Option::is_none")]
    session_id: Option<String>,
    message: String,
    source_lang: String,
    target_lang: String
}

mod wlt;
mod wa;
mod utils;

fn main() {
    dotenv().ok();
    let wlt_api_key = env::var("WLT_APIKEY").expect("Undefined WLT_APIKEY");
    let wlt_endpoint = env::var("WLT_ENDPOINT").expect("Undefined WLT_ENDPOINT");
    let wlt_version = env::var("WLT_VERSION").expect("Undefined WLT_VERSION");
    let wlt_retry = env::var("WLT_RETRY").map_or(1usize, |r| r.parse().expect("WLT_RETRY shall be numeric"));

    let wa_endpoint = std::env::var("WA_ENDPOINT").expect("Fail to find WA_ENDPOINT from environment variable");
    let wa_id = std::env::var("WA_ID").expect("Fail to find WA_ID from environment variable");
    let wa_api_key = std::env::var("WA_APIKEY").expect("Fail to find WA_APIKEY from environment variable");
    let wa_version = std::env::var("WA_VERSION").expect("Fail to find WA_VERSION from environment variable");
    let wa_retry = std::env::var("WA_RETRY").map_or(1usize, |r| r.parse::<usize>().expect("WA_RETRY shall be numeric"));
    // println!("{:?}", api_key.unwrap_or(("APIKEY".to_owned(),"Undefined".to_owned())));

    let args = env::args().collect::<Vec<String>>();

    if args.len() == 2 {
        let mut params: Params = serde_json::from_str(&args[1]).expect("Missing one or more parameters.");
        futures::executor::block_on(async move {
            if params.message.trim().len() > 0 {
                if params.source_lang != params.target_lang {
                    for attempt in 0..=wlt_retry {
                        println!("Attempting {} for WLT from {} to {}", attempt + 1, params.source_lang, params.target_lang);
                        let input = &[params.message.as_str()];
                        let request = wlt::WLTTranslationRequest::new(&wlt_endpoint, &wlt_api_key, input, &params.source_lang, &params.target_lang, &wlt_version);
                        let wlt_result = request.send().await;
                        if let Ok(result) = wlt_result {
                            println!("Translate successful in attempt {}, replacing original input message with translated one", attempt + 1);
                            params.message = result.translations[0].translation.to_owned();
                            break;
                        } else {
                            println!("WLT return error");
                        }
                    }
                } else {
                    println!("Source and target language is the same, forward request to WA");
                }
            } else {
                println!("Receive empty message");
            }

            println!("Establishing WA Session");
            let wa_session = match params.session_id {
                Some(id) => wa::WASession::re_attach(wa_endpoint, wa_api_key, wa_id, wa_version, id),
                None => wa::WASession::new(wa_endpoint, wa_api_key, wa_id, wa_version).await.expect("Fail to create new WA session")
            };

            println!("Mapping user input context to WA context");
            
            let mut result = None;
            let context : wa::UnknownType = params.context.unwrap_or(wa::UnknownType::Value(json!({})));

            for attempt in 0..=wa_retry {
                println!("Attempting to send WA message for {} try", attempt + 1);
                if let Ok(r) = wa_session.send_txt_with_context(&params.message, context.clone()).await {
                    println!("WA successfully return response");
                    result = Some(r);
                    break;
                } else {
                    println!("Fail {} times", attempt + 1);
                }
            }

            if let Some(mut r) = result {
                if params.source_lang != params.target_lang {
                    println!("Extracting result from WA response");
                    let mut translation_batch: Vec<&mut String> = Vec::with_capacity(r.output.generic.len());
                    for response in r.output.generic.iter_mut() {
                        match response.response_type {
                            wa::ResponseType::Text => {
                                translation_batch.push(response.text.as_mut().expect("Missing text from response of type text"));
                            },
                            wa::ResponseType::Suggestion => {
                                translation_batch.push(response.title.as_mut().expect("Missing title for suggestions"));

                                if let Some(ref mut suggestions) = response.suggestions {
                                    for s in suggestions {
                                        translation_batch.push(&mut s.label);
                                    }
                                }
                            },
                            wa::ResponseType::Option => {
                                translation_batch.push(response.title.as_mut().expect("Missing title for suggestions"));

                                if let Some(ref mut options) = response.options {
                                    for o in options {
                                        translation_batch.push(&mut o.label);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    println!("Total text to be translated: {} text", translation_batch.len());
                    if translation_batch.len() > 0 {
                        println!("Sending translation batch to WLT");
                        // Perform batch translation
                        let mut wa_translated: Option<RawResponse<wlt::WLTTranslationResponse>> = None;
                        let to_be_translate = translation_batch.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
                        for attempt in 0..=wlt_retry {
                            if let Ok(t) = wlt::WLTTranslationRequest::new(
                                    &wlt_endpoint, 
                                    &wlt_api_key, 
                                    to_be_translate.as_slice(), 
                                    &params.target_lang, 
                                    &params.source_lang, 
                                    &wlt_version).send().await {
                                
                                println!("WLT return {} text", t.translations.len());
                                wa_translated = Some(t);
                            } else {
                                println!("Failed to translate WA response for {} time(s)", attempt + 1);
                            }
                        }
                        // replace original wa response text with translated text
                        if let Some(t) = wa_translated {
                            // consume translation and move the translated value in place of original
                            t.into_inner().1.translations.into_iter().zip(translation_batch.into_iter()).for_each(|(translated, original)| {
                                *original = translated.translation;
                            });
                        }
                    }
                }
                println!("{{\"status\": 200, \"sessionId\": \"{}\", \"result\": {}}}", wa_session.session_id, serde_json::to_string(&*r).expect("Fail to convert result object to JSON"));
            } else {
                println!("{}", "{\"status\": 400}");
            }
        });
    } else {
        println!("{}", "{}");
    }
}
