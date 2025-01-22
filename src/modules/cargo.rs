use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    address: Address,
    phones: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Address {
    street: String,
    city: String,
}

pub fn run() {
    let mut doc = json::parse(
        r#"
    {
        "code": 200,
        "success": true,
        "payload": {
            "features": [
                "awesome",
                "easyAPI",
                "lowLearningCurve"
            ]
        }
    }
    "#,
    )
    .expect("parse failed");
    println!("debug: {:?}", doc);
    println!("display: {}", doc);

    let code = doc["code"].as_u32().unwrap_or(0);
    let success = doc["success"].as_bool().unwrap_or(false);

    assert_eq!(code, 200);
    assert!(success);

    {
        let payload = &mut doc["payload"];
        // features here is a reference, and has to be a reference
        // otherwise we would be trying to move a value out of the JSON document
        let features = &mut payload["features"];
        // let mut features = &mut doc["payload"]["features"];
        for v in features.members() {
            println!("{}", v.as_str().unwrap());
        }
        // `push` will fail if the 'features' wasn't an array, hence it return `Result<()>`
        features.push("cargo!").expect("push failed");
    }
    {
        // what if the 'payload' object doesn't have a 'features' key?
        // the 'features' object will be set to Null
        let empty = &mut doc["empty"];
        println!("empty: {:?}", empty);
    }

    let data = object! {
    "name" => "John Doe",
    "age" => 30,
    "numbers" => array![10, 53, 553],
    };
    assert_eq!(
        data.dump(),
        r#"{"name":"John Doe","age":30,"numbers":[10,53,553]}"#
    );
    // There is a downside to using this crate, because of the mismatch between the amorphous
    // dynamically-typed nature of JSON and the structured static nature of Rust
    // (The readme explicitly speaks of 'friction')
    // So if you did want to map JSON to Rust data structures, you would end up doing a lot of checking
    // because you can not assume that the received structure matches your structs

    // For that, a better solution is serde_json where you serialize Rust data structures into JSON
    // and deserialize JSON into Rust.

    let data = r#"{
    "name": "John Doe", "age": 43,
    "address": { "street": "10 Downing Street", "city": "London" },
    "phones": [ "+44 1234567", "+44 2345678" ]
    }"#;
    let p: Person = serde_json::from_str(data).expect("deserialize error");
    println!("Please call {} at the nubmer {}", p.name, p.phones[0]);
    println!("{:?}", p);
}
