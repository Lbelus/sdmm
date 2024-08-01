#include <safe_sdmm.h>


tagged_value_t safe_sdmm_malloc(size_t size, tag_t tag)
{
    tagged_value_t tagged_value;
    tagged_value.tag = tag;
    switch (tag)
    {
        case TAG_INT:
            tagged_value.value.i = (int*)sdmm_malloc(size);
            break;
        case TAG_FLOAT:
            tagged_value.value.f = (float*)sdmm_malloc(size);
            break;
        case TAG_STRING:
            tagged_value.value.s = (char*)sdmm_malloc(size);
            break;
        default:
            tagged_value.value.i = NULL;
    }
    return tagged_value;
}

// tagged_value_t safe_sdmm_realloc(void* ptr, size_t size)
// {
//     void* new_ptr = sdmm_realloc(ptr, size);
//     return new_ptr;
// }

// tagged_value_t safe_sdmm_calloc(size_t num, size_t size)
// {
//     void* ptr = sdmm_calloc(num, size);
//     return ptr;
// }

// void safe_sdmm_free(void* ptr)
// {
//     if (ptr != NULL)
//     {
//         sdmm_free(ptr);
//     }
// }

// size_t safe_sdmm_malloc_usable_size(void* ptr)
// {
//     if (ptr == NULL)
//     {
//         return 0;
//     }
//     return sdmm_malloc_usable_size(ptr);
// }