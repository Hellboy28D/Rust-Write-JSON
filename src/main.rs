use serde::{Deserialize, Serialize};


#[derive(Serialize,Deserialize)]
struct Paragrapgh{
    name: String
}

#[derive(Serialize,Deserialize)]
struct  Article{
    article:String,
    author:String,
    paragraph:Vec<Paragrapgh>
}
fn main(){
    let article: Article = Article { 
        article:String::from("how to work with json in Rust"), 
        author:String::from("Hellboy"), 
        paragraph:vec![
            Paragrapgh{
                name:String::from("first sentence")
            },
            Paragrapgh{
                name:String::from("body of paragraph") 
            },
            Paragrapgh{
                name:String::from("end of paragraph")
            },
        ]
    };

    let json = serde_json::to_string(&article).unwrap();
    println!("The json is: {}", json)
}