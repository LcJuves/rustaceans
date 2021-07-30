#![allow(dead_code)]

/**
 * Created at 2021/7/30 10:28
 * @author Liangcheng Juves
 */

#[derive(Debug)]
pub enum RequestMethod {
    GET(&'static str),
    POST(&'static str),
}

#[derive(Debug)]
pub struct Request {
    pub uri: String,
    pub method: RequestMethod,
    pub version: f32,
}

impl Request {
    pub fn new(uri: String, method: RequestMethod, version: f32) -> Self {
        Request {
            uri,
            method,
            version,
        }
    }
}
