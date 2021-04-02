---
title: "Install"
permalink: /Install-Download
---

# Pegassas Energy Management System

[Home](https://m30819-2020.github.io/cw-code-t1)

## Downloads and Install files

### Download

- The below download links provide an executable to which will install the latest version of the Pegassas Energy Management System.
- Any issues please let the development team know.

[Download](https://github.com/M30819-2020/cw-code-t1/releases)

### Install

This installation can only be used on the below systems:

- Ubuntu
- Debian
- Raspberry Pi OS

1. Download the `.deb` files you want to install from [here](https://github.com/M30819-2020/cw-code-t1/releases)
2. Run `sudo dpkg -i path/to/file.deb` for each
3. Edit configuration files
   1. For Pegassas Web App they are in `/etc/peggassas_webapp/`
   2. For Pegassas Aggregator they are in `/etc/pegassas_aggregator/`
4. Start services
   1. For Pegassas Web App use `sudo service pegassas_webapp start` to run until system shutdown, or use `sudo systemctl enable pegassas_webapp` to set it to run at startup automatically
   2. For Pegassas Aggregator use `sudo service pegassas_aggregator start` to run until system shutdown, or use `sudo systemctl enable pegassas_aggregator` to set it to run at startup automatically
5. Pegassas is now running
