/**
 * Created at 2020/11/16 22:49.
 * 
 * @author Liangcheng Juves
 */
#include <stdio.h>
#include <unistd.h>
#include <time.h>

#define forever while (1)

int main(int argc, char const *argv[])
{
    forever
    {
        printf("\r%ld", time(NULL));
        usleep(600);
        fflush(stdout);
    }
    return 0;
}
