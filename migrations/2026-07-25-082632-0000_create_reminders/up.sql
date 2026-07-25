CREATE TABLE reminders (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    subscription_id INTEGER NOT NULL,
    days_before INTEGER NOT NULL,
    reminder_time TIME NOT NULL,
    FOREIGN KEY (subscription_id) REFERENCES subscriptions (id) ON DELETE CASCADE
);
