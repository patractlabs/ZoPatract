#!/bin/bash

# This script takes care of building your crate and packaging it for release

set -ex

main() {
    local src=$(pwd) \
          stage=

    case $TRAVIS_OS_NAME in
        linux)
            stage=$(mktemp -d)
            ;;
        osx)
            stage=$(mktemp -d -t tmp)
            ;;
    esac

    case $TARGET in
        x86_64-pc-windows-gnu)
            BINARY_NAME=zopatract.exe
            ;;
        *)
            BINARY_NAME=zopatract
            ;;
    esac

    test -f Cargo.lock || cargo generate-lockfile

    cross build --bin zopatract --target $TARGET --release

    # Package artifacts
    # Binary
    cp target/$TARGET/release/$BINARY_NAME $stage/
    # Standard library
    cp -r zopatract_stdlib/stdlib $stage

    cd $stage
    tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
    cd $src

    rm -rf $stage
}

main
