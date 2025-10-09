// Test Pointer
// Program-ID:   TPointer.c
// Author:       Kuo-pao Yang
// OS:           Ubuntu 24
// Compiler:     GNU C
// Note:
// The following instructions are used to
//      edit, compile, and run this program
//    $ nano     TPointer.c
//    $ gcc      TPointer.c
//    $ ./a.out

#include <stdio.h>

void main() {
    int n1, n2, n3;    //declare var names
    int *p1, *p2, *p3; //declare pointers to integers 

    p1 = &n1;          //assign "address of" n1 to p1
    p2 = &n2;          //assign "address of" n2 to p2
    p3 = &n3;          //assign "address of" n3 to p3

    //Put values into memory locations pointed to by the ptrs
    n1  = 5;
    *p2 = 7;           // assign 7 to "deference" p2
    *p3 = *p1 + *p2;

    //Print out the adresses of the vars and their contents
    printf("Address\t\t\t Content\t\t Dereference\n");
    printf("-------\t\t\t -------\t\t -----------\n");
    printf("&n1 = %p\t n1 = %d\n", &n1, n1); //%p: address of pointer
    printf("&n2 = %p\t n2 = %d\n", &n2, n2); //\t: tab, \n: new line
    printf("&n3 = %p\t n3 = %d\n", &n3, n3); //%d: decimal format
    printf("&p1 = %p\t p1 = %p\t *p1 = %d\n", &p1, p1, *p1);
    printf("&p2 = %p\t p2 = %p\t *p2 = %d\n", &p2, p2, *p2);
    printf("&p3 = %p\t p3 = %p\t *p3 = %d\n", &p3, p3, *p3);
}

/* Output
Address                  Content                 Dereference
-------                  -------                 -----------
&n1 = 0x7ffeb481e834     n1 = 5
&n2 = 0x7ffeb481e838     n2 = 7
&n3 = 0x7ffeb481e83c     n3 = 12
&p1 = 0x7ffeb481e840     p1 = 0x7ffeb481e834     *p1 = 5
&p2 = 0x7ffeb481e848     p2 = 0x7ffeb481e838     *p2 = 7
&p3 = 0x7ffeb481e850     p3 = 0x7ffeb481e83c     *p3 = 12
*/
