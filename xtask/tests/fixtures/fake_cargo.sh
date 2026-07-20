#!/bin/sh
# Controllable fake cargo for integration tests.
# Set FAKE_CARGO_EXIT=1 in the calling environment to simulate test failure.
exit "${FAKE_CARGO_EXIT:-0}"
