<html>
    <head>
        <title>P3.php</title>
    </head>
    <body>
        <?php
            $correct = ["a", "c", "a", "a", "c", "b", "a", "c", "a", "P3.php"];
            $answers = $_POST['answers'];
            $score = 0;

            print "First Name: " . $_POST['first_name'] . ", Last Name: " . $_POST['last_name'] . "<br>";
            print "-- Grade CMPS401 HTML Quiz --<br>";

            for ($i = 0; $i < count($correct); $i++) {
                if (isset($answers[$i]) && $answers[$i] == $correct[$i]) {
                    print "Your answer on question " . ($i + 1) . " is {$answers[$i]}. It is correct.<br>";
                    $score++;
                } else {
                    print "Your answer on question " . ($i + 1) . " is {$answers[$i]}. It is incorrect. Correct answer is {$correct[$i]}.<br>";
                }
            }

            print "Your overall grade is $score / 10.<br>";
        ?>
    </body>
</html>

