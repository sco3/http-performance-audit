# HTTP Performance Audit

This document contains the results of a performance benchmark test conducted on this project.

## Benchmark Configuration

The benchmark was executed with the following parameters:

*   **URL**: `http://127.0.0.1:8080`
*   **Duration**: 10 seconds
*   **Connections**: 1
*   **Threads**: 1

## Summary of Results

| Tool  | Requests/sec | Average Latency |
| :---- | :----------- | :-------------- |
| `hey`   | 25,798       | 0.00ms          |
| `wrk`   | 88,725       | 0.01ms          |
| `rewrk` | 71,946       | 0.01ms          |

## Detailed Benchmark Output

<details>
<summary>Click to see the full output from the benchmark script</summary>

```
Starting benchmarks with the following parameters:
URL: http://127.0.0.1:8080
Duration: 10s
Connections: 1
Threads: 1
-------------------------------------------------

Running benchmark with hey...

Summary:
  Total:        10.0014 secs
  Slowest:      0.0018 secs
  Fastest:      0.0000 secs
  Average:      0.0000 secs
  Requests/sec: 25798.5147
  
  Total data:   3354273 bytes
  Size/request: 13 bytes

Response time histogram:
  0.000 [1]     |
  0.000 [257486]        |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.000 [490]   |
  0.001 [39]    |
  0.001 [1]     |
  0.001 [1]     |
  0.001 [0]     |
  0.001 [2]     |
  0.001 [0]     |
  0.002 [0]     |
  0.002 [1]     |


Latency distribution:
  10%% in 0.0000 secs
  25%% in 0.0000 secs
  50%% in 0.0000 secs
  75%% in 0.0000 secs
  90%% in 0.0001 secs
  95%% in 0.0001 secs
  99%% in 0.0001 secs

Details (average, fastest, slowest):
  DNS+dialup:   0.0000 secs, 0.0000 secs, 0.0003 secs
  DNS-lookup:   0.0000 secs, 0.0000 secs, 0.0000 secs
  req write:    0.0000 secs, 0.0000 secs, 0.0002 secs
  resp wait:    0.0000 secs, 0.0000 secs, 0.0018 secs
  resp read:    0.0000 secs, 0.0000 secs, 0.0005 secs

Status code distribution:
  [200] 258021 responses




-------------------------------------------------

Running benchmark with wrk...
Running 10s test @ http://127.0.0.1:8080
  1 threads and 1 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    12.05us   26.51us   2.00ms   99.00%
    Req/Sec    89.33k     8.43k  100.66k    80.00%
  887277 requests in 10.00s, 110.00MB read
Requests/sec:  88725.22
Transfer/sec:     11.00MB

-------------------------------------------------

Running benchmark with rewrk...
Beginning round 1...
Benchmarking 1 connections @ http://127.0.0.1:8080 for 10 second(s)
  Latencies:
    Avg      Stdev    Min      Max      
    0.01ms   0.01ms   0.01ms   1.48ms   
  Requests:
    Total: 719504  Req/Sec: 71946.05
  Transfer:
    Total: 89.20 MB Transfer Rate: 8.92 MB/Sec


-------------------------------------------------

All benchmarks are complete.
```

</details>
