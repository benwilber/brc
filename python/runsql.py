import csv
import sqlite3

CREATE_TABLE_SQL = """
create table measurements (
    id text,
    num real
);
create index idx_id on measurements(id);
"""


def insert_batch(cursor, batch):
    cursor.execute("begin transaction")
    cursor.executemany("insert into measurements (id, num) values (?, ?)", batch)
    cursor.execute("commit")


def main():
    conn = sqlite3.connect(":memory:")
    cursor = conn.cursor()
    cursor.executescript(CREATE_TABLE_SQL)

    with open("measurements.txt") as f:
        reader = csv.reader(f, delimiter=";")
        batch = []

        for i, (id, num) in enumerate(reader, start=1):
            batch.append((id, float(num)))

            if len(batch) == 10_000:
                insert_batch(cursor, batch)
                batch = []

        if batch:
            insert_batch(cursor, batch)


if __name__ == "__main__":
    main()
