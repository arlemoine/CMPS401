<html>
   <head>
      <title>PHP Test Selections  </title>
   </head>
   <body>
      <?php
         // Test Selections:      if, if-else, nested if-else
         // Logical Operators:    &&, ||, !
         // Relational Operators: <, >, ==, <=, >=, !=
         // Program-ID: TSel.php
         // Author:     Kuo-pao Yang
         // OS:         Ubuntu 24
         // Parser:     PHP 8

         $i1=1; $i2=2; $i3=3; $i4=4; $i5=5; $i6=6;

         // Test a simple if
         if ($i4 > $i1) print "i4 > i1 <br>"; 

         // Test if-else
         if (($i5 < $i2) && ($i3 >= $i2))
            print "(i5 <  i2) && (i3 >= i2) <br>";
         else         
            print "(i5 >= i2) || (i3 <  i2) <br>";

         // Test nested if-else
         if ($i1 != $i2) {
            print "(i1 != i2) <br>";
         }
         else {
            if (($i4 == $i5) || ($i5 != $i6)) {
               print "(i1 == i2)&& ((i4 == i5) || (i5 != i6)) <br>";
            }
         }
      
         /* Output:
         i4 > i1 
         (i5 >= i2) || (i3 < i2) 
         (i1 != i2) 
         */
      ?> 
   </body>
</html> 
