FROM zopatract/env:latest

COPY . src
RUN cd src/zopatract_js && npm run build && rm -rf target/

RUN cd / && mkdir build \
    && mv src/zopatract_js/* /build \
    && rm -rf src