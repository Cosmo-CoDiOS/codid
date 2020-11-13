FROM rustembedded/cross:aarch64-unknown-linux-gnu-0.2.1

RUN dpkg --add-architecture arm64 \
    && apt-get update \
    && apt-get install -y aptitude \
    && aptitude install -y libdbus-1-dev:arm64 libdbus-1-dev \
       pkg-config libglib2.0-dev:arm64 libudev-dev:arm64 \
       libglib2.0-dev libudev-dev
