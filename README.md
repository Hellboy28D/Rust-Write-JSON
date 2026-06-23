🦀 Rust Write JSON

A beginner-friendly Rust project demonstrating how to create, serialize, and write structured JSON data using serde and serde_json.

This project focuses on converting Rust structs into JSON in a type-safe and efficient way.

⸻

✨ Features

* Create structured Rust data models
* Serialize Rust structs into JSON
* Generate clean formatted JSON output
* Work with nested objects and arrays
* Learn Rust + Serde fundamentals

⸻

📸 What this project does

Converts Rust objects like:

struct Paragraph {
    name: String,
}
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

into JSON output like:

{
  "article": "How to work with JSON in Rust",
  "author": "hellboy",
  "paragraph": [
    {
      "name": "starting sentence"
    },
    {
      "name": "body of the paragraph"
    },
    {
      "name": "end of the paragraph"
    }
  ]
}

⸻

🛠 Tech Stack

* Rust 🦀
* Serde
* serde_json

⸻

📦 Dependencies

Add these to Cargo.toml

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

⸻

▶ Getting Started

Clone the repository:

git clone https://github.com/Hellboy28D/Rust-Write-JSON.git

Move into the project:

cd Rust-Write-JSON

Run the project:

cargo run

⸻

📂 Project Structure

Rust-Write-JSON
│
├── src
│   └── main.rs
│
├── Cargo.toml
├── Cargo.lock
├── README.md
└── .gitignore

⸻

🎯 Learning Goals

This project helped practice:

✅ Rust structs
✅ Serialization
✅ JSON formatting
✅ Nested data structures
✅ Type-safe programming in Rust

⸻

🚀 Future Improvements

* Write JSON to external files
* Add user input support
* Pretty-print JSON output
* Build a CLI version
* Read and write JSON dynamically

⸻

Made with 🦀 + ☕ + curiosity
