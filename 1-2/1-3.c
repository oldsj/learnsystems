
#include <stdio.h>

main() {
	float fahr, celsius;
	int lower, upper, step;

	lower = 0;
	upper = 300;
	step = 20;

	fahr = lower;

  printf("Fahrenheit %18s", "Celsius\n");
	while (fahr <= upper) {
		celsius = (5.0/9.0) * (fahr-32.0);
		printf("%3.0f %23.1f\n", fahr, celsius);
		fahr = fahr + step;
	}
}
