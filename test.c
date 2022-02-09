/**
 * Created at 2021/7/15 12:00
 *
 * @author Liangcheng Juves
 */
#include <unistd.h>

#include "rustdef.h"

void main(void) {
    u128 q = 2;
    println("%ld", sizeof(q) * 8);
    u8 num = 255;
    println("%d", utoi_t(num, char));

    usize count = 0;

    loop {
        if (1000 == count) {
            break;
        }
        usleep(6000);
        count++;
    }
}
