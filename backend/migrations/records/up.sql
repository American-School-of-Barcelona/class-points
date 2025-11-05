CREATE TABLE records (
    change INTEGER NOT NULL,
    reason TEXT NOT NULL,
    date TEXT NOT NULL,
    student INTEGER NOT NULL,
    points INTEGER NOT NULL,
    FOREIGN KEY (student) REFERENCES students(id)
)