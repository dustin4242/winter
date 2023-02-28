format ELF64 executable

macro print message {
	mov rax, 1
	xor rdi, rdi
	movzx rdx, [message]
	mov [message],0
	mov rsi, message
	syscall
}

macro exit code {
	mov rax, 60
	mov rdi, code
	syscall
}

segment readable writeable
msg db 13, "Hello World", 10

segment readable executable
entry $

print msg
exit 0
