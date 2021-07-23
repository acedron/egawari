FROM archlinux:latest
LABEL maintainer="acedron <acedrons@yahoo.co.jp>"

WORKDIR /app

COPY . .

RUN pacman -Sy --noconfirm rust --needed
RUN cargo build
