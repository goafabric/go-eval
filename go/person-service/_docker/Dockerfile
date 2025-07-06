FROM alpine:3.17.0

WORKDIR /app

ADD ./static ./static
ADD ./configs ./configs

COPY person-service ./
CMD ./person-service

EXPOSE 50800
