
hyper benchmark using wrk
=========================

This 'benchmark' was performed on my AMD Phenom 8-core 16gig Ram
gaming rig while I was listening to pandora, had irc running, and
running several chromes.


```shell
    wrk -c 64 -d 30s -t 64  http://localhost:3000
```
```
Running 30s test @ http://localhost:3000
  64 threads and 64 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   336.28us    1.44ms  26.47ms   99.08%
    Req/Sec     5.18k     1.92k    9.00k    68.47%
  1473410 requests in 30.01s, 148.95MB read
  Socket errors: connect 0, read 0, write 0, timeout 798
Requests/sec:  49099.28
Transfer/sec:      4.96MB
```
