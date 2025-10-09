<html>
   <head>
      <title>PHP Test Variables </title>
   </head>
   <body>
      <?php
         // Test variables: No declaration, ALL var $
         // Note: PHP is a "Loosely Typed Language"
         //   In PHP a variable does not need to be 
         //   declared before being set.
         // Program-ID: TVar.php
         // Author:     Kuo-pao Yang
         // OS:         Ubuntu 24
         // Parser:     PHP 8

         $i1 = 1;   $i2 = 2;
         $f1 = 3.3; $f2 = 4.4;
         $c  = 'a';
         $s  = "bcd";
         $f1 = $i1;  // Loosely typed (no casting)
         $i2 = $f2;  // Loosely type (no type checking)
         $c  = $c.$s." "."efg"; // Concatenation Operator (.)
         $s  = strlen($s); // string function: length 
         print "i1 = ".$i1."<br>";
         print "i2 = ".$i2."<br>";
         print "f1 = ".$f1."<br>";
         print "f2 = ".$f2."<br>";
         print "c = ".$c."<br>";
         print "s     = ".$s."<br>"; //output only 1 space

         /* Output:
         i1 = 1 
         i2 = 4.4 
         f1 = 1 
         f2 = 4.4 
         c = abcd efg   
         s = 3 
         */
     ?> 
   </body>
</html> 

