#include <stdio.h>
#include <string.h>
#include <assert.h>

char* string_rev(char *s) {
    int i, len;
    char temp;
    len = strlen(s);

    for (i = 0; i < len/2; i++) {
	temp = s[i];
	s[i] = s[len - i - 1];
	s[len - i - 1] = temp;
    }

    return s;
}

int check_if_pal_str(char s[]) {
    if (s == string_rev(s)) return 1; 
    return 0;
}

long int int_rev(long int num) {
    long int rev = 0;
    while(num != 0) {
	rev = rev * 10 + num % 10;
	num = num / 10;
    }

    return rev;
}

int check_if_pal_int(long int num) {
    if(num == int_rev(num)) return 1;
    return 0;
}


int main() {
    char s[] = "Test";
    char *sOut = NULL;
    char s2[] = "tseT";
    sOut = string_rev(s);

    long int n1 = 12343;
    long int n2 = 32321;

    assert(!strcmp(sOut, s2));
    assert(int_rev(n1) == n2);
    assert(check_if_pal_str(s));
    assert(check_if_pal_int(n1) == n2);

    return 0;
}

