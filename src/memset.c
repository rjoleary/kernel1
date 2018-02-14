void *memset(void *s, int c, unsigned n) {
    unsigned i = 0;
    for (; i < n; i++) {
        ((unsigned char*)s)[i] = (unsigned char)(c);
    }
    return s;
}

void *memcpy(void *dest, const void *src, unsigned n) {
    unsigned i = 0;
    for (; i < n; i++) {
        *((char*)(dest) + i) = *((const char*)(src) + i);
    }
    return dest;
}
