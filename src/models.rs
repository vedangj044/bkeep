use super::schema::book_table;

#[derive(Debug, Queryable)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub read: bool,
}

#[derive(Insertable)]
#[table_name = "book_table"]
pub struct BookInsert {
    pub title: String,
}
