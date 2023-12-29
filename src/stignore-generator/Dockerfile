FROM python:3.11-slim

WORKDIR /workspace

COPY requirements-dev.lock pyproject.toml README.md .

RUN pip install -r requirements-dev.lock

COPY . .

ENTRYPOINT ["python", "./src/stignore_generator/generate-stignore.py"]