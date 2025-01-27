_start:
   mov r1 #-5
   mov r2 #123
   out r1      // this is the testing
   nli
   out r2
   nli
   add r1 r2
   out r1
   jmp _loop
   hlt 
   

         ; will this gonna work?
_loop:
   mov r5 #2
   out r5
   hlt
