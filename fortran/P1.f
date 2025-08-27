	  PROGRAM P1
	  INTEGER choice
10    CONTINUE
	  WRITE (*,*) 'Enter a conversion option:'
	  WRITE (*,*) '--------------------------'
	  WRITE (*,*) '(1) Pounds to Kilograms   '
	  WRITE (*,*) '(2) Kilograms to Pounds   '
	  WRITE (*,*) '(3) Feet to meters        '
	  WRITE (*,*) '(4) Meters to feet        '
	  WRITE (*,*) '(5) Fahrenheit to Celsius '
	  WRITE (*,*) '(6) Celsius to Fahrenheit '
	  WRITE (*,*) '(0) Exit this program     '
	  WRITE (*,*) '--------------------------'
	  READ (*,*) choice

	  IF (choice .EQ. 1) THEN
		  CALL poundsToKg()
	  ELSE IF (choice .EQ. 2) THEN
		  CALL kgToPounds()
	  ELSE IF (choice .EQ. 3) THEN
		  CALL feetToMeters()
	  ELSE IF (choice .EQ. 4) THEN
		  CALL metersToFeet()
	  ELSE IF (choice .EQ. 5) THEN
		  CALL fahrenheitToCelsius()
	  ELSE IF (choice .EQ. 6) THEN
		  CALL celsiusToFahrenheit()
	  ELSE IF (choice .EQ. 0) THEN
		  WRITE (*,*) 'Goodbye.'
		  STOP
	  ELSE
		  WRITE (*,*) 'Invalid selection'
	  END IF

	  GOTO 10
	  IF (choice .NE. 0) GOTO 10
	  END
      
	  SUBROUTINE poundsToKg()
		REAL pounds, kg
		WRITE (*,*) 'Enter Pounds: '
		READ (*,*) pounds
		kg = pounds / 2.205
		WRITE (*,*) 'Kilograms:', kg
	  END

	  SUBROUTINE kgToPounds()
		REAL pounds, kg
		WRITE (*,*) 'Enter Kilograms: '
		READ (*,*) kg
		pounds = kg * 2.205
		WRITE (*,*) 'Pounds:', pounds
	  END

	  SUBROUTINE feetToMeters()
		REAL feet, meters
		WRITE (*,*) 'Enter Feet: '
		READ (*,*) feet
		meters = feet * 0.3048
		WRITE (*,*) 'Meters:', meters
	  END

	  SUBROUTINE metersToFeet()
		REAL feet, meters
		WRITE (*,*) 'Enter Meters: '
		READ (*,*) meters
		feet = meters / 0.3048
		WRITE (*,*) 'Feet:', feet
	  END

	  SUBROUTINE fahrenheitToCelsius()
		REAL fahrenheit, celsius
		WRITE (*,*) 'Enter Fahrenheit: '
		READ (*,*) fahrenheit
		celsius = ((fahrenheit - 32) / 1.8)
		WRITE (*,*) 'Celsius:', celsius
		RETURN
	  END

	  SUBROUTINE celsiusToFahrenheit()
		REAL fahrenheit, celsius
		WRITE (*,*) 'Enter Celsius: '
		READ (*,*) celsius
		fahrenheit = ((celsius * 1.8) + 32)
		WRITE (*,*) 'Fahrenheit:', fahrenheit
		RETURN
	  END