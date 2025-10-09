<html>
   <head>
     <title>PHP Test Loops </title>
   </head>
   <body>
      <?php
         // Test Loops: while, for, nested loops (1-D, 2-D, and Associative Arrays)
         // Program-ID: TLoop.php
         // Author:     Kuo-pao Yang
         // OS:         Ubuntu 24
         // Parser:     PHP 8

         $a = array(1, 2, 3);
         $b = array( array(10, 20, 30),
                     array(40, 50, 60),
                     array(70, 80, 90));
         //Associative Arrays
         $c = array("p"=>100, "q"=>200, "r"=>300);
         $d = array("x"=>array(100, 200, 300),
                    "y"=>array(400, 500, 600),
                    "z"=>array(700, 800, 900));
 
         print "<br>Test while loop: 1-D Array<br>";
         $i = 0;
         while($i < 3) {
            print "a[".$i."]=".$a[$i]." ";
            $i++;
         }

         print "<br>Test for loop: 2-D Array<br>";
         for($j = 0; $j < 3; $j++) {
            print "b[1,".$j."]=".$b[1][$j]." ";
         }

         print "<br>Test nested loop: 2-D Array";
         for($i = 0; $i < 3; $i++) {
            print "<br>";
            for($j = 0; $j < 3; $j++) {
               print "b[".$i.",".$j."]=".$b[$i][$j]." ";
            }
         }

         print "<br>Test Associative 1-D and 2-D Arrays<br>";
         foreach ($c as $e => $f)
            print "c[".$e."]=".$f." ";
         foreach ($d as $e => $f) {
            print"<br>";
            for ($j = 0; $j < 3; $j++)
               print "d[".$e.",".$j."]=".$f[$j]." ";
         }
      
         /* Output:
         Test while loop: 1-D Array
         a[0]=1 a[1]=2 a[2]=3 
         Test for loop: 2-D Array
         b[1,0]=40 b[1,1]=50 b[1,2]=60 
         Test nested loop: 2-D Array
         b[0,0]=10 b[0,1]=20 b[0,2]=30 
         b[1,0]=40 b[1,1]=50 b[1,2]=60 
         b[2,0]=70 b[2,1]=80 b[2,2]=90 
         Test Associative 1-D and 2-D Arrays
         c[p]=100 c[q]=200 c[r]=300 
         d[x,0]=100 d[x,1]=200 d[x,2]=300 
         d[y,0]=400 d[y,1]=500 d[y,2]=600 
         d[z,0]=700 d[z,1]=800 d[z,2]=900 
         */
      ?> 
   </body>
</html> 

