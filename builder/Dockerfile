FROM amazonlinux:latest
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable && \
    PATH="$HOME/.cargo/bin:$PATH" rustup install stable && \
    PATH="$HOME/.cargo/bin:$PATH" rustup install nightly
RUN yum install -y bzip2-devel gcc gcc-c++ git openssl-devel python27-devel python36-devel readline-devel sqlite-devel zip \
 && yum clean all
ADD build.sh /usr/local/bin/
VOLUME ["/code"]
WORKDIR /code
ENTRYPOINT ["/usr/local/bin/build.sh"]
