#ifndef __TAGGED_UNION_HEADER_
#define __TAGGED_UNION_HEADER_

#include <common_header.h>
#include <my_libasm.h>


#ifndef _TAG_ENUM_
#define _TAG_ENUM_
enum tag_enum
{
    TAG_INT,
    TAG_FLOAT,
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
        int     *i;
        float   *f;
        char    *s;
    } value;
};
typedef struct tagged_value_s tagged_value_t;
#endif

#endif