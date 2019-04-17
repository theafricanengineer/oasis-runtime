#!/bin/bash -e

WORKDIR=${1:-$(pwd)}

# Paths to Go node and keymanager enclave, assuming they were built according to the README
: ${EKIDEN_NODE:=/go/src/github.com/oasislabs/ekiden/go/ekiden/ekiden}
: ${KM_MRENCLAVE:=/go/src/github.com/oasislabs/ekiden/target/enclave/ekiden-keymanager-trusted.mrenclave}
: ${KM_ENCLAVE:=/go/src/github.com/oasislabs/ekiden/target/enclave/ekiden-keymanager-trusted.so}

# Paths to ekiden binaries
: ${EKIDEN_WORKER:=$(which ekiden-worker)}
: ${KM_NODE:=$(which ekiden-keymanager-node)}

# Path to benchmark client
: ${GENESIS_CLIENT:=${WORKDIR}/target/release/genesis}
: ${STATE:=${WORKDIR}/resources/benchmark/state.json}
: ${GATEWAY:=${WORKDIR}/target/release/gateway}

source ${SCRIPTS_UTILS:-scripts/utils.sh}

# Ensure cleanup on exit.
# cleanup() is defined in scripts/utils.sh
trap 'cleanup' EXIT

run_test() {
    # Start genesis state injector.
    echo "Starting genesis state injector."
    ${GENESIS_CLIENT} \
	    ${STATE} # TODO: output_file missing!
}

run_test_network
run_test
cleanup
