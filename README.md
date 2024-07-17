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

- solution00:
    > recode the floating-point arithmetic point part.
    
    > Allowing arbitrary system calls and operations could introduce security vulnerabilities. eBPF's design restricts what can be done to prevent malicious code from exploiting these capabilities. 
    ```sh
    src/my_malloc/malloc_misc.c:106:21: error: 0x561a6907bf10: i64 = GlobalAddress<ptr @mmap> 0, src/my_malloc/malloc_misc.c:106:21 too many arguments
    106 |         void* ptr = mmap(NULL, size_page, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
        |                     ^
    1 error generated.
    ```
    - fix attempt: making the syscall directly in assembly.

    > A call to my_bzero is considered like built-in memset.
    ```sh
    src/my_string/my_bzero.c:9:16: error: A call to built-in function 'memset' is not supported.
    9 |         arr[i] = '\0';
      |                ^
    1 error generated.
    ```
    - Code was removed.
    > Warning about the missing .note.GNU-stack section implies that some of the assembly code might result in an executable stack:
    ```sh
      missing .note.GNU-stack section implies executable stack
      /usr/bin/ld: NOTE: This behaviour is deprecated and will be removed in a future version of the linker
      cp ./build/bpfmalloc.o ./
      root@55a1229bb432:/workspace/ft_malloc# ls -l build/bpfmalloc.o
    ```
    - To address this, we need to add the .note.GNU-stack section to the assembly files.

- solution01:
  > recode a simpler malloc implementation that does not rely on bitmaps and floating point shenanigans;