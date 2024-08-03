#include <safe_sdmm.h>


tagged_value_t safe_sdmm_malloc(size_t size, tag_t tag)
{
    tagged_value_t tagged_value;
    tagged_value.tag = tag;
    switch (tag)
    {
        case TAG_INT:
            tagged_value.value._int_ = (int*)sdmm_malloc(size);
            break;
        case TAG_INT8:
            tagged_value.value.int8 = (int8_t*)sdmm_malloc(size);
            break;
        case TAG_INT16:
            tagged_value.value.int16 = (int16_t*)sdmm_malloc(size);
            break;
        case TAG_INT32:
            tagged_value.value.int32 = (int32_t*)sdmm_malloc(size);
            break;
        case TAG_INT64:
            tagged_value.value.int64 = (int64_t*)sdmm_malloc(size);
            break;
        case TAG_UINT8:
            tagged_value.value.uint8 = (uint8_t*)sdmm_malloc(size);
            break;                                                        
        case TAG_UINT16:
            tagged_value.value.uint16 = (uint16_t*)sdmm_malloc(size);
            break;
        case TAG_UINT32:
            tagged_value.value.uint32 = (uint32_t*)sdmm_malloc(size);
            break;
        case TAG_UINT64:
            tagged_value.value.uint64 = (uint64_t*)sdmm_malloc(size);
            break;
        case TAG_FLOAT32:
            tagged_value.value.float32 = (float*)sdmm_malloc(size);
            break;
        case TAG_FLOAT64:
            tagged_value.value.float64 = (double*)sdmm_malloc(size);
            break;
        case TAG_STRING:
            tagged_value.value.string = (char*)sdmm_malloc(size);
            break;
        default:
            tagged_value.value._int_ = NULL;
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