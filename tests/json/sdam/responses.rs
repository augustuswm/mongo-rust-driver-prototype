use bson;
use bson::{Bson, Document};
use mongodb::connstring::{self, Host};
use serde_json::Value as Json;

pub struct Responses {
    pub data: Vec<(Host, Document)>,
}

impl Responses {
    pub fn from_json(array: &[Json]) -> Result<Responses, String> {
        let mut data = Vec::new();

        for json in array {
            let inner_array = val_or_err!(*json,
                                          Json::Array(ref arr) => arr,
                                          "`responses` must be an array of arrays.");

            if inner_array.len() != 2 {
                return Err(String::from("Response item must contain the host string and ismaster object."));
            }

            let host = val_or_err!(
                inner_array[0],
                Json::String(ref s) => s.to_owned(),
                "Response item must contain the host string as the first argument.");

            let ismaster = val_or_err!(
                inner_array[1],
                Json::Object(ref obj) => bson::to_bson(&Json::Object(obj.clone())).unwrap(),
                "Response item must contain the ismaster object as \
                the second argument.");

            match ismaster {
                Bson::Document(doc) => {
                    data.push((connstring::parse_host(&host).unwrap(), doc));
                }
                _ => return Err(String::from("`ismaster` parse must return a Bson Document")),
            }
        }

        Ok(Responses { data: data })
    }
}
