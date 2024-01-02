# stignore-generator

[English](./README.md) | 简体中文

## 开发

进入项目根目录，运行以下命令启动开发环境

```bash
docker compose up -d
```

使用 VSCode 在开发容器中编码

## 构建 Docker 镜像

```bash
# build image
docker build -t registry.cn-hangzhou.aliyuncs.com/117503445-mirror/stignore-generator .
# run container
docker run --rm -t -v $PWD:/workspace registry.cn-hangzhou.aliyuncs.com/117503445-mirror/stignore-generator
```

## 构建可执行文件

```bash
docker build -t 117503445/stignore-generator-builder -f Dockerfile_build .
docker run --rm -v $(pwd):/app 117503445/stignore-generator-builder
```
