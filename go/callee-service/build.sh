#!/bin/bash
GOOS=linux GOARCH=arm64 go build .
docker build --tag goafabric/callee-service-go:1.0.0 .
rm ./callee-service
docker run --rm --name callee-service -p50900:50900 goafabric/callee-service-go:1.0.0
