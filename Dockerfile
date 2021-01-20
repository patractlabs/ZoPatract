FROM zopatract/env:latest as build

ENV WITH_LIBSNARK=1
WORKDIR /build

COPY . src
RUN cd src; ./build_release.sh

FROM ubuntu:18.04
ENV ZOPATRACT_HOME=/home/zopatract/.zopatract

COPY --from=build /build/src/scripts/install_libsnark_prerequisites.sh /tmp/

RUN /tmp/install_libsnark_prerequisites.sh \
&& useradd -u 1000 -m zopatract

USER zopatract
WORKDIR /home/zopatract

COPY --from=build --chown=zopatract:zopatract /build/src/target/release/zopatract $ZOPATRACT_HOME/bin/
COPY --from=build --chown=zopatract:zopatract /build/src/zopatract_cli/examples $ZOPATRACT_HOME/examples
COPY --from=build --chown=zopatract:zopatract /build/src/zopatract_stdlib/stdlib $ZOPATRACT_HOME/stdlib

ENV PATH "$ZOPATRACT_HOME/bin:$PATH"
ENV ZOPATRACT_STDLIB "$ZOPATRACT_HOME/stdlib"