#Use Alpine Edge as a base
#since the Rust packages are only available on Edge.
FROM alpine:edge

ENV GRADLE_VERSION 3.5
WORKDIR /usr/local

#Add the experimental repository.
RUN echo '@testing http://dl-cdn.alpinelinux.org/alpine/edge/testing' >> /etc/apk/repositories && \
#Install the most recent version of Rust.
apk update && \
apk add \
#Add the shell we're used to...
    bash \
#    bash-doc \
#    bash-completion \
#and the shell builtins.
#    util-linux \
#    pciutils \
#    usbutils \
#    coreutils \
#    binutils \
#    findutils \
    grep \
    curl \
    g++ \
    openjdk8 \
    rust@testing \
    cargo@testing && \
#Install Gradle.
curl -L -O https://services.gradle.org/distributions/gradle-$GRADLE_VERSION-bin.zip && \
    unzip gradle-$GRADLE_VERSION-bin.zip && \
    rm -f gradle-$GRADLE_VERSION-bin.zip && \
    ln -s gradle-$GRADLE_VERSION gradle

#Sometimes rustup doesn't update the PATH?
#May need to manually add the new install path.
ENV IVY_HOME /cache
ENV GRADLE_HOME /usr/local/gradle
ENV PATH ${PATH}:${HOME}/.cargo/bin:${GRADLE_HOME}/bin

#By default this runs a shell.
ENTRYPOINT ["/bin/sh", "-c"]
CMD ["/bin/bash"]