#ifndef __LIBASM_HEADER_

// Declare the assembly function
extern int      _my_strlen(const char* str);
extern void*    _my_memset(void* dest, char ch, int len);
extern void*    _my_memcpy(void* dest, const void* src, int len);

#endif