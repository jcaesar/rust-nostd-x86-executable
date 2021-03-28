.global sys_write
sys_write:
	mov     $0x1,%rax
	syscall
	ret

.global sys_exit
sys_exit:
	mov    $0x3C,%rax
	syscall
	hlt
