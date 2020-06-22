	.intel_syntax noprefix
	.text
_start:   
	mov       rax, 0x02000004      
	mov       rdi, 1               
	mov       rsi, message         
	mov       rdx, 13             
	syscall                       
	mov       rax, 0x02000001      
	xor       rdi, rdi             
	syscall                         

message: 	
	.ascii 	"this world\n"
