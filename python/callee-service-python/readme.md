PYTHONPATH=src uv run uvicorn callee_service.main:app --host 0.0.0.0 --port 50900 --reload

docker build -t goafabric/callee-service-python:1.0.0-SNAPSHOT . && docker push goafabric/callee-service-python:1.0.0-SNAPSHOT

docker run --name callee-service --rm -p 50900:50900 goafabric/callee-service-python:1.0.0-SNAPSHOT

# python scaling
- a single python worker leads to: 66 MB / 100% CPU / 1804 req/s
- spawning "--workers 4" workers" leads to: 290 MB / 400% CPU / 4500 req/s
=> so while memory and cpu is just 4*, the req/s do not scale the same way

- so preferably kubernetes replicasets should be used, which consumes the same amount resources, with better isolation and scaling
- however for agents this will require to put the chat memory to redis, via a session id

- the underlying tech is called GIL, which is basicaly a single thread that can spawn processes async to for non blocking ops (similar to reactive in java)
- the code is mostly interpreted, unless delegated to a c library (parts of uv*) 
- newer versions of python allow to disable GIL and allow for real multi threading, as an experimental feature

