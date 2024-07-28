#ifndef __INTERVAL_TREE_HEADER_
#define __INTERVAL_TREE_HEADER_

#include <common_header.h>
#include <sdmm_types.h>
#include <my_free_list.h>

extern void* my_mmap(int size);

intree_t*    fetch_node();
intree_t*    insert(intree_t* root, void *ptr);
intree_t*    find_page_start(intree_t* root, void* ptr);

#endif