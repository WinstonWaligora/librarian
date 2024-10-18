use std::collections::HashMap;

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

pub fn display_results(library: &Library, results: Vec<(usize, usize)>) -> HashMap<usize, (usize, Vec<String>)> {
    let mut document_map: HashMap<usize, (usize, Vec<String>)> = HashMap::new();
    let mut doc_id_map: HashMap<usize, usize> = HashMap::new(); // Maps doc_id to counter
    let mut counter = 1;

    // Build the document map and snippets
    for (doc_id, pos) in &results {
        let doc = &library.documents[*doc_id];
        let snippet = extract_snippet(&doc.content, &[*pos], 3);

        if let Some(&num) = doc_id_map.get(doc_id) {
            document_map.get_mut(&num).unwrap().1.push(snippet.clone());
        } else {
            doc_id_map.insert(*doc_id, counter);
            document_map.insert(counter, (*doc_id, vec![snippet.clone()]));
            counter += 1;
        }
    }

    let mut subjects_seen = HashMap::new();

    // Collect documents for sorting
    let mut sorted_documents: Vec<(&usize, &(usize, Vec<String>))> = document_map.iter().collect();
    sorted_documents.sort_by_key(|&(num, _)| num);

    // Display results in ascending order
    for (num, (doc_id, snippets)) in sorted_documents {
        let doc = &library.documents[*doc_id];
        let subject = &doc.subject;
        let doc_name = &doc.name;

        if !subjects_seen.contains_key(subject) {
            println!("Subject: {}", subject);
            subjects_seen.insert(subject, true);
        }

        println!("{}. Document: {}", num, doc_name);
        for snippet in snippets {
            println!("   - Snippet: {}", snippet);
        }
    }

    document_map
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
