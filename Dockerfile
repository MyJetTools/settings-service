FROM ubuntu:22.04
COPY ./target/release/settings-service ./target/release/settings-service
COPY ./wwwroot ./wwwroot 
ENTRYPOINT ["./target/release/settings-service"]