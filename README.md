# Scaramanga

![ci](https://github.com/shempignon/scaramanga/workflows/ci/badge.svg)

[Pacman Mirrorlist generator](https://www.archlinux.org/mirrorlist/) wrapper written in Rust

## Motivation

Slow mirrors are Evil!

This binary aims at automating the process of refreshing your Pacman mirrorlist.

## Installation

Available as an [AUR package](https://aur.archlinux.org/packages/scaramanga/), so either clone the package and install it with `makepkg -si` or use [aur package manager](https://wiki.archlinux.org/index.php/AUR_helpers)

## Configuration

The configuration file is located at `/etc/scaramanga/config.toml`

You can configure wether to use `http` and/or `https`, `ipv4` and/or `ipv6` and a list of `countries`

## Usage

The binary will automatically rank the mirrors by speed.

Running `scaramanga` will print a speed sorted mirrors list.

In order to replace the content of your mirrorlist run:

```
scaramanga > /etc/pacman.d/mirrorlist
```
