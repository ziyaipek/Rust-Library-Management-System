# Rust Library Management System

This is a Rust program representing a library management system. It allows users to manage books and magazines separately.

## Features

- Defines an enum `Publication` with two variants: `Book` and `Magazine`.
- Each variant contains different data types:
  - `Book` variant contains `title`, `author`, and `page_count` fields.
  - `Magazine` variant contains `title`, `issue`, and `topic` fields.
- Creates instances of books and magazines and stores them in a `Vec<Publication>` array.
- Provides a function to print each publication differently based on its type:
  - For books: "Book: [title] - Author: [author] - Page Count: [page_count]" format.
  - For magazines: "Magazine: [title] - Issue: [issue] - Topic: [topic]" format.

## Usage

1. Clone the repository:

git clone https://github.com/your_username/library_management_system.git


2. Navigate to the project directory:

cd rust_library_management_system


3. Run the program:

cargo run


## Contribution

Contributions are welcome! Feel free to open an issue or submit a pull request.
