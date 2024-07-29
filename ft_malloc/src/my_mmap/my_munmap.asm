section .text
    global my_munmap
    my_munmap:

    ; RDI already contains the first argument (addr from C function call)
    ; RSI already contains the second argument (len from C function call)
    ; https://filippo.io/linux-syscall-table/
    mov rax, 11      
    syscall

    ret