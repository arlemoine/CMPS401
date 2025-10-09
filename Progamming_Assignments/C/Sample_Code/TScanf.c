// Test Data types and variables
// int, float, double, char, char[](String)
// NO boolean variable in C
// Program-ID:   TScanf.c
// Author:       Kuo-pao Yang
// OS:           Ubuntu 24
// Compiler:     GNU C
// Note:
// The following instructions are used to
//      edit, compile, and run this program
//    $ nano     TScanf.c
//    $ gcc      TScanf.c
//    $ ./a.out

#include <stdio.h>

void main() {
    int i; 
    printf("Enter integer i: ");
    scanf("%d", &i); //%d: decimal format, &: address of
    printf("i = %d\n", i );

    float f;
    printf("Enter float f: "); 
    scanf("%f", &f); //%f: float, &: address of
    printf("f = %f\n", f ); 

    double d;
    printf("Enter double d: ");
    scanf("%lf", &d); //%f: float, &: address of
    printf("d = %lf\n", d );

    char s[80];
    printf("Enter string s[80]: ");
    scanf("%s", s); //%s: string, NO address of (&)
    printf("s = %s\n", s );
    printf("s+1 = %s\n", s+1 );
}

/* Output:
Enter integer i: 1
i = 1
Enter float f: 2.2
f = 2.200000
Enter double d: 3.3
d = 3.300000
Enter string s[80]: abcd
s = abcd
s+1 = bcd
*/
