#!/bin/bash

##
#  Övervakt
#
#  Microservices Status Page
#  Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
#  License: Mozilla Public License v2.0 (MPL v2.0)
##

# Read arguments
while [ "$1" != "" ]; do
    argument_key=`echo $1 | awk -F= '{print $1}'`
    argument_value=`echo $1 | awk -F= '{print $2}'`

    case $argument_key in
        -v | --version)
            # Notice: strip any leading 'v' to the version number
            OVERVAKT_VERSION="${argument_value/v}"
            ;;
        *)
            echo "Unknown argument received: '$argument_key'"
            exit 1
            ;;
    esac

    shift
done

# Ensure release version is provided
if [ -z "$OVERVAKT_VERSION" ]; then
  echo "No Övervakt release version was provided, please provide it using '--version'"

  exit 1
fi

# Define release pipeline
function release_for_architecture {
    final_tar="v$OVERVAKT_VERSION-$1.tar.gz"

    rm -rf ./overvakt/ && \
        cross build --target "$2" --release && \
        mkdir ./overvakt && \
        cp -p "target/$2/release/overvakt" ./overvakt/ && \
        cp -r ./overvakt.toml ./res overvakt/ && \
        tar --owner=0 --group=0 -czvf "$final_tar" ./overvakt && \
        rm -r ./overvakt/
    release_result=$?

    if [ $release_result -eq 0 ]; then
        echo "Result: Packed architecture: $1 to file: $final_tar"
    fi

    return $release_result
}

# Run release tasks
ABSPATH=$(cd "$(dirname "$0")"; pwd)
BASE_DIR="$ABSPATH/../"

rc=0

pushd "$BASE_DIR" > /dev/null
    echo "Executing release steps for Övervakt v$OVERVAKT_VERSION..."

    release_for_architecture "x86_64" "x86_64-unknown-linux-musl" && \
        release_for_architecture "aarch64" "aarch64-unknown-linux-musleabihf"
    rc=$?

    if [ $rc -eq 0 ]; then
        echo "Success: Done executing release steps for Övervakt v$OVERVAKT_VERSION"
    else
        echo "Error: Failed executing release steps for Övervakt v$OVERVAKT_VERSION"
    fi
popd > /dev/null

exit $rc
