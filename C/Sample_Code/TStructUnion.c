// Test Struct and Union 
// Program-ID:   TStructUnion.c
// Author:       Kuo-pao Yang
// OS:           Ubuntu 24
// Compiler:     GNU C
// Note:
// The following instructions are used to
//      edit, compile, and run this program
//    $ nano     TStructUnion.c
//    $ gcc      TStructUnion.c
//    $ ./a.out

#include <stdio.h>

struct date {
    int y;
    int m;
    int d;
};

union u {
    int i;
    int j;    
};

void main() {
    struct date b[2];
    b[0].y = 1980; b[0].m = 10; b[0].d = 12;
    b[1].y = 1986; b[1].m = 11; b[1].d = 22;
    struct date *p;
    p = b;
    printf("Test Struct and Pointer\n");
    printf("b[0].y = %d\t (*p).y = %d\t p->y = %d\n", b[0].y,(*p).y, p->y);
    printf("b[0].m = %d\t (*p).m = %d\t p->m = %d\n", b[0].m,(*p).m, p->m);
    printf("b[0].d = %d\t (*p).d = %d\t p->d = %d\n", b[0].d,(*p).d, p->d);
    p++;
    printf("b[1].y = %d\t (*p).y = %d\t p->y = %d\n", b[1].y,(*p).y, p->y);
    printf("b[1].m = %d\t (*p).m = %d\t p->m = %d\n", b[1].m,(*p).m, p->m);
    printf("b[1].d = %d\t (*p).d = %d\t p->d = %d\n", b[1].d,(*p).d, p->d);

    union u q;
    union u *r = &q;
    q.i = 20;
    q.j = 30;
    printf("Test Union and Pointer\n");
    printf("q.i = %d\t (*r).i = %d\t r->i = %d\n", q.i,(*r).i, r->i);
    printf("q.j = %d\t (*r).j = %d\t r->j = %d\n", q.j,(*r).j, r->j);
}

/* Output
Test Struct and Pointer
b[0].y = 1980    (*p).y = 1980   p->y = 1980
b[0].m = 10      (*p).m = 10     p->m = 10
b[0].d = 12      (*p).d = 12     p->d = 12
b[1].y = 1986    (*p).y = 1986   p->y = 1986
b[1].m = 11      (*p).m = 11     p->m = 11
b[1].d = 22      (*p).d = 22     p->d = 22
Test Union and Pointer
q.i = 30         (*r).i = 30     r->i = 30
q.j = 30         (*r).j = 30     r->j = 30
*/
