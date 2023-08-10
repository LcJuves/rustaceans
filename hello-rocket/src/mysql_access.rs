//! [https://docs.rs/mysql/24.0.0]

use lazy_static::lazy_static;

use crate::model::items::Items;
use mysql::Error::MySqlError;
use mysql::MySqlError as InnerMySqlError;

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
    let mysql_conn_url_ref = MYSQL_CONN_POOL.as_ref();
    if mysql_conn_url_ref.is_err() {
        return Err(MySqlError(InnerMySqlError {
            state: String::from("MySQL Connection pool"),
            code: 1u16,
            message: String::from("setup with error"),
        }));
    }

    let mut conn = mysql_conn_url_ref.unwrap().get_conn()?;

    let selected_items = conn.query_map(
        "SELECT id,title,description,url,img_url FROM t_item;",
        |(id, title, description, url, img_url)| Items::new(id, title, description, url, img_url),
    )?;

    Ok(selected_items)
}
