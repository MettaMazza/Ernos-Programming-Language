
/* Built-in: string concatenation */
long long concat(long long a, long long b) {
    const char* sa = (const char*)a;
    const char* sb = (const char*)b;
    long long la = strlen(sa);
    long long lb = strlen(sb);
    char* result = malloc(la + lb + 1);
    memcpy(result, sa, la);
    memcpy(result + la, sb, lb);
    result[la + lb] = '\0';
    ep_gc_register(result, EP_OBJ_STRING);
    return (long long)result;
}

long long int_to_string(long long val) {
    char* buf = malloc(32);
    snprintf(buf, 32, "%lld", val);
    ep_gc_register(buf, EP_OBJ_STRING);
    return (long long)buf;
}

long long ep_int_to_str(long long val) { return int_to_string(val); }

typedef struct { char* data; long long len; long long cap; } EpStringBuilder;

long long ep_sb_create(long long dummy) {
    (void)dummy;
    EpStringBuilder* sb = (EpStringBuilder*)malloc(sizeof(EpStringBuilder));
    sb->cap = 256;
    sb->len = 0;
    sb->data = (char*)malloc(sb->cap);
    sb->data[0] = '\0';
    return (long long)sb;
}

long long ep_sb_append(long long sb_ptr, long long str_ptr) {
    EpStringBuilder* sb = (EpStringBuilder*)sb_ptr;
    const char* s = (const char*)str_ptr;
    if (!s) return sb_ptr;
    long long slen = strlen(s);
    while (sb->len + slen + 1 > sb->cap) {
        sb->cap *= 2;
        sb->data = (char*)realloc(sb->data, sb->cap);
    }
    memcpy(sb->data + sb->len, s, slen);
    sb->len += slen;
    sb->data[sb->len] = '\0';
    return sb_ptr;
}

long long ep_sb_append_int(long long sb_ptr, long long val) {
    char buf[32];
    snprintf(buf, sizeof(buf), "%lld", val);
    return ep_sb_append(sb_ptr, (long long)buf);
}

long long ep_sb_to_string(long long sb_ptr) {
    EpStringBuilder* sb = (EpStringBuilder*)sb_ptr;
    char* result = (char*)malloc(sb->len + 1);
    memcpy(result, sb->data, sb->len + 1);
    ep_gc_register(result, EP_OBJ_STRING);
    free(sb->data);
    free(sb);
    return (long long)result;
}

long long ep_sb_length(long long sb_ptr) {
    return ((EpStringBuilder*)sb_ptr)->len;
}

long long str_to_ptr(long long s) { return s; }
long long ptr_to_str(long long p) {
    if (p == 0) return (long long)strdup("");
    char* copy = strdup((const char*)p);
    ep_gc_register(copy, EP_OBJ_STRING);
    return (long long)copy;
}

long long peek_byte(long long ptr, long long offset) {
    return (long long)((unsigned char*)ptr)[offset];
}
long long poke_byte(long long ptr, long long offset, long long value) {
    ((unsigned char*)ptr)[offset] = (unsigned char)value;
    return 0;
}
long long alloc_bytes(long long size) {
    return (long long)calloc((size_t)size, 1);
}
long long free_bytes(long long ptr) {
    free((void*)ptr);
    return 0;
}
long long list_to_bytes(long long list_ptr) {
    long long len = length_list(list_ptr);
    unsigned char* buf = (unsigned char*)malloc(len);
    for (long long i = 0; i < len; i++) {
        buf[i] = (unsigned char)get_list(list_ptr, i);
    }
    return (long long)buf;
}
long long bytes_to_list(long long ptr, long long len) {
    long long list = create_list();
    unsigned char* buf = (unsigned char*)ptr;
    for (long long i = 0; i < len; i++) {
        append_list(list, (long long)buf[i]);
    }
    return list;
}

long long ep_gc_get_minor_count() {
    return ep_gc_minor_count;
}
long long ep_gc_get_major_count() {
    return ep_gc_major_count;
}
long long ep_gc_get_nursery_count() {
    return ep_gc_nursery_count;
}

