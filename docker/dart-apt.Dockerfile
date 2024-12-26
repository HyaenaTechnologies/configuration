FROM amd64/debian:latest

WORKDIR /dart-application

COPY ./ ./

RUN apt update && apt -y full-upgrade 
RUN apt -y install apt-transport-https
RUN wget -qO- https://dl-ssl.google.com/linux/linux_signing_key.pub \ | gpg  --dearmor -o /usr/share/keyrings/dart.gpg
RUN echo 'deb [signed-by=/usr/share/keyrings/dart.gpg arch=amd64] https://storage.googleapis.com/download.dartlang.org/linux/debian stable main' \ | tee /etc/apt/sources.list.d/dart_stable.list
RUN apt update && apt -y install dart
RUN dart analyze lib
RUN dart doc .
RUN dart info
RUN dart fix lib --dry-run
RUN dart fix lib --apply
RUN dart format lib
RUN dart pub get
RUN dart pub deps
RUN dart pub upgrade
RUN dart run lib/main.dart
RUN dart test
RUN dart compile exe lib/main.dart --output /bin/linux/dart-application
