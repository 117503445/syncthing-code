# stignore-generator

Describe your project here.

docker build -t 117503445/stignore-generator .
docker run --rm -v $PWD:/workspace 117503445/stignore-generator > .stignore

ln -s ~/.docker/syncthing/data/workspace ~/workspace