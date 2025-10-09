// Test Data types and variables
// int, float, double, char, char[](String)
// NO boolean variable in C
// Program-ID:   TVar.c
// Author:       Kuo-pao Yang
// OS:           Ubuntu 24
// Compiler:     GNU C
// Note:
// The following instructions are used to
//      edit, compile, and run this program
//    $ nano     TVar.c
//    $ gcc      TVar.c
//    $ ./a.out

#include <stdio.h>

void main() {
    int i1 = 1, i2 = 2;
    float f1 = 3.3, f2 = 4.4;
    double d = 5.5;
    char c = 'a';
    char s[] = "bcd";
    char* t = s;

    f1 = i1 ; // implicit casting
    i2 = f2;  // type narrowing (no type checking in C)
    printf("i1 = %d\n", i1); //%d: decimal format
    printf("f1 = %f\n", f1); //%f: floating point format 
    printf("d  = %lf\n",d ); //%lf: double floating point format
    printf("f2 = %f\n", f2); 
    printf("i2 = %d\n", i2);
    printf("c  = %c\n", c ); //%c: char
    printf("s  = %s\n", s ); //%s: string
    printf("t  = %s\n", t ); 
}

/* Output:
    i1 = 1
    f1 = 1.000000
    d  = 5.500000
    f2 = 4.400000
    i2 = 4
    c  = a
    s  = bcd
    t  = bcd
*/
