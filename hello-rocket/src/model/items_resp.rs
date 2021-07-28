/**
 * Created at 2021/7/28 23:11
 *
 * @author Liangcheng Juves
 */
use super::items::Items;

use rocket::serde::ser::{Serialize, SerializeStruct, Serializer};

use mysql::serde_json::json;

pub struct ItemResp<'a> {
    flag: u8,
    data: Option<Vec<Items<'a>>>,
}

impl<'a> ItemResp<'a> {
    pub const fn new(flag: u8, data: Option<Vec<Items<'a>>>) -> Self {
        ItemResp { flag, data }
    }
}

impl<'a> Serialize for ItemResp<'a> {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> std::result::Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ItemResp", 5)?;
        s.serialize_field("flag", &self.flag)?;

        let data_json = match &self.data {
            Some(ret) => json!(ret).to_string(),
            None => "[]".to_string(),
        };

        s.serialize_field("data", &data_json)?;
        s.end()
    }
}
