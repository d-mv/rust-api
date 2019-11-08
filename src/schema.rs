// infer_schema!("dotenv:DATABASE_URL"); // might not compile
table! {
    books (id) {
        id -> Int4,
        title -> Varchar,
        author -> Varchar,
        published -> Bool,
    }
}
