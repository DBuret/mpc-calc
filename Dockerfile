# --- Étape 1 : Build ---
FROM rust:1.85-slim AS builder

RUN apt-get update && apt-get install -y \
    musl-tools \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

# --- Étape 2 : Runtime final ---
FROM scratch

# --- LABEL OCI STANDARDS ---
# Informations de base
LABEL org.opencontainers.image.title="MCP Calc Server"
LABEL org.opencontainers.image.description="High-performance SSE MCP server for arithmetic expression evaluation. Offloads arithmetic from LLMs to a deterministic Rust engine."
LABEL org.opencontainers.image.vendor="DBuret"
LABEL org.opencontainers.image.authors="DBuret"

# Liens et documentation
LABEL org.opencontainers.image.url="https://github.com/DBuret/mcp-calc"
LABEL org.opencontainers.image.source="https://github.com/DBuret/mcp-calc"
LABEL org.opencontainers.image.documentation="https://github.com/DBuret/mcp-calc/blob/main/README.adoc"

# Versioning (à mettre à jour à chaque release)
LABEL org.opencontainers.image.version="0.1.0"
LABEL org.opencontainers.image.revision="HEAD"

# Licensing
LABEL org.opencontainers.image.licenses="MIT"

# Spécificités Runtime
LABEL com.paitrimony.mcp.protocol_version="2024-11-05"
LABEL com.paitrimony.mcp.transport="sse"
LABEL com.paitrimony.mcp.tools="evaluate"

# Certificats SSL (nécessaires pour les sockets TCP Axum/Tokio)
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

# Binaire compilé statiquement
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/mcp-calc /app/mcp-calc

# Variables d'environnement par défaut
ENV MCP_CALC_LOG="info"
ENV MCP_CALC_PORT="3000"

WORKDIR /app
EXPOSE 3000
USER 1000

ENTRYPOINT ["/app/mcp-calc"]
