My solutions to leetcode.

- Try doing things as the problem intended to (like using i32 and dealing with overflowing)
- Prefer recursion (but don't abuse)
- Straitforward. No over-engineering. No extreme optimizing.
- Self contained. Every file can be compiled independently.

## Some random notes:

### 0032

I came up with a "novel" idea that we can repeatedly merge tokens like a syntax parser, later I noticed that this is
exactly the "stack method" in the LeetCode Solutions.

BTW, Rust's built-in double linked list sucks.

### Dancing links in 0037, 0051 and 0052

It took me really long time trouble shooting this. The original paper use different arrays to store different parts of
the nodes, which I feel uncomfortable. I went the array-of-struct way and quickly trapped by Rust's ownership and
lifetime checker, so I finally went to the evil mode, putting `unsafe` and `*` everywhere.

The paper doesn't mention (or I have missed) that this algorithm also support *at most once* cover. It different from
the exact cover in that some columns may not be covered in solutions. N-Queens is this case, becuase some diagonal lines
could be empty. It's actually very easy to support this. Just change the code that checks if the search is done to skip
those optional columns.

Also, I found that **S is VERY important**. I first went without it and wasted a lot of time profiling my program because
it times out. By adding `S`, the time for sudoku drops from 2s to <1ms!

### 0045

This one is very funny. I first thought it was regular DP and only beat about 50%. Later I found that I missed an
important property: if you can get to i in x steps, you must be able to get to anywhere less than i in no more than x
steps. This changed the whole process from "find shorted path to i" to "find farest place reachable in x steps".

