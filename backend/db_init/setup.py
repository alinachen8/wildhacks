import types
import os
import psycopg2

conn = psycopg2.connect(os.environ.get("DB_URL"))
print(conn)
cur = conn.cursor()

### SET UP OUR TABLES
cur.execute("""
    CREATE TABLE IF NOT EXISTS users (
        id serial primary key,
        name text
    );
""")

conn.commit()

cur.execute("""
    CREATE TABLE IF NOT EXISTS listings (
        id serial primary key,
        name text,
        price float,
        interest float,
        image_url text
    );
""")

conn.commit()

"""
USERS-LISTING RELATIONSHIP TABLE
(user_id: 32, listing_id: i32, value: i32)
"""

cur.execute("""
    CREATE TABLE IF NOT EXISTS portfolio (
        user_id int references users(id),
        listing_id int references listings(id),
        quantity float
    );
""")


conn.commit()
cur.close()
conn.close()