// Test Scope: global, local, block, and static variables
// Program-ID:   TScope.c
// Author:       Kuo-pao Yang
// OS:           Ubuntu 24
// Compiler:     GNU C
// Note:
// The following instructions are used to
//      edit, compile, and run this program
//    $ nano     TScope.c
//    $ gcc      TScope.c
//    $ ./a.out

#include <stdio.h>

int i = 1;       //global var i

void func() {
    printf("func() global i = %d\n", i);
    static int j = 4; //static var j
    j++;
    printf("func() static j = %d\n", j);
}

void main() {
    int i = 2;   //local var i
    {
      int i = 3; //block var i
      printf("block  i = %d\n", i);
    }
    printf("main() i = %d\n", i);
    func();
    func();
}

/* Output:
block  i = 3
main() i = 2
func() global i = 1
func() static j = 5
func() global i = 1
func() static j = 6
*/
