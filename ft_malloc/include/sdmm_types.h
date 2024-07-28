#ifndef __SDMM_TYPE_HEADER_
#define __SDMM_TYPE_HEADER_

#include <common_header.h>

#ifndef IT_NODE_S
#define IT_NODE_S
struct it_node
{
    void* address;
    uintptr_t max;
    struct it_node* left;
    struct it_node* right;
};
typedef struct it_node intree_t;
#endif

#define BITMAP_SIZE 32

#ifndef _BITMAP_S_
#define _BITMAP_S_
struct bitmap_s
{
    unsigned char   data;
};
typedef struct bitmap_s bmp_t;
#endif


#ifndef _BITLIST_S_
#define _BITLIST_S_
struct bitlist_s
{
    int                 nb_page;
    struct bitlist_s*   next;
    bmp_t               bmp[BITMAP_SIZE];
};
typedef struct bitlist_s bitlist_t;
#endif


#ifndef _PAGE_S_
#define _PAGE_S_
struct _page_s_
{
    bitlist_t*  bitnode;
    char*   	byte;
};
typedef struct _page_s_ page_t;
#endif


#ifndef _TEE_S_
#define _TEE_S_
struct tee_s
{
    unsigned char 	nb_slot;
	int 			magic_number;
};
typedef struct tee_s tee_t;
#endif

#ifndef MEM_HANDLER_S
#define MEM_HANDLER_S
struct mem_handler_s
{
	size_t    	cursor;
	size_t    	size_page;
	void*     	memory;
	intree_t*  	search_tree;
	bitlist_t*  head;
};

typedef struct mem_handler_s handler_t; 
#endif

#ifndef _PAGE_S_
#define _PAGE_S_
struct _page_s_
{
    bitlist_t*  bitnode;
    char*   	byte;
};
typedef struct _page_s_ page_t;
#endif

extern handler_t*   handler;

#endif