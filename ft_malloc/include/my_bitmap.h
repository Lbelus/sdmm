#ifndef __LINKEDLIST_
#define __LINKEDLIST_

#include <common_header.h>
#include <sdmm_types.h>
#include <my_allocator.h>

bool    is_bitmap_full(bitlist_t* node);
void    set_bits(bmp_t* bmp, unsigned int start, unsigned int len, int value);
void    initialize_bit_list(void* ptr, int nb_page);
int     find_free_slot(bmp_t* bmp, unsigned int len);

#endif