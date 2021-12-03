# Delphinus Node

see www.delphinuslab.com for details

# Running and Deployment
We use [buildkit](https://docs.docker.com/develop/develop-images/build_enhancements/) to facilitate docker image building. Cargo caches are stored locally. So that we can incrementally build docker images.

## Deploy node to remote host

Running the following to deploy a node on remote host `remote-host`.
```
./deploy/deploy.sh -m ubuntu@remote-host
```

Run `./deploy/deploy.sh -h` for more options.

## Running a smallish network for development

```
DOCKER_BUILDKIT=1 docker build . -t zhenxunge-node
docker-compose -f deploy/dev/docker-compose.yml up
```
