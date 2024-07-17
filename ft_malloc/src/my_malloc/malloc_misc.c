#include <main_header.h>

handler_t* handler = NULL;

/* #################### to_page_size ###################
    Set size to page_size standard;
    @return (int) page_size
*/
int to_page_size(size_t size)
{
    int page_size = sysconf(_SC_PAGESIZE);
    int total_size = size + sizeof(page_t);
    int remainder = (unsigned)total_size % (unsigned)page_size;
    int new_size = total_size + (page_size - (remainder ? remainder : page_size));
    return new_size;
}

/* #################### calc_nb_slot ###################
    Calculate the number of necessary slots according to total page_size;
    @return (int) nb_slot
*/
int calc_nb_slot(size_t size, int nb_page)
{
    int total_slot = BITMAP_SIZE * 8;
    int nb_page_size = nb_page * handler->size_page;
    int len = (total_slot * size + nb_page_size - 1) / nb_page_size; // Avoid floating-point
    if (len == 0)
    {
        return 1;
    }
    return len + 1; // Offset pos from last ptr, avoid overlapping interval
}

/* #################### calc_offset ###################
    Calculate the number of bytes to match slot position in page
    @return (int) page_size
*/
int calc_offset(int slot, int nb_page)
{
    int total_slot = BITMAP_SIZE * 8;
    int offset = ((unsigned)(nb_page * handler->size_page) / total_slot) * slot;
    return offset;
}

/* #################### get_ptr ###################
    Get pointer for user; 
    @return (void) ptr
*/
void* get_ptr(size_t size)
{
    void* ptr = NULL;
    tee_t* tee = NULL;
    bitlist_t* tmp = handler->head;
    if (tmp == NULL)
    {
        return NULL;
    }
    int len = calc_nb_slot(size, tmp->nb_page);
    int slot = 0;
    int offset = 0;
    while (tmp != NULL)
    {
        slot = find_free_slot(tmp->bmp, len);
        if (slot + len >= BITMAP_SIZE * 8)
        {
            return NULL;
        }
        if (slot != -1)
        {
            set_bits(tmp->bmp, slot, len, true);
            offset = calc_offset(slot, tmp->nb_page);            
            tee = (tee_t*)((char*)tmp + offset);
            tee->nb_slot = len;
            tee->magic_number = MAGIC_NUMBER;

            handler->memory = (void*)(tee + 1);
            return (void*)(tee + 1);
        }
        tmp = tmp->next;
    }
    return NULL;
}

/* #################### set_handler ###################
    Set handler
    @return (void)
*/
void set_handler(size_t size_page)
{
    handler->size_page = size_page;
    handler->memory = NULL;
    handler->cursor = 0;
    handler->search_tree = NULL;
    handler->head = NULL;  
}

/* #################### initialize_handler ###################
    Initialize handler
    @return (void)
*/
void initialize_handler()
{
    if (handler == NULL)
    {
        size_t size_page = sysconf(_SC_PAGESIZE);
        void* ptr = my_mmap(size_page);
        handler = ptr;
        set_handler(size_page);
        ptr = set_page((void*)(handler + 1), 1);
        int len = calc_nb_slot(sizeof(handler_t) + sizeof(bitlist_t) + sizeof(page_t), 1);
        set_bits(handler->head->bmp, 0, len, true);
    }
}

/* #################### set_page ###################
    Set page
    @return (page_t*)
*/
page_t* set_page(void* ptr, int nb_page)
{
    page_t* page = (page_t*) ptr;
    bitlist_t* bitlist = (bitlist_t*) (((char*)ptr) + sizeof(page_t));
    char* byte = ((char*)bitlist) + sizeof(bitlist_t);
    page->bitnode = bitlist;
    initialize_bit_list((void*)page->bitnode, nb_page);
    page->byte = byte;
    handler->search_tree = insert(handler->search_tree, page);
    return page;
}

/* #################### req_memory ###################
    Require memory from system in page size using my_mmap and set page;
    @return (void*) ptr
*/
void* req_memory(size_t size)
{
    size_t new_size = to_page_size(size);
    void* ptr = NULL;
    int nb_page = new_size / handler->size_page;
    ptr = my_mmap(new_size);
    if (ptr == MAP_FAILED)
    {
        return NULL;
    }
    ptr = set_page(ptr, nb_page);
    return (void*)ptr;
}

/* #################### release_mem ###################
    Check if next bitmap is clear and release page(s) to memory
    @return (void)
*/
void release_mem(bitlist_t* node)
{
    void* ptr = (void*)node->next;
    if (node->next != NULL && is_bitmap_full(node->next))
    {
        node->next = node->next->next;
        size_t size_dealloc = node->nb_page * handler->size_page;
        munmap(ptr, size_dealloc);
    }
}
