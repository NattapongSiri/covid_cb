//! Helper function to help send REST API
//! 
//! It has two functions.
//! The most common one is [post_json](fn.post_json.html) where 
//! it send HTTP POST request to given url.
//! It take url, apikey, and optional post body.
//! It return [RawResponse](struct.RawResponse.html).
//! The struct implement `Deref` and `Deref_mut` to inner
//! parsed object so it can be directly treat like it's inner object.
//! It also allow to unpack it using [into_inner](struct.RawResponse.html#method.into_inner)
//! to consume it.
//! Another function is [delete](fn.delete.html).
//! It take url and apikey as parameters.
//! 
//! Both function may also return [CurlErr](enum.CurlErr.html) to designate
//! there's something wrong with the operation.
use curl::easy::Easy;
use serde::{Deserialize, Serialize};
use std::io::Read;

#[derive(Debug)]
pub enum CurlErr {
    InvalidUrl,
    InvalidInputData,
    UnexpectedOutputData,
    RequestFail,
    IncompatibleResultData
}

/// Send HTTP Post to given URL using `api_key` as authorization and optional JSON `data` 
/// to be sent as body. It return Result with in following format `(Vec<u8>, R)` where 
/// `Vec<u8>` is raw byte buffer of returned data and `R` is parsed JSON object or it return
/// `CurlErr`
pub fn post_json<I, R>(url: &str, api_key: &str, data: Option<&I>) -> Result<R, CurlErr> where I : Serialize, R: for<'r> Deserialize<'r> + Serialize {
    let mut client = Easy::new();
    if client.url(url).is_err() {
        return  Err(CurlErr::InvalidUrl);
    }

    client.username("apikey").unwrap();
    client.password(api_key).unwrap();
    client.post(true).unwrap();
    let mut buf = Vec::new();
    let mut headers = curl::easy::List::new();
    headers.append("Content-Type: application/json").expect("Cannot set Content-Type: application/json");
    client.http_headers(headers).expect("Fail to add header");
    let input;
    {
        // code block to force transfer lifetime to end before we extract result.
        // otherwise, `buf` will be burrowed by closure on write_function below while
        // we try to deserialize `buf`
        // with this code block, we force transfer to drop before that line.
        let mut transfer = client.transfer();

        if let Err(e) = transfer.write_function(|d| {
            buf.extend_from_slice(d);
            Ok(d.len())
        }) {
            println!("Cannot read data from response with following error:{:?}", e);
            return Err(CurlErr::UnexpectedOutputData);
        }

        // only add read_function if data is not None
        if let Some(d) = data {
            input = serde_json::to_vec(d).expect("Fail to convert JSON to string");
            let mut sending_data = input.as_slice();
            if let Err(e) = transfer.read_function(move |into| {
                Ok(sending_data.read(into).unwrap())
            }) {
                println!("Cannot fill the request buffer with input data. Following error returned:{:?}", e);
                return Err(CurlErr::InvalidInputData);
            }
        }
        
        if let Err(e) = transfer.perform() {
            println!("Fail to send request with following error:{:?}", e);
            return Err(CurlErr::RequestFail);
        }
    }

    match serde_json::from_reader(buf.as_slice()) {
        Ok(result) => Ok(result),
        Err(e) => {
            // run into deserialize issue. Print some info to let user know on
            // what cause the error
            println!("Fail to deserialize data into object: {:?}", e);
            Err(CurlErr::IncompatibleResultData)
        }
    }
}

pub fn delete(url: &str, api_key: &str) -> Result<(), CurlErr> {
    let mut client = Easy::new();
    if client.url(url).is_err() {
        return  Err(CurlErr::InvalidUrl);
    }

    client.username("apikey").unwrap();
    client.password(api_key).unwrap();
    client.custom_request("DELETE").unwrap();

    {
        let mut transfer = client.transfer();

        if transfer.perform().is_err() {
            Err(CurlErr::RequestFail)
        } else {
            Ok(())
        }
    }
}