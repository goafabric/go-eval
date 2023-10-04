#!/bin/bash
#GOOS=wasip1 GOARCH=wasm go build -o restservice.wasm restservice.go
docker build -f Dockerfile-restservice --platform wasi/wasm -t goafabric/restservice:1.0.0  .
#docker push goafabric/restservice:1.0.0
docker run --rm -p 8080:8080 --name=restservice --runtime=io.containerd.wasmedge.v1 --platform=wasi/wasm goafabric/restservice:1.0.0