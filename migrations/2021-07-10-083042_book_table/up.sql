-- Your SQL goes here
CREATE TABLE book_table (
    id INT NOT NULL AUTO_INCREMENT,
    title VARCHAR(255) NOT NULL,
    done BOOLEAN NOT NULL DEFAULT 0,
    PRIMARY KEY (id)
);