## CMSC330 Garbage Collector Implementation Project

This project implements three classic garbage collection techniques in Rust: Reference Counting, Mark and Sweep, and Stop and Copy.


---

## Overview

The goal of this project was to explore different approaches to memory management by simulating how garbage collectors function.

The program reads a file describing a sequence of stack references, heap references, and pop operations.
Using this trace, it updates memory states and applies one of the garbage collection algorithms to free unused memory.



# Part 1: Reference Counting

-Each heap object keeps a count of the number of references to it.
-When a reference count reaches zero, the object is freed.
-Handles chain deletions when freeing an object that references other objects.

# Part 2: Mark and Sweep

-Marks all objects reachable from the stack.
-Sweeps through the heap, removing unmarked (unreachable) objects.

# Part 3: Stop and Copy

-Splits the heap into two halves: from-space (currently used) and to-space (free space).
-Copies all reachable objects from from-space to to-space, compacting them in the process.
-Updates all references (both on the stack and in the heap) to point to the new locations.


## Academic Integrity

> This project was developed as part of **CMSC330** at the University of Maryland.  
> **Do not copy, share, or submit this code** for any other course or academic assignment.


