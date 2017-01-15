FROM amazonlinux:latest
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable
RUN yum install -y python27-devel gcc gcc-c++ zip && yum clean all
ADD build.sh /usr/local/bin/
VOLUME ["/code"]
WORKDIR /code
ENTRYPOINT ["/usr/local/bin/build.sh"]
