/**
 * Created at 2021/7/14 23:03
 *
 * @author Liangcheng Juves
 */

#ifndef _RDEF_H
#define _RDEF_H

#include <inttypes.h>
#include <stdbool.h>
#include <stdio.h>

#ifdef __cplusplus
extern "C" {
#endif /* __cplusplus */

#define loop for (;;)
#define print(fmt, ...) printf(fmt, __VA_ARGS__);
#define println(fmt, ...) print(fmt "\n", __VA_ARGS__);

#define utoi_t(x, t) ((signed t)x)
#define itou_t(x, t) ((unsigned t)x)

typedef int8_t i8;  /* 8-bit */
typedef uint8_t u8; /* 8-bit */

typedef int16_t i16;  /* 16-bit */
typedef uint16_t u16; /* 16-bit */

typedef int32_t i32;  /* 32-bit */
typedef uint32_t u32; /* 32-bit */

typedef int64_t i64;  /* 64-bit */
typedef uint64_t u64; /* 64-bit */

typedef __int128_t int128_t;   /* 128-bit */
typedef __uint128_t uint128_t; /* 128-bit */
typedef int128_t i128;         /* 128-bit */
typedef uint128_t u128;        /* 128-bit */

typedef intptr_t isize;  /* arch */
typedef uintptr_t usize; /* arch */

/// Floating-Point
typedef float f32;  /* 32-bit */
typedef double f64; /* 64-bit */

#ifdef __cplusplus
} /* extern "C" */
#endif /* __cplusplus */

#endif /* _RDEF_H */