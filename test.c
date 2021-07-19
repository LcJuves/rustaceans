/**
 * Created at 2021/7/15 12:00
 * 
 * @author Liangcheng Juves
 */
#include "rustdef.h"

int main(int argc, char const *argv[])
{
    u128 q = 2;
    println("%ld", sizeof(q) * 8);
    u8 num = 255;
    println("%d", utoi_t(num, char));
    return 0;
}
