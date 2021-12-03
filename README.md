# Delphinus Node

see www.delphinuslab.com for details

# Running a smallish network for development

We use [buildkit](https://docs.docker.com/develop/develop-images/build_enhancements/) to facilitate docker image building. Cargo caches are stored locally. So that we can incrementally build docker images.

```
DOCKER_BUILDKIT=1 docker build . -t zhenxunge-node
docker-compose -f docker-compose.dev.yml up
```
