#include <my_bitmap.h>

/* #################### insert_bmp ###################
    Insert BMP in a linkedlist format;
    @return (void)
*/
void insert_bmp(bitlist_t** head, bitlist_t* node_to_insert)
{
    node_to_insert->next = *head;
    *head = node_to_insert;
}

/* #################### set_bitmap ###################
    Set the bits inside the given bitmap to 0;
    @return (void)
*/
void set_bitmap(bmp_t* bmp)
{
    for (int index = 0; index < BITMAP_SIZE; index++)
    {
        bmp[index].data =  0x00;
    }
}

/* #################### set_bit ###################
    Set the bit to the given value;
    @return (void)
*/
void set_bit(bmp_t* bmp, int index, int value)
{
    if (value)
    {
        bmp->data |= (1 << index);
    }
    else
    {
        bmp->data &= ~(1 << index);
    }
}

/* #################### is_bitmap_full ###################
    Check if the bitmap is full;
    @return (bool)
*/
bool is_bitmap_full(const bitlist_t* node)
{
    unsigned int start_offset = calc_nb_slot(sizeof(bitlist_t) + sizeof(bitlist_t) + sizeof(page_t), node->nb_page);
    for (unsigned int index = start_offset; index < BITMAP_SIZE; index++)
    {
        if (node->bmp[index].data != 0xFF)
        {
            return false;
        }
    }
    return true;
}

/* #################### set_in_bmp ###################
    Set the bit inside the given bitmap to 0 or 1;
    @return (void)
*/
void set_in_bmp(bmp_t* bmp, unsigned int index, int value)
{
    unsigned int bmp_index = index / 8;
    unsigned int bit_index = index % 8;

    set_bit(&bmp[bmp_index], bit_index, value);
}

/* #################### set_bits ###################
    Set the bit inside the given bitmap to 0 or 1 on an interval;
    @return (void)
*/
void set_bits(bmp_t* bmp, unsigned int start, unsigned int len, int value)
{
    for (unsigned int index = 0; index < len; index++)
    {
        set_in_bmp(bmp, start + index, value);
    }
}

/* #################### initialize_bit_list ###################
    Set the bitmap to the appropriate and usable format;
    - ptr is given memory
    - all bit initialized to 0;
    - space used by page metadata set to occupied
    - insert bmp to head of linkedlist
    @return (void)
*/
void initialize_bit_list(void* ptr, int nb_page)
{
    bitlist_t* new_node = (bitlist_t*)ptr;
    set_bitmap(new_node->bmp);
    new_node->nb_page = nb_page; 
    new_node->next = NULL;
    unsigned int len = calc_nb_slot(sizeof(bitlist_t) + sizeof(bitlist_t) + sizeof(page_t), new_node->nb_page);
    set_bits(new_node->bmp, 0, len, true);
    insert_bmp(&handler->head, new_node);  
}

int get_bit(bmp_t bmp, unsigned int index)
{
    return (bmp.data >> index) & 1;
}

/* #################### count_free_bits ###################
    Count the number of free slots from a given position.
    @return (int) Number of available slots
*/
unsigned int count_free_bits(const bmp_t* bmp, unsigned int from, unsigned int len)
{
    unsigned int byte_index = from / 8;
    unsigned int bit_index = from % 8;
    unsigned int free_bits_count = 0;

    while (free_bits_count < len && byte_index < BITMAP_SIZE)
    {
        while (bit_index < 8 && free_bits_count < len)
        {
            if (!get_bit(bmp[byte_index], bit_index))
            {
                free_bits_count++;
            }
            else
            {
                return free_bits_count;
            }
            bit_index++;
        }

        if (free_bits_count < len)
        {
            byte_index++;
            bit_index = 0;
        }
    }

    return free_bits_count;
}

/* #################### find_free_slot ###################
    Find number len of available slots inside the given bitmap
    @return (int) position of slot inside bitmap
*/
int find_free_slot(const bmp_t* bmp, unsigned int len)
{
    for (unsigned int byte_index = 0; byte_index < BITMAP_SIZE; byte_index++)
    {
        for (unsigned int bit_index = 0; bit_index < 8; bit_index++)
        {
            if (!get_bit(bmp[byte_index], bit_index))
            {
                unsigned int start = byte_index * 8 + bit_index;
                if (start + len > BITMAP_SIZE * 8)
                {
                    return -1;
                }
                unsigned int nb_slot = count_free_bits(bmp, start, len);
                if (nb_slot >= len)
                {
                    return start;
                }
                bit_index += nb_slot % 8;
                byte_index += nb_slot / 8;
                if (bit_index >= 8)
                {
                    byte_index += bit_index / 8;
                    bit_index %= 8;
                }
            }
        }
    }
    return -1;
}