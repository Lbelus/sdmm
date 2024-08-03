#ifndef __TAGGED_UNION_HEADER_
#define __TAGGED_UNION_HEADER_

#include <common_header.h>
#include <my_libasm.h>


#ifndef _TAG_ENUM_
#define _TAG_ENUM_
enum tag_enum
{
    TAG_BOOL,
    TAG_INT,
    TAG_INT8,
    TAG_INT16,
    TAG_INT32,
    TAG_INT64,
    TAG_UINT8,
    TAG_UINT16,
    TAG_UINT32,
    TAG_UINT64,
    TAG_INTPTR,
    TAG_UINTPTR,
    TAG_FLOAT32,
    TAG_FLOAT64,
    TAG_STRING
};
typedef enum tag_enum tag_t;
#endif

#ifndef _TAG_VALUE_S_
#define _TAG_VALUE_S_
struct tagged_value_s
{
    tag_t tag;
    union
    {
        int        *_int_;
        int8_t     *int8;
        int16_t    *int16;
        int32_t    *int32;
        int64_t    *int64;
        uint8_t    *uint8;
        uint16_t   *uint16;
        uint32_t   *uint32;
        uint64_t   *uint64;
        intptr_t   *intptr;
        uintptr_t  *uintptr;        
        float      *float32;
        double     *float64;
        char       *string;
    } value;
};
typedef struct tagged_value_s tagged_value_t;
#endif

#endif