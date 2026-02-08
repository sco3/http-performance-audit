#!/bin/bash

# Script to benchmark the server with different tools.

URL="http://127.0.0.1:8080"
DURATION_SECONDS="10"
CONNECTIONS=500
THREADS=12

echo "Starting benchmarks with the following parameters:"
echo "URL: $URL"
echo "Duration: ${DURATION_SECONDS}s"
echo "Connections: $CONNECTIONS"
echo "Threads: $THREADS"
echo "-------------------------------------------------"
echo ""

# hey benchmark
# -c: Number of workers to run concurrently
# -z: Duration of application to send requests
echo "Running benchmark with hey..."
hey -c $CONNECTIONS -z "${DURATION_SECONDS}s" $URL
echo ""
echo "-------------------------------------------------"
echo ""

# wrk benchmark
# -t: Number of threads to use
# -c: Connections to keep open
# -d: Duration of test
echo "Running benchmark with wrk..."
wrk -t$THREADS -c$CONNECTIONS -d"${DURATION_SECONDS}s" $URL
echo ""
echo "-------------------------------------------------"
echo ""

# rewrk benchmark
# -t: Number of threads to use
# -c: Number of connections
# -d: Duration of test in seconds
# --url: The URL to benchmark
echo "Running benchmark with rewrk..."
rewrk -t $THREADS -c $CONNECTIONS -d "${DURATION_SECONDS}s" --host $URL
echo ""
echo "-------------------------------------------------"
echo ""

echo "All benchmarks are complete."
