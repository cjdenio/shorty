FROM golang:1.16-alpine

WORKDIR /usr/src/app

COPY . .

RUN go get ./...

RUN go get -u github.com/cespare/reflex

EXPOSE 3000

CMD ["reflex", "-s", "--", "go", "run", "./cmd"]