import mysql.connector
import re
import time
import os
from cryptography.hazmat.primitives.asymmetric import ec
from cryptography.hazmat.primitives import serialization


def generate_key_pair():
    try:
        os.remove("/shared/ecc_private_key.pem")
        os.remove("/shared/ecc_public_key.pem")
    except FileNotFoundError:
        pass

    private_key = ec.generate_private_key(ec.SECP256R1())  # P-256

    with open("/shared/ecc_private_key.pem", "wb") as f:
        f.write(private_key.private_bytes(
            encoding=serialization.Encoding.PEM,
            format=serialization.PrivateFormat.PKCS8,
            encryption_algorithm=serialization.NoEncryption()
        ))

    public_key = private_key.public_key()
    with open("/shared/ecc_public_key.pem", "wb") as f:
        f.write(public_key.public_bytes(
            encoding=serialization.Encoding.PEM,
            format=serialization.PublicFormat.SubjectPublicKeyInfo
        ))

    print("\033[92mECC keys were created\033[0m")


max_retries = 10
for i in range(max_retries):
    try:
        conn = mysql.connector.connect(
            host="mysql",
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


def create_tables(cursor, file):
    print("\ncreating tables...\n")
    sql_script = file.read()
    commands = sql_script.split(';')

    for command in commands:
        command = command.strip()
        if command.startswith('--') or not command:
            continue
        try:
            if command.upper().startswith("CREATE TABLE"):
                match = re.search(
                    r'CREATE TABLE\s+(?:IF NOT EXISTS\s+)?`?(\w+)`?',
                    command, re.IGNORECASE
                )
                if match:
                    table = match.group(1)
                    print(f'creating table: {table}')
            cursor.execute(command)
            conn.commit()
        except mysql.connector.Error as err:
            print(f'Error executing command: {err}')


with conn.cursor() as cursor:
    with open("init.sql", "r") as file:
        create_tables(cursor, file)
    generate_key_pair()

conn.close()
