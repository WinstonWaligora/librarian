use protobuf::Message;
use std::io::Read;
use std::io::{self, Write}; // Import the necessary IO modules
mod document;
use document::{Document, Library};

fn main() {
    let mut library = Library::new(); // Create a new library instance

    for i in 1..=5 {
        // Create 5 sample documents
        let mut doc = Document::new(); // Create a new document instance
        doc.set_name(format!("Document {}", i));
	doc.set_subject(format!("Subject {}", i));
        doc.set_content(format!("This is the content of Document {}.", i));
        library.mut_documents().push(doc); // Add the document to the library
    }

    save_library(&library); // Save the library to a file

    let library = load_library();
    let index = create_inverted_index(&library);

    // Prompt user for query
    print!("Enter your search query: ");
    io::stdout().flush().unwrap();
    let mut query = String::new();
    io::stdin().read_line(&mut query).unwrap();
    let query = query.trim();

    // Search with synonyms
    let results = search_with_synonyms(&index, &library, query);

    // Display results
    display_results(&library, results);
}

fn save_library(library: &Library) {
    let mut file = std::fs::File::create("library.bin").unwrap();
    let bytes = library.write_to_bytes().unwrap();
    file.write_all(&bytes).unwrap();
}

fn load_library() -> Library {
    // Load your library from a file (for example)
    let mut file = std::fs::File::open("library.bin").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    Library::parse_from_bytes(&buffer).unwrap()
}

fn create_inverted_index(library: &Library) -> HashMap<String, Vec<(usize, usize)>> {
    let mut index = HashMap::new();
    for (doc_id, doc) in library.documents.iter().enumerate() {
        for (pos, word) in tokenize(&doc.content).iter().enumerate() {
            index
                .entry(word.clone())
                .or_insert(vec![])
                .push((doc_id, pos));
        }
    }
    index
}

fn search(
    index: &HashMap<String, Vec<(usize, usize)>>,
    library: &Library,
    query: &str,
) -> Vec<(usize, usize)> {
    static EMPTY_VEC: Vec<(usize, usize)> = vec![];
    tokenize(query)
        .iter()
        .flat_map(|word| index.get(word).unwrap_or(&EMPTY_VEC))
        .cloned()
        .collect()
}

fn fetch_synonyms(word: &str) -> Vec<String> {
    let rt = Runtime::new().unwrap();
    rt.block_on(get_synonyms(word))
}

fn search_with_synonyms(
    index: &HashMap<String, Vec<(usize, usize)>>,
    library: &Library,
    query: &str,
) -> Vec<(usize, usize)> {
    let synonyms = fetch_synonyms(query);
    let mut results = search(index, library, query);
    for synonym in synonyms {
        results.extend(search(index, library, &synonym));
    }
    results
}

fn display_results(library: &Library, results: Vec<(usize, usize)>) {
    for (doc_id, pos) in results {
        let doc = &library.documents[doc_id];
        let snippet = extract_snippet(&doc.content, &[pos], 3);
        println!(
            "Document: {}\nSubject: {}\nSnippet: {}\n",
            doc.name, doc.subject, snippet
        );
    }
}

fn tokenize(text: &str) -> Vec<String> {
    let stop_words = vec!["and", "the", "is", "in", "at", "of"];
    text.split_whitespace()
        .map(|word| word.to_lowercase())
        .filter(|word| !stop_words.contains(&word.as_str()))
        .collect()
}
use std::collections::HashMap;

fn extract_snippet(doc: &str, positions: &[usize], context_size: usize) -> String {
    let words: Vec<&str> = doc.split_whitespace().collect();
    positions
        .iter()
        .map(|&pos| {
            let start = pos.saturating_sub(context_size);
            let end = (pos + context_size).min(words.len());
            words[start..end].join(" ")
        })
        .collect::<Vec<String>>()
        .join(" ... ")
}
use rayon::prelude::*;

fn parallel_search(
    index: &HashMap<String, Vec<(usize, usize)>>,
    query: &str,
) -> Vec<(usize, usize)> {
    // Define a static empty vector
    static EMPTY_VEC: Vec<(usize, usize)> = vec![];
    tokenize(query)
        .par_iter()
        .flat_map(|word| index.get(word).unwrap_or(&EMPTY_VEC))
        .cloned()
        .collect()
}
use reqwest::Client;
use serde_json::Value;
use tokio::runtime::Runtime;

async fn get_synonyms(word: &str) -> Vec<String> {
    let client = Client::new();
    let url = format!("https://api.datamuse.com/words?ml={}", word);
    let res = client.get(&url).send().await.unwrap();
    let body = res.text().await.unwrap();
    let json: Value = serde_json::from_str(&body).unwrap();

    json.as_array()
        .unwrap()
        .iter()
        .map(|item| item["word"].as_str().unwrap().to_string())
        .collect()
}
