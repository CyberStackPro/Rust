#include <stdio.h>

int main()
{
    for (int i = 1000000; i >= 0; i--)
    {
        printf("Countdown: %d\n", i);
    }
    printf("Liftoff!\n");
    return 0;
}
