# Building Lambda functions with crowbar in Docker

The `Dockerfile` and `build.sh` script here help you build Lambda functions against libraries in Amazon Linux (used for the Lambda execution environment).

It's automatically built on Docker Hub as [ilianaw/crowbar-builder](https://hub.docker.com/r/ilianaw/crowbar-builder/) and can be invoked like this from your source tree:

```bash
docker run --rm -v $(pwd):/code:ro ilianaw/crowbar-builder > lambda.zip
```

If you need extra packages, add them as arguments:

```bash
docker run --rm -v $(pwd):/code:ro ilianaw/crowbar-builder openssl-devel > lambda.zip
```

If you need to build with a nightly compiler set the `TOOLCHAIN` environment variable:

```bash
docker run --rm -v $(pwd):/code:ro -e TOOLCHAIN=nightly ilianaw/crowbar-builder > lambda.zip
```
