#Use Alpine OpenJDK as a base.
FROM openjdk:alpine

ENV GRADLE_VERSION 3.5
WORKDIR /usr/local

#Install the most recent version of Rust.
RUN apk add \
    g++ \
    rust \
    cargo && \
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
CMD ["/bin/bash"]