.global asm_print
asm_print:
	mov     %rsi,%rdx
	mov     %rdi,%rsi
	mov     $0x1,%rdi
	mov     $0x1,%rax
	syscall
	ret

.global asm_exit
asm_exit:
	mov    $0x3C,%rax
	syscall
	hlt
