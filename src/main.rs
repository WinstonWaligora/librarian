use std::io::{self, Write};

mod library;
mod proto;
mod search;
mod utils;

use library::{load_library, save_library};
use proto::{Document, Library};
use search::{create_inverted_index, load_thesaurus, search_with_synonyms};
use utils::{display_full_document, display_results};

fn main() -> io::Result<()> {
    let mut library = generate_demo_data();
    save_library(&library);
    library = load_library();

    let thesaurus = load_thesaurus()?;
    let index = create_inverted_index(&library);

    print!("Enter your search query: ");
    io::stdout().flush().unwrap();
    let mut query = String::new();
    io::stdin().read_line(&mut query).unwrap();
    let query = query.trim();

    let binding = vec![];
    let results = search_with_synonyms(&index, query, &thesaurus);
    let synonyms = thesaurus.get(query).unwrap_or(&binding);
    let snippet_map = display_results(&library, results, query, synonyms);

    print!("Enter the number of the document you want to view in full: ");
    io::stdout().flush().unwrap();
    let mut selection = String::new();
    io::stdin().read_line(&mut selection).unwrap();
    let selection: usize = selection.trim().parse().expect("Invalid input");

    if let Some((doc_id, _)) = snippet_map.get(&selection) {
        display_full_document(&library, *doc_id, query, &synonyms);
    } else {
        println!("Invalid selection.");
    }

    Ok(())
}

fn generate_demo_data() -> Library {
    let mut library = Library::new();

    // Add Narnia chapters
    let narnia_chapters = vec![
        ("Chapter 1", "Once there were four children whose names were Peter, Susan, Edmund, and Lucy."),
        ("Chapter 2", "The children were sent to the house of an old Professor who lived in the heart of the country."),
	("Chapter 5", "The white queen of Narnia gave Edmond a a bunch of delicious treats and a ring."),
    ];
    add_documents(&mut library, "Narnia", narnia_chapters);

    // Add Lord of the Rings chapters
    let lotr_chapters = vec![
        ("Chapter 1", "When Mr. Bilbo Baggins of Bag End announced that he would shortly be celebrating his eleventy-first birthday."),
        ("Chapter 2", "The Fellowship of the Ring sets out on their journey to destroy the One Ring."),
	("Chapter 12", "Bilbo Baggins had to throw the ring into the lava pool at the top of the mountain resisting its temptation."),
    ];
    add_documents(&mut library, "Lord of the Rings", lotr_chapters);

    // Add Bible chapters
    let bible_chapters = vec![
        ("Genesis 1", "In the beginning God created the heavens and the earth."),
        ("Exodus 20", "And God spoke all these words, saying, I am the Lord your God, who brought you out of the land of Egypt."),
	("Numbers 6ish", "A free will offering of a gold ring weighing ten shekels of pure gold."),
    ];
    add_documents(&mut library, "Bible", bible_chapters);

    library
}

fn add_documents(library: &mut Library, subject: &str, chapters: Vec<(&str, &str)>) {
    for (name, content) in chapters {
        let mut doc = Document::new();
        doc.set_name(name.to_string());
        doc.set_subject(subject.to_string());
        doc.set_content(content.to_string());
        library.mut_documents().push(doc);
    }
}
