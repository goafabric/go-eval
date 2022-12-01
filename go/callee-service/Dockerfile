FROM golang:1.16-alpine

WORKDIR /app

COPY callee-service ./

ADD ./static ./static

CMD ./callee-service

EXPOSE 50900
