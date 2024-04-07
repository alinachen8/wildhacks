import types
import os
import psycopg2

conn = psycopg2.connect(os.environ.get("DB_URL"))
print(conn)
cur = conn.cursor()

### SET UP OUR TABLES
cur.execute("DROP TABLE portfolio;")
conn.commit()

cur.execute("DROP TABLE users;")
conn.commit()

cur.execute("DROP TABLE listings;")
conn.commit()

cur.close()
conn.close()