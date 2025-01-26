_start:
   mov r1 #5
   out r1      // this is the testing
   jmp _loop
   mov r3 "maybe?"
   add r6 "oolalalla"
   add r6 "hola"
   hlt 
   

         ; will this gonna work?
_loop:
   mov r5 #20
   out r5
   hlt
