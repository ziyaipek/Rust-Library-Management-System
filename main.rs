enum Publication {
    Book(Book),
    Magazine(Magazine),
}

struct Book {
    title: String,
    author: String,
    page_count: u32
}

struct Magazine {
    title: String,
    issue: u32,
    topic: String
}

fn print_publications(publications: Vec<Publication>) {
    for publication in publications {
        match publication {
            Publication::Book(ref book) => {
                println!(
                    "Book: {} author: {}, {} pages",
                    book.title, book.author, book.page_count
                );
            }
            Publication::Magazine(ref magazine) => {
                println!(
                    "Magazine: {} - Issue: {}, Topic: {}",
                    magazine.title, magazine.issue, magazine.topic
                );
            }
        }
    }
}

fn main() {
    let book1 = Book {
        title: "Red Mars".to_string(),
        author: "Kim Stanley Robinson".to_string(),
        page_count: 572,
    };

    let magazine1 = Magazine {
        title: "McKinsey&Company".to_string(),
        issue: 5,
        topic: "What is Web3?".to_string(),
    };

    let publications = vec![Publication::Book(book1), Publication::Magazine(magazine1)];

    print_publications(publications);
}