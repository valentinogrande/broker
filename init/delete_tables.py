import mysql.connector
import time


max_retries = 10
for i in range(max_retries):
    try:
        conn = mysql.connector.connect(
            host="localhost",
            user="appuser",
            password="apppass",
            database="appdb",
            port=3306
        )
        print("Connected to MySQL")
        break
    except mysql.connector.Error:
        print(f"MySQL not ready, retrying ({i+1}/{max_retries})...")
        time.sleep(2)
else:
    raise Exception("Cannot connect to MySQL after retries")


def delete_tables(cursor):
    print("\ndelteting tables...\n")
    sure = input("are ypu sure? [y/n] ")
    if sure == 'y':
        cursor.execute("SET FOREIGN_KEY_CHECKS = 0;")
        cursor.execute("SHOW TABLES;")
        tables = cursor.fetchall()

        for (table_name,) in tables:
            print(f"Dropping table: {table_name}")
            cursor.execute(f"DROP TABLE IF EXISTS `{table_name}`;")

        cursor.execute("SET FOREIGN_KEY_CHECKS = 1;")

        conn.commit()
        cursor.close()


with conn.cursor() as cursor:
    delete_tables(cursor)

conn.close()
