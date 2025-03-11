# Etapa 1: Construcción del binario
FROM rust:latest  AS builder

# Establecer el directorio de trabajo
WORKDIR /app

# Copiar Cargo.toml y Cargo.lock para aprovechar caché en dependencias
COPY Cargo.toml Cargo.lock ./

# Crear una carpeta "src" temporal para evitar problemas si aún no hay código
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Descargar dependencias y compilarlas (sin compilar el código del proyecto aún)
RUN cargo build --release

# Copiar el código fuente real
COPY . .

# Compilar el binario final en modo release
RUN cargo build --release

# Etapa 2: Crear una imagen más ligera solo con el binario compilado
FROM debian:stable

# Crear el directorio de trabajo
WORKDIR /app

# Copiar el binario desde la imagen "builder"
COPY --from=builder /app/target/release/Dou-Backend /app/

# Exponer el puerto en el que corre la app
EXPOSE 8080

# Ejecutar la aplicación
CMD ["./Dou-Backend"]
