# Summary
Dockerfile for building the `source` repository. By default the image builds all artifacts; in time there may be options added to specify the artifacts to be built.

# Files
  * Dockerfile: The Docker image that builds the `source` repository. Run `docker build -t joseph-build .` while in this directory to build the image, then `docker run joseph-build` to run it.