//StackTest
//push constant 17
@17
D=A
@SP
A=M
M=D
@SP
M=M+1

//push constant 17
@17
D=A
@SP
A=M
M=D
@SP
M=M+1

//eq
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=M-D
@IF_TRUE0
D;JEQ
@SP
A=M
M=0
@IF_FALSE0
0;JMP
(IF_TRUE0)
@SP
A=M
M=-1
(IF_FALSE0)
@SP
M=M+1

//push constant 892
@892
D=A
@SP
A=M
M=D
@SP
M=M+1

//push constant 891
@891
D=A
@SP
A=M
M=D
@SP
M=M+1

//lt
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=M-D
@IF_TRUE1
D;JLT
@SP
A=M
M=0
@IF_FALSE1
0;JMP
(IF_TRUE1)
@SP
A=M
M=-1
(IF_FALSE1)
@SP
M=M+1

//push constant 32767
@32767
D=A
@SP
A=M
M=D
@SP
M=M+1

//push constant 32766
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1

//gt
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=M-D
@IF_TRUE2
D;JGT
@SP
A=M
M=0
@IF_FALSE2
0;JMP
(IF_TRUE2)
@SP
A=M
M=-1
(IF_FALSE2)
@SP
M=M+1

//push constant 56
@56
D=A
@SP
A=M
M=D
@SP
M=M+1

//push constant 31
@31
D=A
@SP
A=M
M=D
@SP
M=M+1

//push constant 53
@53
D=A
@SP
A=M
M=D
@SP
M=M+1

//add
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=D+M
@SP
M=M+1

//push constant 112
@112
D=A
@SP
A=M
M=D
@SP
M=M+1

//sub
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=M-D
@SP
M=M+1

//neg
@SP
M=M-1
A=M
D=-M
M=D
@SP
M=M+1

//and
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=M&D
@SP
M=M+1

//push constant 82
@82
D=A
@SP
A=M
M=D
@SP
M=M+1

//or
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=M|D
@SP
M=M+1

