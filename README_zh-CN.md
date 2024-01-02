# syncthing-code

[English](./README.md) | 简体中文

> 多设备代码同步解决方案，基于 Syncthing

## 动机

我经常在不同台式电脑和笔记本电脑之间切换。为了保持代码文件的一致性，最简单的解决方案是使用一台 PC 作为开发服务器，并在我使用其他设备工作时连接到它。然而，这个方案存在以下缺点：

- 互联网连接可能不稳定，导致经常断开连接
- 部分开发工作无法在远程服务器上完成，比如嵌入式、GPU 计算等
- 延迟会导致编码体验下降

替代方案是在每台设备上设置开发环境，并在设备之间同步代码。常见的代码同步方案是通过 Git 提交到远程仓库，然后在另一台设备上拉取。然而，这要求程序员手动进行提交，而我经常会忘记提交，导致未提交的代码被留在已关机的台式电脑中。

通过同步网盘可以在代码文件变更时自动在设备间同步。其中，[Syncthing](https://syncthing.net) 是一款优秀的开源同步盘软件。但 Syncthing 会同步所有文件，而 `node_modules` 和 `target` 等文件夹是不需要同步的，因为它们可以通过 `package.json` 和 `Cargo.toml` 等文件自动生成。同步这些文件会导致巨大的网络带宽消耗，并显著降低 Syncthing 的性能。

Syncthing 支持使用 [.stignore](https://docs.syncthing.net/users/ignoring.html) 忽略同步文件。可以考虑将 `.gitignore` 中忽略的文件也添加到 `.stignore` 中，这样就可以保证 Syncthing 只同步代码文本文件了。但是，`.gitignore` 和 `.stignore` 的语法存在差异，不能直接使用。本项目可以弥补这个鸿沟，实现思路是在每台设备上运行 `stignore-generator` 进程。`stignore-generator` 进程在后台监听 `.gitignore` 文件的变化，并自动将其转换为 `.stignore` 文件。

## 使用方法

假设每个设备的项目路径都位于 `~/workspace` 下，比如 `~/workspace/project1` 和 `~/workspace/project2`。

1. 使用 Syncthing 同步文件夹 `~/workspace`

2. 运行 `stignore-generator` 服务，可以使用 Docker 和 Systemd 两种方式。

### Docker

准备 `docker-compose.yml` 文件：

```yaml
version: "3.9"
services:
  stignore_generator:
    image: 117503445/stignore-generator
    restart: unless-stopped
    volumes:
      - ~/workspace:/workspace
```

中国用户可以使用以下命令加速 Docker 镜像拉取

```bash
docker pull registry.cn-hangzhou.aliyuncs.com/117503445-mirror/stignore-generator && docker tag registry.cn-hangzhou.aliyuncs.com/117503445-mirror/stignore-generator 117503445/stignore-generator
```

启动 `stignore-generator`：

```bash
docker compose up -d
```

### Systemd

下载 `stignore-generator` 可执行文件

```bash
curl -L -O $(curl -s https://api.github.com/repos/117503445/syncthing-code/releases/latest | grep "browser_download_url" | cut -d '"' -f 4) && chmod +x stignore-generator && mv stignore-generator /usr/bin
```

安装 `stignore-generator` 服务

```bash
cat << EOF > /etc/systemd/system/stignore-generator.service
[Unit]
Description=stignore-generator

[Service]
Type=simple
Restart=always
RestartSec=1
ExecStart=/usr/bin/stignore-generator

[Install]
WantedBy=multi-user.target
EOF
```

启用 `stignore-generator` 服务

```bash
systemctl enable --now stignore-generator
```

## 注意事项

具体的生成方式是

1. 保证 `.stignore` 中有 `#include .stgitignore`。如果 `.stignore` 不存在，则创建它；如果 `.stignore` 存在但不包含 `#include .stgitignore`，则在文件末尾添加它。

2. 将 `.gitignore` 中的内容转换为 `.stgitignore`，并将其保存到 `.stignore` 所在目录下。

这种生成方式允许用户在 `.stignore` 中添加自定义的忽略规则，而不会被 `stignore-generator` 覆盖。典型的，包含机密信息的配置文件也存在于 `.gitignore` 中，程序员可以使用 `!` 语法在自己的设备间同步这些文件。

## 致谢

- [Syncthing and gitignore](https://jupblb.prose.sh/stignore) 提供了解决方案的基本思路
