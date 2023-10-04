#!/bin/bash
#GOOS=wasip1 GOARCH=wasm go build -o saymname.wasm saymname.go
docker build -f Dockerfile-saymyname --platform wasi/wasm -t goafabric/saymname:1.0.0  .
#docker push goafabric/saymname:1.0.0
docker run --rm --name=saymname --runtime=io.containerd.wasmedge.v1 --platform=wasi/wasm goafabric/saymname:1.0.0