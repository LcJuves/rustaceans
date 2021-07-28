//! [https://docs.rs/mysql/21.0.1]

use crate::model::items::Items;
use mysql::prelude::*;
use mysql::*;

pub fn read() -> Result<Vec<Items>> {
    let conn_url = String::from_utf8_lossy(include_bytes!("mysql_conn_url"));

    let opts = Opts::from_url(&conn_url)?;
    let pool = Pool::new(opts)?;

    let mut conn = pool.get_conn()?;

    let selected_items = conn.query_map(
        "SELECT id,title,description,url,img_url FROM t_item;",
        |(id, title, description, url, img_url)| Items::new(id, title, description, url, img_url),
    )?;

    Ok(selected_items)
}
