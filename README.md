# clippy

```sh
# pull the latest image
docker pull ghcr.io/gldraphael/clippy:main

# run it in the background
docker run -it -d -p 8080:80 --name clippy --rm ghcr.io/gldraphael/clippy:main

# curl it
curl localhost:8080

# once you're done, stop it to delete it
docker stop clippy
```
