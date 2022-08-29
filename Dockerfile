FROM rust:slim
COPY ./target/release/settings-service ./target/release/settings-service
COPY ./wwwroot ./wwwroot 
ENTRYPOINT ["./target/release/settings-service"]