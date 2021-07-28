// use std::io::Cursor;

// use rocket::http::{ContentType, Status};
// use rocket::request::Request;
// use rocket::response::{self, Responder, Response};

use rocket::serde::ser::*;

// use mysql::serde_json::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Items {
    id: u32,
    title: Option<String>,
    description: Option<String>,
    url: Option<String>,
    img_url: Option<String>,
}

impl Items {
    #[allow(dead_code)]
    pub fn new(
        id: u32,
        title: Option<String>,
        description: Option<String>,
        url: Option<String>,
        img_url: Option<String>,
    ) -> Self {
        Items {
            id,
            title,
            description,
            url,
            img_url,
        }
    }
}

impl<'a> Serialize for Items {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> std::result::Result<
        <S as rocket::serde::Serializer>::Ok,
        <S as rocket::serde::Serializer>::Error,
    >
    where
        S: rocket::serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Items", 5)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("title", &self.title)?;
        s.serialize_field("description", &self.description)?;
        s.serialize_field("url", &self.url)?;
        s.serialize_field("img_url", &self.img_url)?;
        s.end()
    }
}

// impl<'r> Responder<'r, 'static> for Items {
//     fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
//         let json = json!(self).to_string();
//         Response::build()
//             .status(Status::Ok)
//             .header_adjoin(ContentType::JSON)
//             .sized_body(json.len(), Cursor::new(&json))
//             .raw_header_adjoin("Access-Control-Allow-Methods", "GET")
//             .raw_header_adjoin(
//                 "Access-Control-Allow-Origin",
//                 "docs.liangchengj.com,127.0.0.1,localhost",
//             )
//             .ok()
//     }
// }
