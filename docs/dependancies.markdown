---
title: "Dependenacies"
permalink: /Dependenacies
---

# Pegassas Energy Management System

[Home](https://m30819-2020.github.io/cw-code-t1)

## Consolidated list of Dependancies for this project

### RUST

<details>
<summary> Aggregator </summary>
<br>

``` Rust
[dependencies]
tokio = {version ="1.2.0", features = ["full"]}
futures = "0.3.13"
ssdp-client = "1.0.0"
reqwest = {version = "0.11.1", features = ["json"]}
serde_json = "1.0.64"
influx_db_client = "0.5.0"
serde = {version = "1.0.123", features = ["derive"]}
xmltree = "0.10.2"
postgres = {version = "0.19.0", features = ["with-eui48-0_4"]}
eui48 = "0.4.6"
rustls = "0.19.0"
proptest-derive = "0.2.0"
proptest = "1.0.0"
log = "0.4.14"
env_logger = "0.8.3"
regex = "1.4.5"

```
</summary>
<br>

<details>
<summary> Maintenance </summary>
<br>

``` Rust

[dependencies]
tokio = {version ="1.2.0", features = ["full"]}
futures-util = "0.3.13"
ssdp-client = "1.0.0"
reqwest = {version = "0.11.1", features = ["json"]}
serde_json = "1.0.64"
influx_db_client = "0.5.0"
serde = {version = "1.0.123", features = ["derive"]}
xmltree = "0.10.2"
postgres = "0.19.0"
rustls = "0.19.0"
lettre = "0.10.0-beta.2"
mailin-embedded = "0.6.1"
tokio-postgres = {version="0.5.5", features=["with-eui48-0_4", "with-serde_json-1"]}

```
</summary>
<br>

<details>
<summary>Web Application </summary>
<br>

``` Rust
[dependencies]
actix-web = "3.3.2"
actix-files = "0.5.0"
influx_db_client = "0.3.6"
tokio = {version = "0.2.25", features = ["full"]}
serde_json = "1.0.64"
serde = { version = "1.0", features = ["derive"] }
actix-session = "0.4.0"
bcrypt = "0.9.0"
tokio-postgres = {version="0.5.5", features=["with-eui48-0_4", "with-serde_json-1"]}
chrono = {version = "0.4.19", features = ["serde"]}
serde_postgres = "0.2.0"
eui48 = "0.4.6"

```

<details>
<summary>Web Database API </summary>
<br>

``` Rust
</summary>
<br>

[dependencies]
actix-web = "3.3.2"
actix-files = "0.5.0"
influx_db_client = "0.3.6"
tokio = {version = "0.2.25", features = ["full"]}
serde_json = "1.0.64"
serde = { version = "1.0", features = ["derive"] }
actix-session = "0.4.0"
bcrypt = "0.9.0"
tokio-postgres = {version="0.5.5", features=["with-eui48-0_4", "with-serde_json-1", "with-chrono-0_4"]}
chrono = {version = "0.4.19", features = ["serde"]}
serde_postgres = "0.2.0"
eui48 = "0.4.6"
actix-rt = "2.1.0"
log = "0.4.14"
femme = "1.2.0"
async-log = "2.0.0"

```

</summary>
<br>

<details>
<summary> Java </summary>
<br>

</summary>
<br>
<details>
<summary> C++ </summary>
<br>

</summary>
<br>