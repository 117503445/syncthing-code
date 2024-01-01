# syncthing-code

每个机器上都在 ~/workspace 下写代码

使用 syncthing 同步 文件夹 `workspace`

挂载

```sh
ln -s ~/.docker/syncthing/data/workspace ~/workspace
```

启动服务

```yaml
version: "3.9"
services:
  stignore_generator:
    image: 117503445/stignore-generator
    restart: unless-stopped
    volumes:
      - ~/workspace:/workspace
```
