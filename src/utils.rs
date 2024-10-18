use std::collections::HashSet;

use regex::Regex;

use crate::proto::Library;

pub fn display_full_document(library: &Library, doc_id: usize, query: &str, synonyms: &[String]) {
    let doc = &library.documents[doc_id];

    // Combine the query and its synonyms, all lowercased for case-insensitive matching
    let mut highlight_terms: Vec<String> = vec![query.to_lowercase()];
    highlight_terms.extend(synonyms.iter().map(|s| s.to_lowercase()));

    // Highlight each term in the document content, case-insensitively and whole word match
    let mut highlighted_content = doc.content.clone();
    for term in &highlight_terms {
        let re = Regex::new(&format!(r"(?i)\b{}\b", regex::escape(term))).unwrap();
        highlighted_content = re
            .replace_all(&highlighted_content, |caps: &regex::Captures| {
                format!("\x1b[31m{}\x1b[0m", &caps[0])
            })
            .to_string();
    }

    println!(
        "Document: {}\nSubject: {}\nContent:\n{}",
        doc.name, doc.subject, highlighted_content
    );
}

pub fn display_results(
    library: &Library,
    results: Vec<(usize, usize)>,
) -> std::collections::HashMap<usize, (usize, String)> {
    let mut grouped: std::collections::HashMap<
        String,
        std::collections::HashMap<String, std::collections::HashSet<String>>,
    > = std::collections::HashMap::new();
    let mut snippet_map = std::collections::HashMap::new();
    let mut counter = 1;

    for (doc_id, pos) in results {
        let doc = &library.documents[doc_id];
        let snippet = extract_snippet(&doc.content, &[pos], 3);

        grouped
            .entry(doc.subject.clone())
            .or_insert_with(std::collections::HashMap::new)
            .entry(doc.name.clone())
            .or_insert_with(std::collections::HashSet::new)
            .insert(snippet.clone());

        if !snippet_map.contains_key(&counter) {
            snippet_map.insert(counter, (doc_id, snippet.clone()));
        }
        counter += 1;
    }

    counter = 1;

    for (subject, documents) in grouped {
        println!("Subject: {}", subject);
        for (doc_name, snippets) in documents {
            println!("{}. Document: {}", counter, doc_name);
            for snippet in snippets {
                println!("   - Snippet: {}", snippet);
            }
            counter += 1;
        }
    }

    snippet_map
}

pub fn extract_snippet(doc: &str, positions: &[usize], context_size: usize) -> String {
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

pub fn tokenize(text: &str) -> Vec<String> {
    let stop_words = vec!["and", "the", "is", "in", "at", "of"];
    let punctuations: &[char] = &['.', ',', ';', ':', '!', '?'];

    text.split_whitespace()
        .flat_map(|word| word.split(punctuations))
        .map(|word| word.to_lowercase())
        .filter(|word| !stop_words.contains(&word.as_str()))
        .collect()
}
