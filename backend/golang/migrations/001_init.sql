CREATE TABLE IF NOT EXISTS users (
                                     id INTEGER PRIMARY KEY AUTOINCREMENT,
                                     name TEXT NOT NULL,
                                     email TEXT UNIQUE NOT NULL,
                                     password TEXT NOT NULL,
                                     address TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS customers (
                                         user_id INTEGER PRIMARY KEY,
                                         name TEXT NOT NULL,
                                         address TEXT NOT NULL,
                                         phone TEXT,
                                         FOREIGN KEY(user_id) REFERENCES users(id)
);