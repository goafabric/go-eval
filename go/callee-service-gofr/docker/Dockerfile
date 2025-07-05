FROM alpine:3.17.0

WORKDIR /app

ADD ./static ./static
ADD ./configs ./configs

COPY callee-service ./
CMD ./callee-service

EXPOSE 50900
