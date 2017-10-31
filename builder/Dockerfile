FROM lambci/lambda:build-python3.6
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable
ADD build.sh /usr/local/bin/
VOLUME ["/code"]
WORKDIR /code
ENTRYPOINT ["/usr/local/bin/build.sh"]
