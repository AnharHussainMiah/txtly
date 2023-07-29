# 📨 TXTLY

![Logo](./docs/logo.png)

> a distributed software SMS gateway

![alt Daemon](https://img.shields.io/badge/Type-Daemon-red.svg)
![alt Rust](https://img.shields.io/badge/Language-Rust-orange.svg)
![alt Binary](https://img.shields.io/badge/Architecture-binary-green.svg)
![alt Failed](https://img.shields.io/badge/Failed-👎_0-red.svg)
![alt Passed](https://img.shields.io/badge/Passed-👍_0-green.svg)
![alt Version](https://img.shields.io/badge/version-0.1.0_ALPHA-blue.svg)

## Introduction

Sending text messages also known as SMS _(Short Message Service)_ is still hugely effective and an instant
way to send alerts and notifications to your customers, clients and end users.

There already exists a lot of online cloud services that provide APIs to integrate into and send SMS texts.
However all these services always charge per SMS. While they're mostly competitive typically the cost will be between 2.5 - 3p in the UK.

As a bussiness if you send out a lot of notifications, then this can quickly add up!

At the same time the actual costs of sending SMS from a normal handset is exceptionally cheap, and there are many PAYG _(Pay As You Go)_ text SIM only deals that provide unlimited SMS for a low monthly fee.

The only restrictions that these SIM deals impose are that the messages sent are NOT online marketing SMS's.

For this reason there are many _hardware_ based solutions, that simply have multiple SIM slots. These hardware solutions are very expensive, but do provide a nice API. Other then the high costs of these units they don't scale because they come with fixed SIM slots, in order to scale one would need to buy more units or purchase a bigger unit.

`TXTLY` is thus a software solution to the same problem and is made up of two parts:

* A software service that runs on the server, this is called the `TXTLY Server` and is the gateway that provides the API to connect to.
* An Android app called the `TXTLY Client` that is installed onto any android phone (with a working SIM).

Once the `TXTLY Client` is paired with the `TXTLY Server` it automatically joins a cluster. You can then simply
scale up by adding as many `TXTLY Client` to as many phones as you want. The SMS requests will be automatically distributed across all clients.

The `TXTLY Server` does not require _any_ databases as everything is handled in memory, but it does however
persist the requests to disk so that if the service is restarted it can pick up from where it left off without
losing any of the requests.

Everything is kept `K.I.S.S` and the `TXTLY Server` only requires a single `config` file.

Client phones can freely join and drop from the cluster as the `TXTLY Server` will dynamically scale up or down the cluser in real time.

## Features

[TODO]

## Building

[TODO]

## Installation & Setup

[TODO]

## Version

0.1.0-ALPHA

## Contributing

1. Fork it (<https://github.com/anharhussainmiah/txtly/fork>)
2. Create your feature branch (`git checkout -b my-new-feature`)
3. Commit your changes (`git commit -am 'Add some feature'`)
4. Push to the branch (`git push origin my-new-feature`)
5. Create a new Pull Request

## Contributors

- [anharmiah](https://github.com/anharhussainmiah) Anhar Miah - creator