//! [https://docs.rs/mysql/21.0.1]

use lazy_static::lazy_static;

use crate::model::items::Items;
use mysql::prelude::*;
use mysql::*;

lazy_static! {
    static ref MYSQL_CONN_POOL: Result<Pool> = {
        let conn_url = String::from_utf8_lossy(include_bytes!("mysql_conn_url"));
        let opts = Opts::from_url(&conn_url)?;
        Pool::new(opts)
    };
}

pub fn read() -> Result<Vec<Items<'static>>> {
    let mut conn = MYSQL_CONN_POOL.as_ref().unwrap().get_conn()?;

    let selected_items = conn.query_map(
        "SELECT id,title,description,url,img_url FROM t_item;",
        |(id, title, description, url, img_url)| Items::new(id, title, description, url, img_url),
    )?;

    Ok(selected_items)
}
