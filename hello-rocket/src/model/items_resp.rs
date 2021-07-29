/**
 * Created at 2021/7/28 23:11
 *
 * @author Liangcheng Juves
 */
use super::items::Items;

use rocket::serde::ser::{Serialize, SerializeStruct, Serializer};

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
        let mut s = serializer.serialize_struct("ItemResp", 2)?;
        s.serialize_field("flag", &self.flag)?;
        s.serialize_field("data", &self.data)?;
        s.end()
    }
}
