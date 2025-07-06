#!/bin/bash
cd ..
GOOS=linux GOARCH=arm64 go build .
docker build -f docker/Dockerfile --tag goafabric/callee-service-go:1.0.1 .
docker push goafabric/callee-service-go:1.0.1
rm ./callee-service
docker run --rm --name callee-service-go -p50900:50900 goafabric/callee-service-go:1.0.1
