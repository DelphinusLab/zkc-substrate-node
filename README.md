# Delphinus Node

see www.delphinuslab.com for details

# Running

We use [buildkit](https://docs.docker.com/develop/develop-images/build_enhancements/) to facilitate docker image building. Cargo caches are stored locally. So that we can incrementally build docker images.

```
DOCKER_BUILDKIT=1 docker-compose up
```
