#include <safe_sdmm.h>


tagged_value_t safe_sdmm_malloc(size_t num_elements, tag_t tag)
{
    tagged_value_t tagged_value;
    tagged_value.tag = tag;
    switch (tag)
    {
        case TAG_INT:
            tagged_value.value._int_    = (int*)sdmm_malloc(num_elements * sizeof(int));
            break;
        case TAG_INT8:
            tagged_value.value.int8     = (int8_t*)sdmm_malloc(num_elements * sizeof(int8_t));
            break;
        case TAG_INT16:
            tagged_value.value.int16    = (int16_t*)sdmm_malloc(num_elements * sizeof(int16_t));
            break;
        case TAG_INT32:
            tagged_value.value.int32    = (int32_t*)sdmm_malloc(num_elements * sizeof(int32_t));
            break;
        case TAG_INT64:
            tagged_value.value.int64    = (int64_t*)sdmm_malloc(num_elements * sizeof(int64_t));
            break;
        case TAG_UINT8:
            tagged_value.value.uint8    = (uint8_t*)sdmm_malloc(num_elements * sizeof(uint8_t));
            break;                                                        
        case TAG_UINT16:
            tagged_value.value.uint16   = (uint16_t*)sdmm_malloc(num_elements * sizeof(uint16_t));
            break;
        case TAG_UINT32:
            tagged_value.value.uint32   = (uint32_t*)sdmm_malloc(num_elements * sizeof(uint32_t));
            break;
        case TAG_UINT64:
            tagged_value.value.uint64   = (uint64_t*)sdmm_malloc(num_elements * sizeof(uint64_t));
            break;
        case TAG_INTPTR:
            tagged_value.value.intptr   = (intptr_t*)sdmm_malloc(num_elements * sizeof(intptr_t));
            break;
        case TAG_UINTPTR:
            tagged_value.value.uintptr  = (uintptr_t*)sdmm_malloc(num_elements * sizeof(uintptr_t));
            break;
        case TAG_FLOAT32:
            tagged_value.value.float32  = (float*)sdmm_malloc(num_elements * sizeof(float));
            break;
        case TAG_FLOAT64:
            tagged_value.value.float64  = (double*)sdmm_malloc(num_elements * sizeof(double));
            break;
        case TAG_STRING:
            tagged_value.value.string   = (char*)sdmm_malloc(num_elements * sizeof(char));
            break;
        default:
            tagged_value.value._int_    = NULL;
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

void safe_sdmm_free(tagged_value_t tagged_value)
{
    switch (value.tag)
    {
        case TAG_INT:
            sdmm_free(tagged_value.value._int_);
            break;
        case TAG_INT8:
            sdmm_free(tagged_value.value.int8);
            break;
        case TAG_INT16:
            sdmm_free(tagged_value.value.int16);
            break;
        case TAG_INT32:
            sdmm_free(tagged_value.value.int32);
            break;
        case TAG_INT64:
            sdmm_free(tagged_value.value.int64);
            break;
        case TAG_UINT8:
            sdmm_free(tagged_value.value.uint8);
            break;
        case TAG_UINT16:
            sdmm_free(tagged_value.value.uint16);
            break;
        case TAG_UINT32:
            sdmm_free(tagged_value.value.uint32);
            break;
        case TAG_UINT64:
            sdmm_free(tagged_value.value.uint64);
            break;
        case TAG_INTPTR:
            sdmm_free(tagged_value.value.intptr);
            break;
        case TAG_UINTPTR:
            sdmm_free(tagged_value.value.uintptr);
            break;
        case TAG_FLOAT32:
            sdmm_free(tagged_value.value.float32);
            break;
        case TAG_FLOAT64:
            sdmm_free(tagged_value.value.float64);
            break;
        case TAG_STRING:
            sdmm_free(tagged_value.value.string);
            break;
        default:
            break;
    }
}

// size_t safe_sdmm_malloc_usable_size(void* ptr)
// {
//     if (ptr == NULL)
//     {
//         return 0;
//     }
//     return sdmm_malloc_usable_size(ptr);
// }