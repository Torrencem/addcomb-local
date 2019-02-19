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

    test -f Cargo.lock || cargo generate-lockfile

    # TODO Update this to build the artifacts that matter to you
    cross rustc --bin addcomb --target $TARGET --release -- -C lto

    # Move the binaries (whichever exist) to the stage
    if [ -f target/$TARGET/release/addcomb.exe]; then
        cp target/$TARGET/release/addcomb.exe $stage/
    fi
    if [ -f target/$TARGET/release/addcomb]; then
        cp target/$TARGET/release/addcomb $stage/
    fi

    cd $stage
    tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
    cd $src

    rm -rf $stage
}

main
