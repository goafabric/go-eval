#!/bin/bash
container build -t docker.io/goafabric/callee-service-net:1.0.0-SNAPSHOT . && container image push docker.io/goafabric/callee-service-net:1.0.0-SNAPSHOT
#container run --name callee-service --rm -p 50900:50900 goafabric/callee-service-net:1.0.0-SNAPSHOT
container stop --all