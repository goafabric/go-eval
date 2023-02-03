#!/bin/bash
docker buildx build -f docker/Dockerfile-saymyname --platform wasi/wasm32 -t goafabric/saymname:1.0.0  .
docker push goafabric/saymname:1.0.0
docker run --rm --name=saymname --runtime=io.containerd.wasmedge.v1 --platform=wasi/wasm32 goafabric/saymname:1.0.0

#docker buildx build -f docker/Dockerfile-restservice --platform wasi/wasm32 -t goafabric/restservice:1.0.0  .
#docker run --rm -p 8080:8080 --name=restservice --runtime=io.containerd.wasmedge.v1 --platform=wasi/wasm32 goafabric/restservice:1.0.0