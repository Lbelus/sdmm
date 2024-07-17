# sdmm
Solana Dynamic Memory Manager.

### Objective: 
- Create a Solana program that uses a custom heap and cross-program invocations;
- Cross-Compiling with Rust;
- Use that program with Cross-Program Invocations as a way to temporarily store data other programs.


### Would It Make Sense? Would It Be Useful?
tldr; no

#### Pros:

- Custom Memory Management: You can optimize memory allocation to suit specific needs better than the default allocator.
- Shared Memory: Facilitate shared data storage across programs.

### Cons:

- Complexity: Adds complexity to your Solana programs and introduces more points of failure.
- Overhead: Cross-program invocations introduce overhead that might negate the benefits of custom allocation.

#### Practical Use Cases
- Large Temporary Buffers: If your application requires large temporary buffers that exceed the stack size limitations of Solana programs.
- Custom Allocation Patterns: If your application benefits from specific allocation patterns not supported by the default allocator.


eBPF documentation;
https://www.youtube.com/watch?v=oBW2KJq3FnA
https://qmonnet.github.io/whirl-offload/2020/04/12/llvm-ebpf-asm/
https://medium.com/@megawan/writing-compiling-and-loading-ebpf-program-7b0efa014142

### Current problematic 

- Current malloc implementation is using floating point arithmetic as a workarround:

eBPF does not support the division sign and floating point because they are non-deterministic.

solution:
-> recode the floating-point arithmetic point part;
-> recode a simpler malloc implementation that does not rely on bitmaps and floating point shenanigans;