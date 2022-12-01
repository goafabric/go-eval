FROM alpine:3.17.0

WORKDIR /app

COPY callee-service ./

ADD ./static ./static

CMD ./callee-service

EXPOSE 50900
