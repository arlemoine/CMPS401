// Test Strtok Function
//    Split a string into tokens by sparators (delimiters) 
// Program-ID:   TStrtok.c
// Author:       Kuo-pao Yang
// OS:           Ubuntu 24
// Compiler:     GNU C
// Note:
// The following instructions are used to
//      edit, compile, and run this program
//    $ nano     TStrtok.c
//    $ gcc      TStrtok.c
//    $ ./a.out

#include <string.h>
#include <stdio.h>

#define MAX_BUFFER   1024              // max line buffer
#define MAX_ARGS       64              // max # args
#define SEPARATORS   " \t\n"           // token sparators

void main() {
   char  cmd[MAX_BUFFER] = "ls -al";
   char* args[MAX_ARGS];               // pointers to arg strings
   int   i;

   args[0] = strtok(cmd, SEPARATORS);  // tokenize input
   printf("args[0] = %s\n", args[0]);
   for (i = 1; args[i] = strtok(NULL, SEPARATORS); i++)
      printf("args[%d] = %s\n",i, args[i]);
}

/* Output:
args[0] = ls
args[1] = -al
*/

