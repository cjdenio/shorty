FROM golang:1.16-alpine

WORKDIR /usr/src/app

COPY . .

RUN go get ./...

RUN go build -o ./dist/app ./cmd

FROM alpine:latest

COPY --from=0 /usr/src/app/dist/app /usr/bin/app

CMD ["app"]