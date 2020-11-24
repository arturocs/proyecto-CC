# Este dockerfile está basado en https://github.com/arturocs/crab-iot/blob/master/Dockerfile, 
# Usar Ubuntu 18.04 LTS como imagen base
FROM ubuntu:18.04

# Primero se fijan las variables de entorno necesarias para rustup
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=1.48.0

# 1. Instalar las dependencias de rust y los paquetes necesarios para descargar rustup.
# 2. Descargar rustup para GNU/Linux x86_64, darle permisos de ejecución y ejecutarlo
#    con los argumentos necesarios para instalar la versión 1.48.0 de rustc, rust-std y
#    cargo para GNU/Linux x86_64. Además en lugar de añadir los ejecutables al path, se 
#    utilizarán los directorios especificados mediante las variables de entorno
# 3. Eliminar el instalador de rustup y darle permisos de escritura a todos los usuarios
#    en los directorios de rustup u cargo
# 4. Eliminar paquetes innecesarios y sus dependencias
# 5. Eliminar datos de los paquetes, ya que ocupan bastante y no son necesarios para la imagen

RUN apt-get update; \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    gcc \
    libc6-dev \
    wget \
    make \
    ; \
    wget "https://static.rust-lang.org/rustup/archive/1.22.1/x86_64-unknown-linux-gnu/rustup-init"; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --profile minimal --default-toolchain $RUST_VERSION --default-host x86_64-unknown-linux-gnu; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    apt-get remove -y --auto-remove \
    wget ca-certificates\
    ; \
    rm -rf /var/lib/apt/lists/*;

# Fijar el directorio de trabajo en donde se va a montar el repositorio
WORKDIR /app/test

# Ejcutamos make test cuando se inicie el contenedor
CMD ["make", "test"]


