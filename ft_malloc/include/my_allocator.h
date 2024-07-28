#ifndef __MY_ALLOCATOR_HEADER_
#define __MY_ALLOCATOR_HEADER_

#include <common_header.h>
#include <sdmm_types.h>

#define MAGIC_NUMBER 0xDEADFACE

extern void*        my_mmap(int size);

int     	calc_nb_slot(size_t size, int nb_page);
void*   	get_ptr(size_t size);
void    	initialize_handler();
page_t* 	set_page(void* ptr, int nb_page);
void*   	req_memory(size_t size);
void 		release_mem(bitlist_t* node);

#endif