# original image
FROM rust:latest

ENV LS_OPTIONS='--color=auto'

WORKDIR /root
RUN rustup component add clippy
RUN rustup component add rustfmt

# Download OpenSSL 1.1.1
RUN wget https://github.com/openssl/openssl/archive/refs/tags/OpenSSL_1_1_1w.tar.gz \
    && tar xfvz OpenSSL_1_1_1w.tar.gz \
    && cd openssl-OpenSSL_1_1_1w

# Build and install OpenSSL
WORKDIR /root/openssl-OpenSSL_1_1_1w
RUN ./config --prefix=/etc/ssl enable-tls1 enable-tls1_1 enable-ssl3-method no-shared -DOPENSSL_TLS_SECURITY_LEVEL=0 no-asm \
    && make \
    && make install

ENV OPENSSL_DIR="/etc/ssl" OPENSSL_LIB_DIR="/etc/ssl/lib" \
    PATH="/etc/ssl/bin:$PATH"

WORKDIR /root
RUN echo "export OPENSSL_DIR=\"/etc/ssl\"" >> /root/.bashrc \
    && echo "export OPENSSL_LIB_DIR=\"/etc/ssl/lib\"" >> /root/.bashrc \
    && echo "export PATH=\"/etc/ssl/bin:$PATH\"" >> /root/.bashrc \
    && echo "LS_OPTIONS='--color=auto'" >> /root/.bashrc \
    && echo "alias ls='ls $LS_OPTIONS'" >> /root/.bashrc \
    && echo "alias ll='ls $LS_OPTIONS -l'" >> /root/.bashrc

WORKDIR /
CMD ["/bin/bash"]