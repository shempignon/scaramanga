# Scaramanga

![ci](https://github.com/shempignon/scaramanga/workflows/ci/badge.svg)

[Pacman Mirrorlist generator](https://www.archlinux.org/mirrorlist/) wrapper written in Rust

## Motivation

Slow mirrors are Evil!

This binary aims at automating the process of refreshing your Pacman mirrorlist, it will back up your previous list as `/etc/pacman.d/mirrorlist-*`


## Usage

- Configure the `config.toml` to match your needs: http, https, ipv4, ipv6 and even a country list which is optional
- Run `scaramanga`


