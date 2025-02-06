section .data
    hello_world db `Hello, World!\n`,14
    hello_world_len equ $-hello_world

section .text
    global _start

_start:
    mov rax, 1                          ; write syscall
    mov rdi, 1                          ; stdout
    lea rsi, hello_world                ; address of hello_world
    mov rdx, hello_world_len            ; length of hello_world
    syscall                             ; write hello_world to stdout

    mov rax, 60                         ; exit syscall
    mov rdi, 0                          ; exit code
    syscall                             ; return 0
