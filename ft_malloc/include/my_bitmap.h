#ifndef __BITMAP_HEADER_
#define __BITMAP_HEADER_

#include <common_header.h>
#include <sdmm_types.h>
#include <my_allocator.h>

bool    is_bitmap_full(const bitlist_t* node);
void    set_bits(bmp_t* bmp, unsigned int start, unsigned int len, int value);
void    initialize_bit_list(void* ptr, int nb_page);
int     find_free_slot(const bmp_t* bmp, unsigned int len);

#endif