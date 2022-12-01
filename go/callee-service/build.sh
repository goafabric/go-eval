#!/bin/bash
GOOS=linux GOARCH=arm64 go build .
docker build --tag callee-service-go:latest .
rm ./callee-service
docker run --rm --name callee-service-go -p50900:50900 callee-service-go
