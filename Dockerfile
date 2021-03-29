FROM amazonlinux:2018.03

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN yum install diffutils gcc72 gcc72-c++ openssl-devel pkg-config capnproto -y

RUN curl -O https://capnproto.org/capnproto-c++-0.8.0.tar.gz && tar zxf capnproto-c++-0.8.0.tar.gz && \
    cd capnproto-c++-0.8.0 && ./configure && make -j6 check && make install

WORKDIR /usr/src/assemblylift-iomod-stdlib
ADD modules/crypto/Cargo.* ./modules/crypto/
ADD modules/crypto/src/* ./modules/crypto/src/
ADD modules/crypto/guest/src/* ./modules/crypto/guest/src/
ADD modules/crypto/guest/Cargo.* ./modules/crypto/guest/
ADD modules/dynamodb/Cargo.* ./modules/dynamodb/
ADD modules/dynamodb/src/* ./modules/dynamodb/src/
ADD modules/dynamodb/guest/src/* ./modules/dynamodb/guest/src/
ADD modules/dynamodb/guest/Cargo.* ./modules/dynamodb/guest/
ADD modules/s3/Cargo.* ./modules/s3/
ADD modules/s3/src/* ./modules/s3/src/
ADD modules/s3/src/client/* ./modules/s3/src/client/
ADD modules/s3/guest/Cargo.* ./modules/s3/guest/
ADD modules/s3/guest/src/* ./modules/s3/guest/src/
ADD modules/s3/guest/src/xml_util/* ./modules/s3/guest/src/xml_util/
ADD modules/http/Cargo.* ./modules/http/
ADD modules/http/src/* ./modules/http/src/
ADD modules/http/guest/src/* ./modules/http/guest/src/
ADD modules/http/guest/Cargo.* ./modules/http/guest/
ADD Cargo.* ./

RUN $HOME/.cargo/bin/cargo build --release