long long string_to_int(long long s) {
    if (s == 0) return 0;
    return atoll((const char*)s);
}

long long read_line() {
    char buf[4096];
    if (fgets(buf, sizeof(buf), stdin) == NULL) { buf[0] = '\0'; }
    size_t len = strlen(buf);
    if (len > 0 && buf[len-1] == '\n') buf[len-1] = '\0';
    char* result = strdup(buf);
    ep_gc_register(result, EP_OBJ_STRING);
    return (long long)result;
}

long long read_int() {
    long long val = 0;
    scanf("%lld", &val);
    while(getchar() != '\n');
    return val;
}

/* Read ONE key immediately: no echo, no waiting for Enter. Returns the key's
   number code (one byte at a time — escape sequences such as arrow keys
   arrive as successive codes), or -1 at end of input. When stdin is a pipe
   or a file (scripted tests), it simply reads the next byte. */
long long read_key() {
#if defined(__wasm__)
    return -1;
#elif defined(_WIN32)
    if (!_isatty(_fileno(stdin))) {
        return (long long)fgetc(stdin);
    }
    return (long long)_getch();
#else
    if (!isatty(STDIN_FILENO)) {
        return (long long)fgetc(stdin);
    }
    struct termios old_state, raw_state;
    if (tcgetattr(STDIN_FILENO, &old_state) != 0) {
        return (long long)fgetc(stdin);
    }
    raw_state = old_state;
    raw_state.c_lflag &= ~(ICANON | ECHO);
    raw_state.c_cc[VMIN] = 1;
    raw_state.c_cc[VTIME] = 0;
    tcsetattr(STDIN_FILENO, TCSANOW, &raw_state);
    unsigned char ch = 0;
    long long got = (long long)read(STDIN_FILENO, &ch, 1);
    tcsetattr(STDIN_FILENO, TCSANOW, &old_state);
    if (got <= 0) return -1;
    return (long long)ch;
#endif
}

/* How wide the terminal window is, in characters. 80 when unknown. */
long long terminal_columns() {
#if defined(__wasm__)
    return 80;
#elif defined(_WIN32)
    CONSOLE_SCREEN_BUFFER_INFO info;
    if (GetConsoleScreenBufferInfo(GetStdHandle(STD_OUTPUT_HANDLE), &info)) {
        long long cols = (long long)(info.srWindow.Right - info.srWindow.Left + 1);
        if (cols > 0) return cols;
    }
    return 80;
#else
    struct winsize ws;
    if (ioctl(STDOUT_FILENO, TIOCGWINSZ, &ws) == 0 && ws.ws_col > 0) {
        return (long long)ws.ws_col;
    }
    return 80;
#endif
}

/* How tall the terminal window is, in lines. 24 when unknown. */
long long terminal_rows() {
#if defined(__wasm__)
    return 24;
#elif defined(_WIN32)
    CONSOLE_SCREEN_BUFFER_INFO info;
    if (GetConsoleScreenBufferInfo(GetStdHandle(STD_OUTPUT_HANDLE), &info)) {
        long long rows = (long long)(info.srWindow.Bottom - info.srWindow.Top + 1);
        if (rows > 0) return rows;
    }
    return 24;
#else
    struct winsize ws;
    if (ioctl(STDOUT_FILENO, TIOCGWINSZ, &ws) == 0 && ws.ws_row > 0) {
        return (long long)ws.ws_row;
    }
    return 24;
#endif
}

long long read_float() {
    double val = 0.0;
    scanf("%lf", &val);
    while(getchar() != '\n');
    long long result; memcpy(&result, &val, sizeof(double));
    return result;
}

long long int_to_float(long long val) {
    double d = (double)val;
    long long result; memcpy(&result, &d, sizeof(double));
    return result;
}

long long float_to_int(long long val) {
    double d; memcpy(&d, &val, sizeof(double));
    return (long long)d;
}

