# Fixer
Fixer is the message queue for `Night City`. Instead of building our own Queue from the ground up we use [NATS](https://nats.io/) since it is mature, performant, and easy to setup and integrate with our other services.

## Setup
> Check out the main `Night City Repo` which has commands and docs for deploying a full `Night City` setup including setting up the `Fixer` service.

The easiest way to use this is with `Docker` by running. This exposes the `NATS` ports publicly with the default configurations. Do not use this for production.
```sh
docker run -p 4222:4222 -p 8222:8222 -p 6222:6222 --name fixer -ti nats:latest
```

## NATS
You can read the [NATS docs here](https://docs.nats.io/running-a-nats-service/nats_docker).