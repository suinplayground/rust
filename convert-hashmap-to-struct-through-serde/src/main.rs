use std::collections::BTreeMap;

use serde::Deserialize;
use serde::Serialize;

use crate::hashmapstruct::to_struct;
use crate::hashmapstruct::ConversionError;
use crate::k8s_openapi::ByteString;

/// serdeでJSON経由でhashmapをstructに変換する小技
fn main() {
    // 元データ
    let mut map: BTreeMap<String, ByteString> = BTreeMap::new();
    map.insert("id".to_string(), ByteString("1".as_bytes().to_vec()));
    map.insert(
        "title".to_string(),
        ByteString("Title-1".as_bytes().to_vec()),
    );
    // 構造体へ変換
    let book_data: BookData = to_struct(map).unwrap();
    dbg!(&book_data);
    let book_model: BookModel = book_data.try_into().unwrap();
    dbg!(book_model);

    // 元データで、フィールドが足りてない場合
    let mut map: BTreeMap<String, ByteString> = BTreeMap::new();
    map.insert("id".to_string(), ByteString("1".as_bytes().to_vec()));
    // 変換
    let error: ConversionError = to_struct::<BookData>(map).unwrap_err();
    dbg!(error);
    //=> "missing field `title` at line 1 column 10"
}

#[derive(Debug, Serialize, Deserialize)]
struct BookData {
    id: String,
    title: String,
}

#[derive(Debug)]
struct BookModel {
    id: i64,
    title: String,
}

impl TryFrom<BookData> for BookModel {
    type Error = std::num::ParseIntError;

    fn try_from(data: BookData) -> Result<Self, Self::Error> {
        let id = data.id.parse::<i64>()?;
        Ok(BookModel {
            id,
            title: data.title,
        })
    }
}

// 以下汎用的なやつ
mod hashmapstruct {
    use std::collections::BTreeMap;

    use super::k8s_openapi::ByteString;

    pub fn to_struct<T>(data: BTreeMap<String, ByteString>) -> Result<T, ConversionError>
    where
        T: serde::de::DeserializeOwned,
    {
        Ok(btree_map_to_struct(to_serializable_map(data)?)?)
    }

    fn to_serializable_map(
        map: BTreeMap<String, ByteString>,
    ) -> Result<BTreeMap<String, String>, ConversionError> {
        let mut new_map = BTreeMap::new();
        for (key, value) in map {
            let value = String::from_utf8(value.0).map_err(to_error)?;
            new_map.insert(key, value);
        }
        Ok(new_map)
    }

    fn btree_map_to_struct<T>(map: BTreeMap<String, String>) -> Result<T, ConversionError>
    where
        T: serde::de::DeserializeOwned,
    {
        let json = serde_json::to_string(&map).map_err(to_error)?;
        let struct_: T = serde_json::from_str(&json).map_err(to_error)?;
        Ok(struct_)
    }

    #[derive(Debug)]
    pub struct ConversionError(String);

    fn to_error(error: impl ToString) -> ConversionError {
        ConversionError(error.to_string())
    }
}

mod k8s_openapi {
    pub struct ByteString(pub Vec<u8>); // 本当はk8s-openapiのByteStringを使う
}
