//! [https://docs.rs/mysql/21.0.1]

use mysql::prelude::*;
use mysql::*;
use rocket::serde::ser::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Items {
    id: u32,
    title: Option<String>,
    description: Option<String>,
    url: Option<String>,
    img_url: Option<String>,
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

pub fn read() -> Result<Vec<Items>> {
    let conn_url = String::from_utf8_lossy(include_bytes!("mysql_conn_url"));

    let opts = Opts::from_url(&conn_url)?;
    let pool = Pool::new(opts)?;

    let mut conn = pool.get_conn()?;

    let selected_items = conn.query_map(
        "SELECT id,title,description,url,img_url FROM t_item;",
        |(id, title, description, url, img_url)| Items {
            id,
            title,
            description,
            url,
            img_url,
        },
    )?;

    Ok(selected_items)
}
