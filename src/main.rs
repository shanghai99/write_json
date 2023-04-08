use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>
}

fn main() {
    let article: Article = Article {
        article: String::from("How to Work with JSON in Rust"),
        author: String::from("Frank the Rabbit"),
        paragraph: vec![
            Paragraph {
                name: String::from("The first sentence is first.")
            },
            Paragraph {
                name: String::from("The body of the paragraph comes next.")
            },
            Paragraph {
                name: String::from("The last sentence is at the end of the paragraph.")
            }
        ]
    };

    let json = serde_json::to_string(&article).unwrap();
    println!("The JSON is: {}", json);
}