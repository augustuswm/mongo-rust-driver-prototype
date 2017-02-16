use bson;
use bson::Bson;
use json::FromJson;

use mongodb::coll::options::{AggregateOptions, CountOptions, FindOneAndDeleteOptions,
                             FindOneAndUpdateOptions, FindOptions, ReturnDocument};

// use rustc_serialize::json::{Object, Json};
use serde_json::Map;
use serde_json::Value as Json;

impl FromJson for AggregateOptions {
    fn from_json(object: &Map<String, Json>) -> AggregateOptions {
        let mut options = AggregateOptions::new();

        options.batch_size = match object.get("batchSize") {
            Some(&Json::Number(ref n)) => n.as_i64().unwrap() as i32,
            _ => options.batch_size,
        };

        options
    }
}

impl FromJson for CountOptions {
    fn from_json(object: &Map<String, Json>) -> CountOptions {
        let mut options = CountOptions::new();

        options.skip = match object.get("skip") {
            Some(&Json::Number(ref n)) => n.as_i64(),
            _ => options.skip,
        };

        options.limit = match object.get("limit") {
            Some(&Json::Number(ref n)) => n.as_i64(),
            _ => options.limit,
        };

        options
    }
}

impl FromJson for FindOptions {
    fn from_json(object: &Map<String, Json>) -> FindOptions {
        let mut options = FindOptions::new();

        let f = |x| bson::to_bson(x).ok();
        options.sort = match object.get("sort").and_then(f) {
            Some(Bson::Document(doc)) => Some(doc),
            _ => None,
        };

        options.skip = match object.get("skip") {
            Some(&Json::Number(ref n)) => n.as_i64(),
            _ => options.skip,
        };

        options.limit = match object.get("limit") {
            Some(&Json::Number(ref n)) => n.as_i64(),
            _ => options.limit,
        };

        options.batch_size = match object.get("batchSize") {
            Some(&Json::Number(ref n)) => Some(n.as_i64().unwrap() as i32),
            _ => options.batch_size,
        };
        options

    }
}

impl FromJson for FindOneAndDeleteOptions {
    fn from_json(object: &Map<String, Json>) -> FindOneAndDeleteOptions {
        let mut options = FindOneAndDeleteOptions::new();

        if let Some(Bson::Document(projection)) = object.get("projection").map(|x| bson::to_bson(x).unwrap()) {
            options.projection = Some(projection);
        }

        if let Some(Bson::Document(sort)) = object.get("sort").map(|x| bson::to_bson(x).unwrap()) {
            options.sort = Some(sort);
        }

        options
    }
}

impl FromJson for FindOneAndUpdateOptions {
    fn from_json(object: &Map<String, Json>) -> FindOneAndUpdateOptions {
        let mut options = FindOneAndUpdateOptions::new();

        if let Some(Bson::Document(projection)) = object.get("projection").map(|x| bson::to_bson(x).unwrap()) {
            options.projection = Some(projection);
        }

        if let Some(Bson::String(s)) = object.get("returnDocument").map(|x| bson::to_bson(x).unwrap()) {
            match s.as_ref() {
                "After" => options.return_document = Some(ReturnDocument::After),
                "Before" => options.return_document = Some(ReturnDocument::Before),
                _ => {}
            };
        }


        if let Some(Bson::Document(sort)) = object.get("sort").map(|x| bson::to_bson(x).unwrap()) {
            options.sort = Some(sort);
        }

        if let Some(Bson::Boolean(upsert)) = object.get("upsert").map(|x| bson::to_bson(x).unwrap()) {
            options.upsert = Some(upsert);
        }

        options
    }
}
