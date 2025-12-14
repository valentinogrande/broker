FROM python:3.12-slim

WORKDIR /app

RUN apt-get update && \
    apt-get install -y --no-install-recommends default-mysql-client && \
    rm -rf /var/lib/apt/lists/*


COPY create_database.py .
COPY init.sql .
COPY requeriments.txt .

RUN pip install --upgrade pip
RUN pip install --no-cache-dir -r requeriments.txt

CMD ["python3", "create_database.py", "create_all"]
