// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

// Put your code here.

// loop:
//     set position to 0
//     set color to white
//     get keyboard value

//     if keyboard != 0:
//         set color to black

//     while position != 8192:
//         RAM[SCREEN + position] = color
//         position++




(LOOP)

@position
M=0         // initialize position to 0

@color
M=0         // set color to white

@KBD
D=M         // get keyboard value

@WHITE
D;JEQ       // if keyboard != 0

@color
M=-1            // set color to black

(WHITE)     // end if




(PRINTLOOP)   // while

@position
D=M

@8192
D=D-A

@LOOP
D;JEQ       // while position != 8192

@position
D=M

@SCREEN
D=A+D           // SCREEN + position

@R0
M=D             // R0 <- SCREEN + position

@color
D=M             // get color

@R0
A=M
M=D             // RAM[SCREEN + position] = color

@position
M=M+1

@PRINTLOOP
0;JMP


@LOOP
0;JMP