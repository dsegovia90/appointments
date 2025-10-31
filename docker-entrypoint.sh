#!/bin/sh
set -e

# This entrypoint script allows us to use the exec form in CMD
# while still supporting the CARGO_PACKAGE_NAME build argument

exec ./${CARGO_PACKAGE_NAME} "$@"
