    .file	"test.c"
	.intel_syntax noprefix
	.text
	.globl	main
	.type	main, @function
	.string	"String literal test: Hello, world (but from an x86 program on an ARM processor)"
main:
.LFB0:
	.cfi_startproc
	push	rbp
    call    .L2
	lea rbp, .L2[rip]
    .cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	mov	rbp, rsp
	.cfi_def_cfa_register 6
.L2:
    ret
	jmp	.L2
	.cfi_endproc
.LFE0:
	.size	main, .-main
	.ident	"GCC: (GNU) 14.2.1 20240910"
	.section	.note.GNU-stack,"",@progbits
