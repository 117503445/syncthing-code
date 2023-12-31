FROM python:3.11-slim

RUN apt-get update && apt-get install -y inotify-tools && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY requirements-dev.lock pyproject.toml README.md ./

RUN pip install -r requirements-dev.lock

COPY . .

ENTRYPOINT [ "./watch.sh" ]