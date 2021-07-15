/**
 * Created at 2021/7/15 12:00
 * 
 * @author Liangcheng Juves
 */
#include "rust_impl.h"

int main(int argc, char const *argv[])
{
    u128 q = 2;
    println("%ld", sizeof(q) * 8);
    return 0;
}
