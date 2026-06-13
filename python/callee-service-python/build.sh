#!/bin/bash
docker build -t goafabric/callee-service-python:1.0.1-SNAPSHOT . && docker push goafabric/callee-service-python:1.0.1-SNAPSHOT
docker run --name callee-service --rm -p 50900:50900 goafabric/callee-service-python:1.0.1-SNAPSHOT