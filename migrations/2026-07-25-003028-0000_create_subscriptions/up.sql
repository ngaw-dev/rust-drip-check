CREATE TABLE subscriptions (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    title TEXT NOT NULL,
    price INTEGER NOT NULL,
    duration TEXT NOT NULL,
    start_date DATE NOT NULL
);
