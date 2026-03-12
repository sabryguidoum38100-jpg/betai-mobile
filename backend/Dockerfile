# Utilise une image Rust officielle
FROM rust:1.75-slim as builder

# Crée un répertoire de travail
WORKDIR /app

# Copie les fichiers du projet
COPY . .

# Compile l'application
RUN cargo build --release

# Crée une image finale légère
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/betai-backend /app/betai-backend

# Expose le port 8080
EXPOSE 8080

# Commande de démarrage
CMD ["./betai-backend"]
