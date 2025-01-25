_even:
   mov r0 #10
   _loopE
   dec r0
   out r0
   nli
   cmp r1 r0
   jmz _loopE
   hlt

_odd:
   mov r0 #10 
   _loopO
   out r1
   inc r1
   cmp r1 r0
   jmz _loopO
   hlt

   // fking assembly

_start:
   mov r1 #2
   sin r0
   mod r0 r1
   clr r1
   cmp r1 r0
   jme _even
   jmp _odd
