FROM rust:1.41-alpine
WORKDIR /opt/server
RUN apk update && \
    apk upgrade
ADD ./target/x86_64-unknown-linux-musl/release/time_server time
ADD ./static static
EXPOSE 5000
ENTRYPOINT ["/opt/server/time"]
#CMD ["/opt/server/time"]
#ENTRYPOINT ["/opt/server/time"]
#CMD ["/opt/server/time"]
