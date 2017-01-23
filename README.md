[![Travis](https://travis-ci.org/mongodb-labs/mongo-rust-driver-prototype.svg)](https://travis-ci.org/mongodb-labs/mongo-rust-driver-prototype) [![Crates.io](https://img.shields.io/crates/v/mongodb.svg)](https://crates.io/crates/mongodb) [![docs.rs](https://docs.rs/mongodb/badge.svg)](https://docs.rs/mongodb) [![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)

MongoDB Rust Driver Prototype
=============================

This branch contains active development on a new driver written for Rust 1.x and MongoDB 3.0.x.

The API and implementation are currently subject to change at any time. You should not use this driver in production as it is still under development and is in no way supported by MongoDB Inc. We absolutely encourage you to experiment with it and provide us feedback on the API, design, and implementation. Bug reports and suggestions for improvements are welcomed, as are pull requests.

**Note**: This driver currently only supports MongoDB 3.0.x and 3.2.x. This driver is **not** expected to work with MongoDB 2.6 or any earlier versions. Do not use this driver if you need support for other versions of MongoDB.

Installation
------------

#### Dependencies

-	[Rust 1.7+ with Cargo](http://rust-lang.org)

#### Importing

The 1.0 driver is available on crates.io. To use the MongoDB driver in your code, add the bson and mongodb packages to your `Cargo.toml`:

```
[dependencies]
bson = "0.3.1"
mongodb = "0.1.8"
```

Then, import the bson and driver libraries within your code.

```rust
#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;
```

Examples
--------

Here's a basic example of driver usage:

```rust
use bson::Bson;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

fn main() {
    let client = Client::connect("localhost", 27017)
        .ok().expect("Failed to initialize standalone client.");

    let coll = client.db("test").collection("movies");

    let doc = doc! { "title" => "Jaws",
                      "array" => [ 1, 2, 3 ] };

    // Insert document into 'test.movies' collection
    coll.insert_one(doc.clone(), None)
        .ok().expect("Failed to insert document.");

    // Find the document and receive a cursor
    let mut cursor = coll.find(Some(doc.clone()), None)
        .ok().expect("Failed to execute find.");

    let item = cursor.next();

    // cursor.next() returns an Option<Result<Document>>
    match item {
        Some(Ok(doc)) => match doc.get("title") {
            Some(&Bson::String(ref title)) => println!("{}", title),
            _ => panic!("Expected title to be a string!"),
        },
        Some(Err(_)) => panic!("Failed to get next from server!"),
        None => panic!("Server returned no results!"),
    }
}
```

Testing
-------

The driver test suite is largely composed of a mixture of integration tests and behavioral unit-tests, relying on the official [MongoDB specifications repo](https://github.com/mongodb/specifications). The easiest way to thoroughly test the driver is to set your fork up with TravisCI.

To test the driver locally, you'll need to setup integration tests and specification tests.

> NOTE: Each integration test uses a unique database/collection to allow tests to be parallelized, and will drop their dependencies before running. However, effects are _not_ cleaned up afterwards.

#### Setting up integration tests

All integration tests run on the default MongoDB port, 27017. Before running the tests, ensure that you setup a test database on that port.

Download a version of MongoDB:
```
wget http://fastdl.mongodb.org/linux/mongodb-linux-x86_64-3.2.11.tgz
tar xvf mongodb-linux-x86_64-3.2.11.tgz
mv mongodb-linux-x86_64-3.2.11 3.2.11
```

Run a MongoDB server on 27017:
```
mkdir -p ./data/test_db_3_2
3.2.11/bin/mongod --fork --nopreallocj --dbpath ./data/test_db_3_2 --syslog --port 27017
```

See a full list of versions being tested on Travis in [the travis config](/.travis.yml).

#### Setting up the specifications submodule

Pull in the specifications submodule at `tests/json/data/specs`.

```
git submodule update --init
```

#### Running Tests

Run tests like a regular Rust program:

```
cargo test --verbose
```
