use bson;
use bson::{Bson, Document};
use serde_json::Map;
use serde_json::Value as Json;

pub struct Collection {
    pub name: Option<String>,
    pub data: Vec<Document>,
}

pub struct Outcome {
    pub result: Bson,
    pub collection: Option<Collection>,
}

impl Outcome {
    pub fn from_json(object: &Map<String, Json>) -> Result<Outcome, String> {
        let result = match object.get("result") {
            Some(json) => bson::to_bson(json).unwrap(),
            None => Bson::Null,
        };

        let coll_obj = match object.get("collection") {
            Some(&Json::Object(ref obj)) => obj.clone(),
            _ => {
                return Ok(Outcome {
                    result: result,
                    collection: None,
                })
            }
        };

        let name = match coll_obj.get("name") {
            Some(&Json::String(ref s)) => Some(s.clone()),
            _ => None,
        };

        let array = val_or_err!(coll_obj.get("data"),
                               Some(&Json::Array(ref arr)) => arr,
                              "`result` must be an array");

        let mut data = vec![];

        for json in array {
            match bson::to_bson(json).unwrap() {
                Bson::Document(doc) => data.push(doc),
                _ => return Err(String::from("`data` array must contain only objects")),
            }
        }

        let collection = Collection {
            name: name,
            data: data,
        };

        Ok(Outcome {
            result: result,
            collection: Some(collection),
        })
    }
}
