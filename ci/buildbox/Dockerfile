#Use https://github.com/niaquinto/docker-gradle 's file as a base.
FROM niaquinto/gradle

#Install the most recent version of Rust as well.
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable

#Sometimes rustup doesn't update the PATH?
#May need to manually add the new install path.
#ENV PATH $PATH:$HOME/.cargo/bin

#By default this lists the Rust installation's version,
#but it can run whatever command you feel like.
WORKDIR /usr/bin/app
ENTRYPOINT ["/bin/bash -c"]
CMD ["rustc", "--version"]