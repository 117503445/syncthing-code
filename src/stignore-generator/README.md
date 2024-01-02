# stignore-generator

English | [简体中文](./README_zh-CN.md)

## Development

To start the development environment, navigate to the project root directory and run the following command:

```bash
docker compose up -d
```

Use VSCode to code within the development container.

## Build Docker Image

```bash
# build image
docker build -t 117503445/stignore-generator .
# run container
docker run --rm -t -v $PWD:/workspace 117503445/stignore-generator
```

## Build Executable

```bash
docker build -t 117503445/stignore-generator-builder -f Dockerfile_build .
docker run --rm -v $(pwd):/app 117503445/stignore-generator-builder
```
