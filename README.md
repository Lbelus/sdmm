# sdmm
Solana Dynamic Memory Manager.

### Objective: 
The goal is to push on Solana blockchain an assembly, C, rust cross-compiled  monstrosity that would make David Cronenberg proud.

I want to create a heap smart contract that would handle other programs memory to bypass the 4kb stack limit.  it relies on my_malloc implementation that I made a while ago.

That malloc implementation used float point operation  which is not supported by eBPF VM. The implementation on itself can run linux programs with no issue using the pre_load trick. 

- Create a Solana program that uses a custom heap and cross-program invocations;
- Cross-Compiling with Rust;
- Use that program with Cross-Program Invocations as a way to temporarily store data from other programs.


### Would It Make Sense? Would It Be Useful?
tldr; no

#### Pros:

- Custom Memory Management: It may optimize memory allocation to suit specific needs better than the default allocator.
- Shared Memory: It will Facilitate shared data storage across programs.

#### Cons:

- Complexity: Adds complexity to the Solana programs and introduces more points of failure.
- Overhead: Cross-program invocations introduce overhead that might negate the benefits of custom allocation.

#### Practical Use Cases
- Large Temporary Buffers: If an application requires large temporary buffers that exceed the stack size limitations of Solana programs.
- Custom Allocation Patterns: If an application benefits from specific allocation patterns not supported by the default allocator.



#### Implementation

An eBPF environment can have strict requirements, one of which is the compilation time in a JIT (Just-In-Time) paradigm.

The implementation relies on a C and ASM coded malloc function bound to Rust. The issue is that sdmm_malloc returns, as one would expect, a void pointer.

A void pointer is a type-agnostic pointer, which could lead, in Rust, to the abundant use of unsafe code, creating undefined behavior, reading forbidden locations in memory, and dramatically increasing compilation time. All of these issues are absolute no-gos in an eBPF environment.

The solution to this is to provide a deterministic type-to-type (C to Rust) implementation that relies on a tagged union implemented in C and then ported to Rust.

So, instead of asking for memory via a raw void pointer, the user requests a given number of elements of a specific type.

##### generic vs harcoded type. 

While probably more elegant and less tedious, the use of generics could lead to increased compilation time. Solana and eBPF have an hard limit regarding compilation, so, in that regard, a harcoded approach was chosen.



### eBPF documentation;
- https://www.youtube.com/watch?v=oBW2KJq3FnA
- https://qmonnet.github.io/whirl-offload/2020/04/12/llvm-ebpf-asm/
- https://medium.com/@megawan/writing-compiling-and-loading-ebpf-program-7b0efa014142
- https://solana.com/docs/programs/faq#berkeley-packet-filter-bpf
- https://ebpf.io/what-is-ebpf/
- https://qmonnet.github.io/whirl-offload/2020/04/12/llvm-ebpf-asm/

### Building a solana Program without framework 
- https://betterprogramming.pub/solana-programming-primer-1c8aae509346

### binding and unsafe c to rust code arround raw void pointers; 

https://medium.com/dwelo-r-d/wrapping-unsafe-c-libraries-in-rust-d75aeb283c65
https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#dereferencing-a-raw-pointer


compile and generate the test: 
- From the root dir:
```bash 
source ./compile_and_cp.sh
or
source ./re.sh
```
- go to ``sdmm``
```bash 
cargo build
cargo test
cargo run
```

### solved problematic 

- Current malloc implementation is using floating point arithmetic as a workarround:

eBPF does not support floating point because they are non-deterministic.

- solution00:
    > Recode the floating-point arithmetic point part.
    
    > Allowing arbitrary system calls and operations could introduce security vulnerabilities. eBPF's design restricts what can be done to prevent malicious code from exploiting these capabilities. 
    ```sh
    src/my_malloc/malloc_misc.c:106:21: error: 0x561a6907bf10: i64 = GlobalAddress<ptr @mmap> 0, src/my_malloc/malloc_misc.c:106:21 too many arguments
    106 |         void* ptr = mmap(NULL, size_page, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
        |                     ^
    1 error generated.
    ```
    - Fix attempt: Making the syscall directly in assembly.

    > A call to my_bzero is considered like built-in memset.
    ```sh
    src/my_string/my_bzero.c:9:16: error: A call to built-in function 'memset' is not supported.
    9 |         arr[i] = '\0';
      |                ^
    1 error generated.
    ```
    - Fix attempt: Code was removed.
    > Warning about the missing .note.GNU-stack section implies that some of the assembly code might result in an executable stack:
    ```sh
      missing .note.GNU-stack section implies executable stack
      /usr/bin/ld: NOTE: This behaviour is deprecated and will be removed in a future version of the linker
      cp ./build/bpfmalloc.o ./
      root@55a1229bb432:/workspace/ft_malloc# ls -l build/bpfmalloc.o
    ```
    - To address this, we need to add the .note.GNU-stack section to the assembly files. It compiled, some tests will be perfomed with the LD_PRELOAD trick to insure that this version of my_malloc can run properly before proceeding further.

- solution01:
  > recode a simpler malloc implementation that does not rely on bitmaps and floating point shenanigans;



