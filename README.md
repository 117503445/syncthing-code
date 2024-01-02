# syncthing-code

English | [简体中文](./README_zh-CN.md)

> Sync code across multiple devices, based on Syncthing

## Motivation

I often switch between different desktop and laptop computers. The simplest solution to keep the code files consistent is to use a PC as a development server and connect to it when I work on other devices. However, this solution has the following disadvantages:

- Internet connection may be unstable, leading to frequent disconnections.
- Some development work cannot be done on a remote server, such as embedded systems and GPU computing.
- Latency can result in a degraded coding experience.

An alternative solution is to set up the development environment on each device and synchronize the code between devices. A common code synchronization approach is to commit to a remote repository using Git and then pull on another device. However, this requires manual commits by the programmer, and I often forget to commit, leaving uncommitted code on a powered-off desktop computer.

Automatic synchronization between devices can be achieved using file synchronization services. Among them, [Syncthing](https://syncthing.net) is an excellent open-source file synchronization software. However, Syncthing synchronizes all files, and folders like `node_modules` and `target` do not need to be synchronized because they can be regenerated from files such as `package.json` and `Cargo.toml`. Synchronizing these files consumes significant network bandwidth and significantly reduces the performance of Syncthing.

Syncthing supports ignoring files for synchronization using [.stignore](https://docs.syncthing.net/users/ignoring.html). It is possible to consider adding the files ignored in `.gitignore` to `.stignore` to ensure that Syncthing only synchronizes code text files. However, there are syntax differences between `.gitignore` and `.stignore`, so they cannot be used directly. This project bridges this gap by running the `stignore-generator` process on each device. The `stignore-generator` process runs in the background, monitors changes to the `.gitignore` file, and automatically converts it to a `.stignore` file.

## Usage

Assume that the project paths on each device are located under `~/workspace`, such as `~/workspace/project1` and `~/workspace/project2`.

1. Sync the `~/workspace` folder using Syncthing.

2. Run the `stignore-generator` service, which can be done using Docker or Systemd.

### Docker

Prepare the `docker-compose.yml` file:

```yaml
version: "3.9"
services:
  stignore_generator:
    image: 117503445/stignore-generator
    restart: unless-stopped
    volumes:
      - ~/workspace:/workspace
```

Start `stignore-generator`:

```bash
docker compose up -d
```

### Systemd

Download the `stignore-generator` executable:

```bash
curl -L -O $(curl -s https://api.github.com/repos/117503445/syncthing-code/releases/latest | grep "browser_download_url" | cut -d '"' -f 4) && chmod +x stignore-generator && mv stignore-generator /usr/bin
```

Install the `stignore-generator` service:

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

Enable the `stignore-generator` service:

```bash
systemctl enable --now stignore-generator
```

## Notes

The specific generation process is as follows:

1. Ensure that `.stignore` contains `#include .stgitignore`. If `.stignore` does not exist, create it. If `.stignore` exists but does not contain `#include .stgitignore`, add it to the end of the file.

2. Convert the contents of `.gitignore` to `.stgitignore` and save it in the same directory as `.stignore`.

This generation process allows users to add custom ignore rules in `.stignore` without being overwritten by `stignore-generator`. Typically, configuration files containing confidential information are also included in `.gitignore`, and programmers can use the `!` syntax to synchronize these files between their devices.

## Acknowledgments

- [Syncthing and gitignore](https://jupblb.prose.sh/stignore) provided the basic idea for the solution.