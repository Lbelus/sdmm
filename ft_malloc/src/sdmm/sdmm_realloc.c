#include <sdmm.h>

void* sdmm_realloc(void* ptr, int size)
{
    if (ptr == NULL)
    {
        ptr = sdmm_malloc(size);
        return ptr;
    }
    if (size == 0)
    {
        sdmm_free(ptr);
        return NULL;
    }
    void* new_ptr = sdmm_malloc(size);
    int cpy_size  = sdmm_malloc_usable_size(ptr);
    _my_memcpy(new_ptr, ptr, cpy_size);
    sdmm_free(ptr);
    return new_ptr;
}