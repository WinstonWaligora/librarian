use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

use rayon::prelude::*;

use crate::proto::Library;
use crate::utils::tokenize;

const THESAURUS_FILE: &str = "thesaurus.txt";

pub fn create_inverted_index(library: &Library) -> HashMap<String, Vec<(usize, usize)>> {
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

pub fn fetch_synonyms(word: &str, thesaurus: &HashMap<String, Vec<String>>) -> Vec<String> {
    thesaurus
        .get(word)
        .cloned()
        .unwrap_or_else(|| vec![word.to_string()])
}

pub fn load_thesaurus() -> io::Result<HashMap<String, Vec<String>>> {
    let mut thesaurus = HashMap::new();
    let file = File::open(THESAURUS_FILE)?;
    for line in io::BufReader::new(file).lines() {
        let line = line?;
        let mut parts = line.split(',');
        if let Some(word) = parts.next() {
            thesaurus.insert(word.to_string(), parts.map(|s| s.to_string()).collect());
        }
    }
    Ok(thesaurus)
}

pub fn parallel_search(
    index: &HashMap<String, Vec<(usize, usize)>>,
    query: &str,
) -> Vec<(usize, usize)> {
    static EMPTY_VEC: Vec<(usize, usize)> = vec![];
    tokenize(query)
        .par_iter()
        .flat_map(|word| index.get(word).unwrap_or(&EMPTY_VEC))
        .cloned()
        .collect()
}

pub fn search_with_synonyms(
    index: &HashMap<String, Vec<(usize, usize)>>,
    query: &str,
    thesaurus: &HashMap<String, Vec<String>>,
) -> Vec<(usize, usize)> {
    let synonyms = fetch_synonyms(query, thesaurus);
    let mut results = search(index, query);
    let mut seen = HashSet::new();
    seen.extend(results.iter().cloned());

    for synonym in synonyms {
        let syn_results = search(index, &synonym);
        for res in syn_results {
            if !seen.contains(&res) {
                results.push(res);
                seen.insert(res);
            }
        }
    }
    results
}

pub fn search(index: &HashMap<String, Vec<(usize, usize)>>, query: &str) -> Vec<(usize, usize)> {
    static EMPTY_VEC: Vec<(usize, usize)> = vec![];
    tokenize(query)
        .iter()
        .flat_map(|word| index.get(word).unwrap_or(&EMPTY_VEC))
        .cloned()
        .collect()
}
