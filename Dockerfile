FROM rust

# copiar archivos
COPY ./ ./

#build
RUN cargo build --release

#ejecucion
CMD ["./target/release/secret-messages"]