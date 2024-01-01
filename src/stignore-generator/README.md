# stignore-generator

docker build -t registry.cn-hangzhou.aliyuncs.com/117503445-mirror/stignore-generator .
docker run --rm -v $PWD:/workspace registry.cn-hangzhou.aliyuncs.com/117503445-mirror/stignore-generator > .stignore

ln -s ~/.docker/syncthing/data/workspace ~/workspace