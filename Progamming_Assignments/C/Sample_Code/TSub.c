// Test Subprograms: Call by Value and Call by Reference
// Program-ID:   TSub.c
// Author:       Kuo-pao Yang
// OS:           Ubuntu 24
// Compiler:     GNU C
// Note:
// The following instructions are used to
//      edit, compile, and run this program
//    $ nano     TSub.c
//    $ gcc      TSub.c
//    $ ./a.out

#include <stdio.h>

int func1(int i1, int *j1) {
    i1 = i1 + 1;
    *j1 = *j1 + 2;
    printf("func1() i1 = %d, j1 = %d\n", i1, *j1);
    return (i1 + *j1);
}
void func2(int i2, int j2[]) {
    i2 = i2 + 3;
    j2[0] = j2[1] + 4;
    printf("func2() i2 = %d, j2 = %d\n", i2, j2[0]);
}
void func3(int i3, int *j3){
    i3 = i3 + 3;
    *j3 = *(j3 + 1) + 4;
    printf("func3() i3 = %d, j3 = %d\n", i3, *j3);
}

void main() {
    //Test call by value and call by reference
    printf("Test call by value and call by reference\n");
    int n1 = 1, n2 = 2;
    int n3 = func1(n1, &n2);
    printf("n1 = %d, n2 = %d, n3 = %d\n", n1, n2, n3);

    //Test Array to Subprogram
    int i;
    int a[3] = {10, 20, 30}; 
    printf("Test Array to Subprogram: way 1 (array)\n");
    func2(a[1], a);
    for(i = 0; i < 3; i++) {
       printf("a[%d] = %d\t", i, a[i]);
    }
    printf("\nTest Array to Subprogram: way 2 (pointer, same result)\n");
    a[0] = 10; a[1] = 20; a[2] = 30;
    func3(a[1], a);
    for(i = 0; i < 3; i++) {
       printf("a[%d] = %d\t", i, a[i]);
    }
}

/* Output:
Test call by value and call by reference
func1() i1 = 2, j1 = 4
n1 = 1, n2 = 4, n3 = 6
Test Array to Subprogram: way 1 (array)
func2() i2 = 23, j2 = 24
a[0] = 24       a[1] = 20       a[2] = 30
Test Array to Subprogram: way 2 (pointer, same result)
func3() i3 = 23, j3 = 24
a[0] = 24       a[1] = 20       a[2] = 30           
*/
