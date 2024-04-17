# Concurrent queue implementation from scratch
Following along Michael and Scott (M&S) lock-free queue algorithm are described in [their PODC 1996 paper](https://www.cs.rochester.edu/~scott/papers/1996_PODC_queues.pdf).
This impl is using atomic operations to ensure thread safety without locking.
Based on the paper, the queue is implemented as a linked list with a sentinel node.

## Benchmarking
The benchmarking demonstrates the performance of the concurrent queue compared to the lock-based queue.