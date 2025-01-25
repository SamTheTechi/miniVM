_start:
   sin r0,
   mov r1 #0
   lod r2 r0 
   dec r0

_loop:
   out r1
   inc r1
   _iloop:
      out r5
      dec r2 
      cmp r1 r2
      jme _iloop
   lod r2 r0 
   cmp r1 r0
   jml _loop
   hlt
