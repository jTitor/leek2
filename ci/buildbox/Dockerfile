#Use https://github.com/niaquinto/docker-gradle 's file as a base.
FROM niaquinto/gradle

#Install the most recent version of Rust as well.
#Odd, seems openjdk image doesn't have build tools by default?
RUN apt-get -y update; apt-get install -y build-essential; curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable

#Sometimes rustup doesn't update the PATH?
#May need to manually add the new install path.
ENV PATH $PATH:/root/.cargo/bin

#By default this runs a shell.
ENTRYPOINT ["/bin/sh", "-c"]
CMD ["/bin/bash"]