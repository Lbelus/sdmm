#ifndef __STRING_HEADER_
#define __STRING_HEADER_

#include <common_header.h>
#include <sdmm_types.h>
#include <my_libasm.h>
#include <my_bitmap.h>
#include <my_interval_tree.h>

void    	sdmm_free(void* ptr);
void*   	sdmm_malloc(size_t size);
void* 		sdmm_realloc(void* ptr, int size);
void* 		sdmm_calloc(size_t num, size_t size);
size_t 		sdmm_malloc_usable_size(void* ptr);

#endif