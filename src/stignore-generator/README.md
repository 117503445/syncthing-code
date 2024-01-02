# stignore-generator

docker build -t registry.cn-hangzhou.aliyuncs.com/117503445-mirror/stignore-generator .
docker run --rm -t -v $PWD:/workspace registry.cn-hangzhou.aliyuncs.com/117503445-mirror/stignore-generator

ln -s ~/.docker/syncthing/data/workspace ~/workspace

docker build -t 117503445/stignore-generator-builder -f Dockerfile_build .
docker run --rm -v $(pwd):/app 117503445/stignore-generator-builder
