.global sys_print
sys_print:
	mov     %rsi,%rdx
	mov     %rdi,%rsi
	mov     $0x1,%rdi
	mov     $0x1,%rax
	syscall
	ret

.global sys_exit
sys_exit:
	mov    $0x3C,%rax
	syscall
	hlt
