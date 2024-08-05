#ifndef __SAFE_SDMM_HEADER_
#define __SAFE_SDMM_HEADER_


#include <common_header.h>
#include <tagged_union.h>
#include <sdmm.h>

#define MAP_FAILED ((void *) -1)


void    	        safe_sdmm_free(tagged_value_t value);
tagged_value_t    	safe_sdmm_malloc(size_t size, tag_t tag);
tagged_value_t      safe_sdmm_realloc(void* ptr, size_t size);
tagged_value_t      safe_sdmm_calloc(size_t num, size_t size);
size_t 		        safe_sdmm_malloc_usable_size(void* ptr);

#endif