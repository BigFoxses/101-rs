//! Adapted from https://github.com/ferrous-systems/teaching-material/blob/main/assignments/serde-lifetimes.adoc

use serde::{Serialize, Deserialize};

/// pretend that we call an API and get a JSON String back
fn fetch_data() -> String {
    String::from(
        r#"
            {
                "id": 1,
                "title": "Hello, Rust"
            }
        "#,
    )
}

#[derive(Debug, Serialize,Deserialize)]
struct BlogPost {
    id: u32,
    title: String,
}

fn main() -> anyhow::Result<()> {
 
    let data = fetch_data();
    let b: BlogPost =serde_json::from_str(&data)?;

    let post: BlogPost = {
                b    };

    println!("deserialized = {:?}", post);

   let post_json: String = serde_json::to_string_pretty(&post)?;
   println!("serialized = {:?}", post_json);
   let config = std::fs::read_to_string("cluster.json")?;
   //let map: = serde_json::from_str(&config)?;
   //println!("cluster info: {:#?}", map);
    Ok(())
}
