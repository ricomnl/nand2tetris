// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/06/add/Add.asm

// Computes R0 = 2 + 3  (R0 refers to RAM[0])

// Set Stack Pointer to address 256
@256 
D=A
@0
M=D

// Push 8 onto the stack and increase SP by 1
@8
D=A // store the value temporarily
@0
A=M // set the address to the SP address
M=D // set the value at M[address] to temporarily stored value
// increase stack pointer
@0 
M=M+1

//@7
//D=A
//@0
//A=M
//M=D
// increase stack pointer
//@0
//M=M+1

// Get the element on top of the stack and store it temporarily
@0
M=M-1 // set the SP back to the topmost element
A=M 
D=M // temporarily store the element at M[address]

@0 
M=M-1 // set the SP to the second element from the top
A=M
D=-D // (+|-|eq|gt|lt|and|or) add the topmost element to the second element from the top at M[address]

@0
A=M
M=D // write the result back to the current stack top
// increase stack pointer
@0
M=M+1