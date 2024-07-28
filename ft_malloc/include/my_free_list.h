#ifndef __MY_FREE_LIST_HEADER_
#define __MY_FREE_LIST_HEADER_

#include <common_header.h>
#include <sdmm_types.h>

void init_memory_segment(void* mem_seg, size_t elem_cnt);
intree_t* allocate_node();

extern intree_t* free_chunk;

#endif