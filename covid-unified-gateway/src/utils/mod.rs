use curl::easy::Easy;
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub enum CurlErr {
    InvalidUrl,
    InvalidInputData,
    UnexpectedOutputData,
    RequestFail,
    IncompatibleResultData
}

/// Raw Curl response that may be partially parsed. By design, `serde_json::value::RawValue` 
/// represent all unparsed data. It is unsized so it need to be stored as reference.
/// Since reference require owner of original value to be present while processing it,
/// We need higher rank lifetime to reference it own value.
/// 
/// This object implement Deref and DerefMut so it can be treat like inner object.
pub struct RawResponse<T> {
    raw: Vec<u8>,
    pub obj: T
}

impl<T> RawResponse<T> {
    /// Unwrap this object into two primitive.
    /// Normally, this is done only to consume object.
    /// It return `(Vec<u8>, T)` where `Vec<u8>` is a raw byte and
    /// `T` is a partially parsed data where unparsed data will reference
    /// into raw bytes.
    /// It's an error to drop raw bytes before `T`
    pub fn into_inner(self) -> (Vec<u8>, T) {
        (self.raw, self.obj)
    }
}

impl<T> Deref for RawResponse<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.obj
    }
}

impl<T> DerefMut for RawResponse<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.obj
    }
}

/// Send HTTP Post to given URL using `api_key` as authorization and optional JSON `data` 
/// to be sent as body. It return Result with in following format `(Vec<u8>, R)` where 
/// `Vec<u8>` is raw byte buffer of returned data and `R` is parsed JSON object or it return
/// `CurlErr`
pub fn post_json<I, R>(url: &str, api_key: &str, data: Option<&I>) -> Result<RawResponse<R>, CurlErr> where I : Serialize, R: for<'r> Deserialize<'r> + Serialize {
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
        Ok(result) => Ok(RawResponse {raw: buf, obj: result}),
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