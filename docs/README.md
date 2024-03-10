# Quick reference

-	**Maintained by**:  
     [Pasindu Akalpa](https://github.com/pAkalpa)

-	**Where to get help**:  
     [gmail](mailto:pasinduakalpa1998@gmail.com)

# Supported tags and respective `Dockerfile` links

-	[`latest`](https://github.com/pAkalpa/tauri-dynamic-update-server/blob/main/Dockerfile)

# Quick reference (cont.)

-	**Where to file issues**:  
     [https://github.com/pAkalpa/tauri-dynamic-update-server/issues](https://github.com/pAkalpa/tauri-dynamic-update-server/issues)

-	**Supported architectures**:
     `amd64`, `arm64`

-	**Source of this description**:  
     [docs directory](https://github.com/pAkalpa/tauri-dynamic-update-server/tree/main/docs)

# What is Tauri Dynamic Update Server?

[Tauri Dynamic Update Server](https://github.com/pAkalpa/tauri-dynamic-update-server) is a minimalistic server that can be used to serve updates to Tauri applications dynamically. This server image is ~137MB in size and is based on the [debian:bookworm-slim](https://hub.docker.com/_/debian/tags?page=1&name=bookworm-slim) image.

# How to use this image

## start a tauri dynamic server instance

```console
$ docker run --name dynamic-update-server -p 8000:8000 -e github_token=mygithubtoken -e github_repo_owner=mygithubusername -e github_repo_name=mytauriappreponame -d dynamicus:latest
```

## ... via `docker-compose` or `docker stack deploy`

Example `docker-compose.yml` for `tauri-dynamic-update-server`:

```yaml
version: '3.9'

services:
  api:
    image: dynamicus:latest
    container_name: dynamic-update-server
    restart: always
    ports:
      - '8000:8000'
    environment:
      github_token: mygithubtoken
      github_repo_owner: mygithubusername
      github_repo_name: mytauriappreponame

```

Run `docker stack deploy -c stack.yml dynamic-update-server` (or `docker-compose -f stack.yml up`), wait for it to initialize completely, and visit `http://swarm-ip:8000`, `http://localhost:8000`, or `http://host-ip:8000` (as appropriate).

# How to extend this image

There are many ways to extend the `tauri update server` image. Without trying to support every possible use case, here are just a few that we have found useful.

## Environment Variables

The Tauri Update Server image uses several environment variables which are easy to miss. There are few variables required to run server they are `github_token`, `github_repo_owner`, `github_repo_name`, the rest are optional.

### `github_token`

This is the github token that is used to authenticate with the github api. This is a required field.

### `github_repo_owner`

This is the owner of the github repository that the tauri app is hosted in. This is a required field.

### `github_repo_name`

This is the name of the github repository that the tauri app is hosted in. This is a required field.

### `github_api_url`

This is the url of the github api. This is an optional field. The default value is `https://api.github.com`.

### `home_redirect_url`

This is the url that the server will redirect to when the user visits the root of the server. This is an optional field. The default value is `https://github.com/pAkalpa`.
