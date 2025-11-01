// Test Loop: while, for, nested loops (1-D Arrary and 2-D Array)
// Program-ID:   TLoop.c
// Author:       Kuo-pao Yang
// OS:           Ubuntu 24
// Compiler:     GNU C
// Note:
// The following instructions are used to
//      edit, compile, and run this program
//    $ nano     TLoop.c
//    $ gcc      TLoop.c 
//    $ ./a.out

#include <stdio.h>

void main() {
    int i, j;
    int a[3] = {1, 2, 3};
    int b[3][3] = {{10, 20, 30},
                   {40, 50, 60},
                   {70, 80, 90}};
    int *p;

    printf("\nTest while loop: 1-D Array and Pointer\n");
    p = &a[0];
    i = 0;
    while(i < 3) {
       printf("a[%d] = %d, *p = %d\t", i, a[i], *p);
       p++;
       i++;
    }

    printf("\nTest for loop: 2-D Array and Pointer\n");
    p = &b[1][0];
    for(j = 0; j < 3; j++) {
        printf("b[1,%d] = %d, *p = %d\t", j, b[1][j], *p);
        p++;
    }

    printf("\nTest nested loop: 2-D Array and Pointer");
    p = &b[0][0]; 
    for(i = 0; i < 3; i++) {
      printf("\n");
      for(j = 0; j < 3; j++) {
        printf("b[%d,%d] = %d, *p = %d\t", i, j, b[i][j], *p);
        p++;
      }
    }
}

/* Output:
Test while loop: 1-D Array and Pointer
a[0] = 1, *p = 1        a[1] = 2, *p = 2        a[2] = 3, *p = 3
Test for loop: 2-D Array and Pointer
b[1,0] = 40, *p = 40    b[1,1] = 50, *p = 50    b[1,2] = 60, *p = 60
Test nested loop: 2-D Array and Pointer
b[0,0] = 10, *p = 10    b[0,1] = 20, *p = 20    b[0,2] = 30, *p = 30
b[1,0] = 40, *p = 40    b[1,1] = 50, *p = 50    b[1,2] = 60, *p = 60
b[2,0] = 70, *p = 70    b[2,1] = 80, *p = 80    b[2,2] = 90, *p = 90 
*/
