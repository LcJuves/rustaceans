/**
 * Created at 2021/7/14 23:03
 * 
 * @author Liangcheng Juves
 */

#ifndef _RUST_IMPL_H
#define _RUST_IMPL_H

#include <stdint.h>
#include <stdbool.h>
#include <stdio.h>

#ifdef __cplusplus
extern "C"
{
#endif /* __cplusplus */

#define loop while (1)
#define println(fmt, ...) \
    printf(fmt "\n", __VA_ARGS__);

    typedef int8_t i8;  /* 8-bit */
    typedef uint8_t u8; /* 8-bit */

    typedef int16_t i16;  /* 16-bit */
    typedef uint16_t u16; /* 16-bit */

    typedef int32_t i32;  /* 32-bit */
    typedef uint32_t u32; /* 32-bit */

    typedef int64_t i64;  /* 64-bit */
    typedef uint64_t u64; /* 64-bit */

    // typedef long long long long int128_t;           /* 128-bit */
    // typedef unsigned long long long long uint128_t; /* 128-bit */
    // typedef int128_t i128;                          /* 128-bit */
    // typedef uint128_t u128;                         /* 128-bit */

    /// Floating-Point
    typedef float f32;  /* 32-bit */
    typedef double f64; /* 64-bit */

#if defined(__x86_64__) || (defined(__WORDSIZE) && (__WORDSIZE == 64))
    typedef i64 isize;
    typedef u64 usize;
#elif defined(__i386__) || (defined(__WORDSIZE) && (__WORDSIZE == 32))
typedef i32 isize;
typedef u32 usize;
#endif

#ifdef __cplusplus
} /* extern "C" */
#endif /* __cplusplus */

#endif /* _RUST_IMPL_H */