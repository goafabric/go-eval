PYTHONPATH=src uv run uvicorn callee_service.main:app --host 0.0.0.0 --port 50900 --reload

docker build -t goafabric/callee-service-python:1.0.0-SNAPSHOT . && docker push goafabric/callee-service-python:1.0.0-SNAPSHOT

docker run --name callee-service --rm -p 50900:50900 goafabric/callee-service-python:1.0.0-SNAPSHOT    