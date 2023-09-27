ARG ALPINE_VERSION=3.17
ARG GOLANG_VERSION=1.21.1-alpine${ALPINE_VERSION}

FROM golang:${GOLANG_VERSION} AS builder

ARG USER_ID=1000
ARG GROUP_ID=1000

RUN go install -a github.com/jsonnet-bundler/jsonnet-bundler/cmd/jb@latest
RUN go install github.com/google/go-jsonnet/cmd/jsonnet@latest

RUN addgroup -S user -g $GROUP_ID
RUN adduser -S -g user -u $USER_ID user


FROM alpine:${ALPINE_VERSION}

COPY --from=builder /go/bin/jsonnet /usr/local/bin/jsonnet
COPY --from=builder /go/bin/jb /usr/local/bin/jb

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

RUN apk update
RUN apk add git

USER user

WORKDIR /grafana

CMD ["tail", "-f", "/dev/null"]
