// Test System Function
//    system: pass a command to the shell
// Program-ID:   TSystem.c
// Author:       Kuo-pao Yang
// OS:           Ubuntu 24
// Compiler:     GNU C
// Note:
// The following instructions are used to
//      edit, compile, and run this program
//    $ nano     TSystem.c
//    $ gcc      TSystem.c
//    $ ./a.out

#include <stdio.h>
#include <stdlib.h>

void main() {
   system("ls -al");
}

/* Output:
drwxrwxr-x 2 yang yang 4096 Sep 24 02:35 .
drwxrwxr-x 6 yang yang 4096 Sep 24 01:45 ..
-rwxrwxr-x 1 yang yang 8381 Sep 24 02:35 a.out
-rw-rw-r-- 1 yang yang 2243 Aug 28 20:55 TEnviron.c
-rw-rw-r-- 1 yang yang 1662 Sep 24 02:28 TLoop.c
-rw-rw-r-- 1 yang yang 1747 Sep 24 02:05 TPointer.c
-rw-rw-r-- 1 yang yang 1145 Aug 28 20:55 TScanf.c
-rw-rw-r-- 1 yang yang  856 Aug 28 20:55 TScope.c
-rw-rw-r-- 1 yang yang 1030 Aug 28 20:55 TSel.c
-rw-rw-r-- 1 yang yang  407 Aug 28 20:55 TSimple.c
-rw-rw-r-- 1 yang yang  965 Aug 28 20:55 TStrtok.c
-rw-rw-r-- 1 yang yang 1797 Aug 28 20:55 TStructUnion.c
-rw-rw-r-- 1 yang yang 1800 Sep 24 02:33 TSub.c
-rw-rw-r-- 1 yang yang 1239 Aug 28 20:55 TSystem.c
-rw-rw-r-- 1 yang yang 1137 Aug 28 20:55 TVar.c
*/
