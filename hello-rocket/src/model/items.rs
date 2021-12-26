use rocket::serde::ser::{Serialize, SerializeStruct, Serializer};
use std::marker::PhantomData;

#[derive(Debug, PartialEq, Eq)]
pub struct Items<'a> {
    id: u32,
    title: Option<String>,
    description: Option<String>,
    url: Option<String>,
    img_url: Option<String>,
    lifetime: PhantomData<&'a ()>,
}

impl<'a> Items<'a> {
    #[allow(dead_code)]
    pub const fn new(
        id: u32,
        title: Option<String>,
        description: Option<String>,
        url: Option<String>,
        img_url: Option<String>,
    ) -> Self {
        Items { id, title, description, url, img_url, lifetime: PhantomData }
    }
}

impl<'a> Serialize for Items<'a> {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> std::result::Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Items", 5)?;
        // s.serialize_field("id", &self.id)?;
        s.serialize_field("title", &self.title)?;
        // s.serialize_field("desc", &self.description)?;
        s.serialize_field("url", &self.url)?;
        s.serialize_field("imgUrl", &self.img_url)?;
        s.end()
    }
}
