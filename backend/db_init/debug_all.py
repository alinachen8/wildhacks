import types
import os
import psycopg2

conn = psycopg2.connect(os.environ.get("DB_URL"))
print(conn)
cur = conn.cursor()

### SET UP OUR TABLES
cur.execute("SELECT * FROM users;")
conn.commit()
print("----+=USERS=+----")
print(cur.fetchall())

print("----+=Listings=+----")
cur.execute("SELECT * FROM listings;")
conn.commit()
print(cur.fetchall())

print("----+=Portfolios=+----")
cur.execute("SELECT * FROM portfolio;")
conn.commit()

print(cur.fetchall())

cur.close()
conn.close()