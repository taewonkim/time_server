FROM frolvlad/alpine-glibc:latest
ADD ./target/release/time_server /
ENTRYPOINT ["/time_server"]
