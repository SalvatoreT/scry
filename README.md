# Scry

Dynamically show where the image is loaded.

![You are roughly here](https://scry.sal.dev/location.png)

## Setup

```shell
echo 'MAPBOX_ACCESS_TOKEN=<access token>' >> .dev.vars
```

```shell
yarn install
```

## Run

```shell
yarn dev
```

## Deploy

```shell
# Only need to do this once
yarn wrangler secret put MAPBOX_ACCESS_TOKEN
```

```shell
yarn deploy
```
