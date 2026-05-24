#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <unistd.h>
#include <netdb.h>

long long ep_net_connect(const char* host, long long port) {
    int sockfd = socket(AF_INET, SOCK_STREAM, 0);
    if (sockfd < 0) return -1;
    struct hostent* server = gethostbyname(host);
    if (!server) {
        close(sockfd);
        return -1;
    }
    struct sockaddr_in serv_addr;
    memset(&serv_addr, 0, sizeof(serv_addr));
    serv_addr.sin_family = AF_INET;
    memcpy(&serv_addr.sin_addr.s_addr, server->h_addr_list[0], server->h_length);
    serv_addr.sin_port = htons(port);
    if (connect(sockfd, (struct sockaddr*)&serv_addr, sizeof(serv_addr)) < 0) {
        close(sockfd);
        return -1;
    }
    return sockfd;
}

long long ep_net_listen(long long port) {
    int sockfd = socket(AF_INET, SOCK_STREAM, 0);
    if (sockfd < 0) return -1;
    int opt = 1;
    setsockopt(sockfd, SOL_SOCKET, SO_REUSEADDR, &opt, sizeof(opt));
    struct sockaddr_in serv_addr;
    memset(&serv_addr, 0, sizeof(serv_addr));
    serv_addr.sin_family = AF_INET;
    serv_addr.sin_addr.s_addr = INADDR_ANY;
    serv_addr.sin_port = htons(port);
    if (bind(sockfd, (struct sockaddr*)&serv_addr, sizeof(serv_addr)) < 0) {
        close(sockfd);
        return -1;
    }
    if (listen(sockfd, 10) < 0) {
        close(sockfd);
        return -1;
    }
    return sockfd;
}

long long ep_net_accept(long long server_fd) {
    struct sockaddr_in cli_addr;
    socklen_t clilen = sizeof(cli_addr);
    int newsockfd = accept((int)server_fd, (struct sockaddr*)&cli_addr, &clilen);
    return newsockfd;
}

long long ep_net_send(long long fd, const char* data) {
    if (!data) return 0;
    return send((int)fd, data, strlen(data), 0);
}

char* ep_net_recv(long long fd, long long max_len) {
    char* buf = malloc(max_len + 1);
    if (!buf) {
        char* empty = malloc(1);
        if (empty) empty[0] = '\0';
        return empty;
    }
    ssize_t n = recv((int)fd, buf, max_len, 0);
    if (n < 0) n = 0;
    buf[n] = '\0';
    return buf;
}

long long ep_net_close(long long fd) {
    return close((int)fd);
}

#define ROTRIGHT(word,bits) (((word) >> (bits)) | ((word) << (32-(bits))))
#define CH(x,y,z) (((x) & (y)) ^ (~(x) & (z)))
#define MAJ(x,y,z) (((x) & (y)) ^ ((x) & (z)) ^ ((y) & (z)))
#define EP0(x) (ROTRIGHT(x,2) ^ ROTRIGHT(x,13) ^ ROTRIGHT(x,22))
#define EP1(x) (ROTRIGHT(x,6) ^ ROTRIGHT(x,11) ^ ROTRIGHT(x,25))
#define SIG0(x) (ROTRIGHT(x,7) ^ ROTRIGHT(x,18) ^ ((x) >> 3))
#define SIG1(x) (ROTRIGHT(x,17) ^ ROTRIGHT(x,19) ^ ((x) >> 10))

typedef struct {
    unsigned char data[64];
    unsigned int datalen;
    unsigned long long bitlen;
    unsigned int state[8];
} EP_SHA256_CTX;

static const unsigned int sha256_k[64] = {
    0x428a2f98,0x71374491,0xb5c0fbcf,0xe9b5dba5,0x3956c25b,0x59f111f1,0x923f82a4,0xab1c5ed5,
    0xd807aa98,0x12835b01,0x243185be,0x550c7dc3,0x72be5d74,0x80deb1fe,0x9bdc06a7,0xc19bf174,
    0xe49b69c1,0xefbe4786,0x0fc19dc6,0x240ca1cc,0x2de92c6f,0x4a7484aa,0x5cb0a9dc,0x76f988da,
    0x983e5152,0xa831c66d,0xb00327c8,0xbf597fc7,0xc6e00bf3,0xd5a79147,0x06ca6351,0x14292967,
    0x27b70a85,0x2e1b2138,0x4d2c6dfc,0x53380d13,0x650a7354,0x766a0abb,0x81c2c92e,0x92722c85,
    0xa2bfe8a1,0xa81a664b,0xc24b8b70,0xc76c51a3,0xd192e819,0xd6990624,0xf40e3585,0x106aa070,
    0x19a4c116,0x1e376c08,0x2748774c,0x34b0bcb5,0x391c0cb3,0x4ed8aa4a,0x5b9cca4f,0x682e6ff3,
    0x748f82ee,0x78a5636f,0x84c87814,0x8cc70208,0x90befffa,0xa4506ceb,0xbef9a3f7,0xc67178f2
};

void ep_sha256_transform(EP_SHA256_CTX *ctx, const unsigned char *data) {
    unsigned int a, b, c, d, e, f, g, h, i, j, t1, t2, m[64];
    for (i = 0, j = 0; i < 16; ++i, j += 4)
        m[i] = (data[j] << 24) | (data[j + 1] << 16) | (data[j + 2] << 8) | (data[j + 3]);
    for ( ; i < 64; ++i)
        m[i] = SIG1(m[i - 2]) + m[i - 7] + SIG0(m[i - 15]) + m[i - 16];
    a = ctx->state[0]; b = ctx->state[1]; c = ctx->state[2]; d = ctx->state[3];
    e = ctx->state[4]; f = ctx->state[5]; g = ctx->state[6]; h = ctx->state[7];
    for (i = 0; i < 64; ++i) {
        t1 = h + EP1(e) + CH(e,f,g) + sha256_k[i] + m[i];
        t2 = EP0(a) + MAJ(a,b,c);
        h = g; g = f; f = e; e = d + t1; d = c; c = b; b = a; a = t1 + t2;
    }
    ctx->state[0] += a; ctx->state[1] += b; ctx->state[2] += c; ctx->state[3] += d;
    ctx->state[4] += e; ctx->state[5] += f; ctx->state[6] += g; ctx->state[7] += h;
}

void ep_sha256_init(EP_SHA256_CTX *ctx) {
    ctx->datalen = 0; ctx->bitlen = 0;
    ctx->state[0] = 0x6a09e667; ctx->state[1] = 0xbb67ae85; ctx->state[2] = 0x3c6ef372; ctx->state[3] = 0xa54ff53a;
    ctx->state[4] = 0x510e527f; ctx->state[5] = 0x9b05688c; ctx->state[6] = 0x1f83d9ab; ctx->state[7] = 0x5be0cd19;
}

void ep_sha256_update(EP_SHA256_CTX *ctx, const unsigned char *data, size_t len) {
    for (size_t i = 0; i < len; ++i) {
        ctx->data[ctx->datalen] = data[i];
        ctx->datalen++;
        if (ctx->datalen == 64) {
            ep_sha256_transform(ctx, ctx->data);
            ctx->bitlen += 512;
            ctx->datalen = 0;
        }
    }
}

void ep_sha256_final(EP_SHA256_CTX *ctx, unsigned char *hash) {
    unsigned int i = ctx->datalen;
    if (ctx->datalen < 56) {
        ctx->data[i++] = 0x80;
        while (i < 56) ctx->data[i++] = 0x00;
    } else {
        ctx->data[i++] = 0x80;
        while (i < 64) ctx->data[i++] = 0x00;
        ep_sha256_transform(ctx, ctx->data);
        memset(ctx->data, 0, 56);
    }
    ctx->bitlen += ctx->datalen * 8;
    ctx->data[63] = ctx->bitlen; ctx->data[62] = ctx->bitlen >> 8;
    ctx->data[61] = ctx->bitlen >> 16; ctx->data[60] = ctx->bitlen >> 24;
    ctx->data[59] = ctx->bitlen >> 32; ctx->data[58] = ctx->bitlen >> 40;
    ctx->data[57] = ctx->bitlen >> 48; ctx->data[56] = ctx->bitlen >> 56;
    ep_sha256_transform(ctx, ctx->data);
    for (i = 0; i < 4; ++i) {
        hash[i]      = (ctx->state[0] >> (24 - i * 8)) & 0x000000ff;
        hash[i + 4]  = (ctx->state[1] >> (24 - i * 8)) & 0x000000ff;
        hash[i + 8]  = (ctx->state[2] >> (24 - i * 8)) & 0x000000ff;
        hash[i + 12] = (ctx->state[3] >> (24 - i * 8)) & 0x000000ff;
        hash[i + 16] = (ctx->state[4] >> (24 - i * 8)) & 0x000000ff;
        hash[i + 20] = (ctx->state[5] >> (24 - i * 8)) & 0x000000ff;
        hash[i + 24] = (ctx->state[6] >> (24 - i * 8)) & 0x000000ff;
        hash[i + 28] = (ctx->state[7] >> (24 - i * 8)) & 0x000000ff;
    }
}

char* ep_sha256(const char* s) {
    if (!s) s = "";
    EP_SHA256_CTX ctx;
    ep_sha256_init(&ctx);
    ep_sha256_update(&ctx, (const unsigned char*)s, strlen(s));
    unsigned char hash[32];
    ep_sha256_final(&ctx, hash);
    char* result = malloc(65);
    if (result) {
        for (int i = 0; i < 32; i++) {
            sprintf(result + (i * 2), "%02x", hash[i]);
        }
        result[64] = '\0';
    }
    return result;
}

typedef struct {
    unsigned int count[2];
    unsigned int state[4];
    unsigned char buffer[64];
} EP_MD5_CTX;

#define F(x,y,z) (((x) & (y)) | (~(x) & (z)))
#define G(x,y,z) (((x) & (z)) | ((y) & ~(z)))
#define H(x,y,z) ((x) ^ (y) ^ (z))
#define I(x,y,z) ((y) ^ ((x) | ~(z)))
#define ROTATE_LEFT(x,n) (((x) << (n)) | ((x) >> (32-(n))))

#define FF(a,b,c,d,x,s,ac) { (a) += F((b),(c),(d)) + (x) + (ac); (a) = ROTATE_LEFT((a),(s)); (a) += (b); }
#define GG(a,b,c,d,x,s,ac) { (a) += G((b),(c),(d)) + (x) + (ac); (a) = ROTATE_LEFT((a),(s)); (a) += (b); }
#define HH(a,b,c,d,x,s,ac) { (a) += H((b),(c),(d)) + (x) + (ac); (a) = ROTATE_LEFT((a),(s)); (a) += (b); }
#define II(a,b,c,d,x,s,ac) { (a) += I((b),(c),(d)) + (x) + (ac); (a) = ROTATE_LEFT((a),(s)); (a) += (b); }

void ep_md5_init(EP_MD5_CTX *ctx) {
    ctx->count[0] = ctx->count[1] = 0;
    ctx->state[0] = 0x67452301;
    ctx->state[1] = 0xefcdab89;
    ctx->state[2] = 0x98badcfe;
    ctx->state[3] = 0x10325476;
}

void ep_md5_transform(unsigned int state[4], const unsigned char block[64]) {
    unsigned int a = state[0], b = state[1], c = state[2], d = state[3], x[16];
    for (int i = 0, j = 0; i < 16; i++, j += 4)
        x[i] = (block[j]) | (block[j+1] << 8) | (block[j+2] << 16) | (block[j+3] << 24);
    FF(a, b, c, d, x[0], 7, 0xd76aa478); FF(d, a, b, c, x[1], 12, 0xe8c7b756); FF(c, d, a, b, x[2], 17, 0x242070db); FF(b, c, d, a, x[3], 22, 0xc1bdceee);
    FF(a, b, c, d, x[4], 7, 0xf57c0faf); FF(d, a, b, c, x[5], 12, 0x4787c62a); FF(c, d, a, b, x[6], 17, 0xa8304613); FF(b, c, d, a, x[7], 22, 0xfd469501);
    FF(a, b, c, d, x[8], 7, 0x698098d8); FF(d, a, b, c, x[9], 12, 0x8b44f7af); FF(c, d, a, b, x[10], 17, 0xffff5bb1); FF(b, c, d, a, x[11], 22, 0x895cd7be);
    FF(a, b, c, d, x[12], 7, 0x6b901122); FF(d, a, b, c, x[13], 12, 0xfd987193); FF(c, d, a, b, x[14], 17, 0xa679438e); FF(b, c, d, a, x[15], 22, 0x49b40821);
    GG(a, b, c, d, x[1], 5, 0xf61e2562); GG(d, a, b, c, x[6], 9, 0xc040b340); GG(c, d, a, b, x[11], 14, 0x265e5a51); GG(b, c, d, a, x[0], 20, 0xe9b6c7aa);
    GG(a, b, c, d, x[5], 5, 0xd62f105d); GG(d, a, b, c, x[10], 9, 0x02441453); GG(c, d, a, b, x[15], 14, 0xd8a1e681); GG(b, c, d, a, x[4], 20, 0xe7d3fbc8);
    GG(a, b, c, d, x[9], 5, 0x21e1cde6); GG(d, a, b, c, x[14], 9, 0xc33707d6); GG(c, d, a, b, x[3], 14, 0xf4d50d87); GG(b, c, d, a, x[8], 20, 0x455a14ed);
    GG(a, b, c, d, x[13], 5, 0xa9e3e905); GG(d, a, b, c, x[2], 9, 0xfcefa3f8); GG(c, d, a, b, x[7], 14, 0x676f02d9); GG(b, c, d, a, x[12], 20, 0x8d2a4c8a);
    HH(a, b, c, d, x[5], 4, 0xfffa3942); HH(d, a, b, c, x[8], 11, 0x8771f681); HH(c, d, a, b, x[11], 16, 0x6d9d6122); HH(b, c, d, a, x[14], 23, 0xfde5380c);
    HH(a, b, c, d, x[1], 4, 0xa4beea44); HH(d, a, b, c, x[4], 11, 0x4bdecfa9); HH(c, d, a, b, x[7], 16, 0xf6bb4b60); HH(b, c, d, a, x[10], 23, 0xbebfbc70);
    HH(a, b, c, d, x[13], 4, 0x289b7ec6); HH(d, a, b, c, x[0], 11, 0xeaa127fa); HH(c, d, a, b, x[3], 16, 0xd4ef3085); HH(b, c, d, a, x[6], 23, 0x04881d05);
    HH(a, b, c, d, x[9], 4, 0xd9d4d039); HH(d, a, b, c, x[12], 11, 0xe6db99e5); HH(c, d, a, b, x[15], 16, 0x1fa27cf8); HH(b, c, d, a, x[2], 23, 0xc4ac5665);
    II(a, b, c, d, x[0], 6, 0xf4292244); II(d, a, b, c, x[7], 10, 0x432aff97); II(c, d, a, b, x[14], 15, 0xab9423a7); II(b, c, d, a, x[5], 21, 0xfc93a039);
    II(a, b, c, d, x[12], 6, 0x655b59c3); II(d, a, b, c, x[3], 10, 0x8f0ccc92); II(c, d, a, b, x[10], 15, 0xffeff47d); II(b, c, d, a, x[1], 21, 0x85845dd1);
    II(a, b, c, d, x[8], 6, 0x6fa87e4f); II(d, a, b, c, x[15], 10, 0xfe2ce6e0); II(c, d, a, b, x[6], 15, 0xa3014314); II(b, c, d, a, x[13], 21, 0x4e0811a1);
    II(a, b, c, d, x[4], 6, 0xf7537e82); II(d, a, b, c, x[11], 10, 0xbd3af235); II(c, d, a, b, x[2], 15, 0x2ad7d2bb); II(b, c, d, a, x[9], 21, 0xeb86d391);
    state[0] += a; state[1] += b; state[2] += c; state[3] += d;
}

void ep_md5_update(EP_MD5_CTX *ctx, const unsigned char *input, size_t input_len) {
    unsigned int i = 0, index = (ctx->count[0] >> 3) & 0x3F, part_len = 64 - index;
    ctx->count[0] += input_len << 3;
    if (ctx->count[0] < (input_len << 3)) ctx->count[1]++;
    ctx->count[1] += input_len >> 29;
    if (input_len >= part_len) {
        memcpy(&ctx->buffer[index], input, part_len);
        ep_md5_transform(ctx->state, ctx->buffer);
        for (i = part_len; i + 63 < input_len; i += 64)
            ep_md5_transform(ctx->state, &input[i]);
        index = 0;
    }
    memcpy(&ctx->buffer[index], &input[i], input_len - i);
}

void ep_md5_final(EP_MD5_CTX *ctx, unsigned char digest[16]) {
    unsigned char bits[8];
    bits[0] = ctx->count[0]; bits[1] = ctx->count[0] >> 8; bits[2] = ctx->count[0] >> 16; bits[3] = ctx->count[0] >> 24;
    bits[4] = ctx->count[1]; bits[5] = ctx->count[1] >> 8; bits[6] = ctx->count[1] >> 16; bits[7] = ctx->count[1] >> 24;
    unsigned int index = (ctx->count[0] >> 3) & 0x3F, pad_len = (index < 56) ? (56 - index) : (120 - index);
    unsigned char padding[64];
    memset(padding, 0, 64); padding[0] = 0x80;
    ep_md5_update(ctx, padding, pad_len);
    ep_md5_update(ctx, bits, 8);
    for (int i = 0; i < 4; i++) {
        digest[i*4]     = ctx->state[i];
        digest[i*4 + 1] = ctx->state[i] >> 8;
        digest[i*4 + 2] = ctx->state[i] >> 16;
        digest[i*4 + 3] = ctx->state[i] >> 24;
    }
}

char* ep_md5(const char* s) {
    if (!s) s = "";
    EP_MD5_CTX ctx;
    ep_md5_init(&ctx);
    ep_md5_update(&ctx, (const unsigned char*)s, strlen(s));
    unsigned char hash[16];
    ep_md5_final(&ctx, hash);
    char* result = malloc(33);
    if (result) {
        for (int i = 0; i < 16; i++) {
            sprintf(result + (i * 2), "%02x", hash[i]);
        }
        result[32] = '\0';
    }
    return result;
}

char* read_file_content(const char* filepath) {
    char mode[3];
    mode[0] = 'r';
    mode[1] = 'b';
    mode[2] = '\0';
    FILE* f = fopen(filepath, mode);
    if (!f) {
        char* empty = malloc(1);
        if (empty) empty[0] = '\0';
        return empty;
    }
    fseek(f, 0, SEEK_END);
    long size = ftell(f);
    fseek(f, 0, SEEK_SET);
    char* buf = malloc(size + 1);
    if (!buf) {
        fclose(f);
        char* empty = malloc(1);
        if (empty) empty[0] = '\0';
        return empty;
    }
    size_t read_bytes = fread(buf, 1, size, f);
    buf[read_bytes] = '\0';
    fclose(f);
    return buf;
}

long long string_length(const char* s) {
    if (!s) return 0;
    return strlen(s);
}

long long get_character(const char* s, long long index) {
    if (!s) return 0;
    long long len = strlen(s);
    if (index < 0 || index >= len) return 0;
    return (unsigned char)s[index];
}

typedef struct {
    long long* data;
    long long capacity;
    long long length;
} EpList;

long long create_list(void) {
    EpList* list = malloc(sizeof(EpList));
    if (!list) return 0;
    list->capacity = 4;
    list->length = 0;
    list->data = malloc(list->capacity * sizeof(long long));
    return (long long)list;
}

long long append_list(long long list_ptr, long long value) {
    EpList* list = (EpList*)list_ptr;
    if (!list) return 0;
    if (list->length >= list->capacity) {
        list->capacity *= 2;
        list->data = realloc(list->data, list->capacity * sizeof(long long));
    }
    list->data[list->length] = value;
    list->length += 1;
    return value;
}

long long get_list(long long list_ptr, long long index) {
    EpList* list = (EpList*)list_ptr;
    if (!list || index < 0 || index >= list->length) return 0;
    return list->data[index];
}

long long set_list(long long list_ptr, long long index, long long value) {
    EpList* list = (EpList*)list_ptr;
    if (!list || index < 0 || index >= list->length) return 0;
    list->data[index] = value;
    return value;
}

long long length_list(long long list_ptr) {
    EpList* list = (EpList*)list_ptr;
    if (!list) return 0;
    return list->length;
}

void free_list(long long list_ptr) {
    EpList* list = (EpList*)list_ptr;
    if (!list) return;
    free(list->data);
    free(list);
}

int ep_argc = 0;
char** ep_argv = NULL;

void init_ep_args(int argc, char** argv) {
    ep_argc = argc;
    ep_argv = argv;
}

long long get_argument_count(void) {
    return ep_argc;
}

const char* get_argument(long long index) {
    if (index < 0 || index >= ep_argc) {
        char* empty = malloc(1);
        if (empty) empty[0] = '\0';
        return empty;
    }
    return ep_argv[index];
}

long long write_file_content(const char* filepath, const char* content) {
    char mode[3];
    mode[0] = 'w';
    mode[1] = 'b';
    mode[2] = '\0';
    FILE* f = fopen(filepath, mode);
    if (!f) return 0;
    size_t len = strlen(content);
    size_t written = fwrite(content, 1, len, f);
    fclose(f);
    return written == len ? 1 : 0;
}

long long run_command(const char* command) {
    if (!command) return -1;
    return system(command);
}

char* substring(const char* s, long long start, long long len) {
    if (!s) {
        char* empty = malloc(1);
        if (empty) empty[0] = '\0';
        return empty;
    }
    long long total_len = strlen(s);
    if (start < 0 || start >= total_len || len <= 0) {
        char* empty = malloc(1);
        if (empty) empty[0] = '\0';
        return empty;
    }
    if (start + len > total_len) {
        len = total_len - start;
    }
    char* sub = malloc(len + 1);
    if (!sub) {
        char* empty = malloc(1);
        if (empty) empty[0] = '\0';
        return empty;
    }
    strncpy(sub, s + start, len);
    sub[len] = '\0';
    return sub;
}

char* string_from_list(long long list_ptr) {
    EpList* list = (EpList*)list_ptr;
    if (!list) {
        char* empty = malloc(1);
        if (empty) empty[0] = '\0';
        return empty;
    }
    char* s = malloc(list->length + 1);
    if (!s) {
        char* empty = malloc(1);
        if (empty) empty[0] = '\0';
        return empty;
    }
    for (long long i = 0; i < list->length; i++) {
        s[i] = (char)list->data[i];
    }
    s[list->length] = '\0';
    return s;
}

long long pop_list(long long list_ptr) {
    EpList* list = (EpList*)list_ptr;
    if (!list || list->length <= 0) return 0;
    list->length -= 1;
    return list->data[list->length];
}

long long display_string(const char* s) {
    if (s) puts(s);
    return 0;
}


/* User Function Prototypes */
long long create_token(long long, long long, long long, long long);
long long get_token_type(long long);
long long get_token_value(long long);
long long get_token_line(long long);
long long get_token_col(long long);
long long match_next_word(long long, long long, long long);
long long tokenize_source(long long);
long long parse_int(long long);
long long make_node_int(long long);
long long make_node_str(long long);
long long make_node_ident(long long);
long long make_node_binary(long long, long long, long long);
long long make_node_comp(long long, long long, long long);
long long make_node_call(long long, long long);
long long make_node_set(long long, long long);
long long make_node_return(long long);
long long make_node_display(long long);
long long make_node_if(long long, long long, long long);
long long make_node_repeat_while(long long, long long);
long long make_node_func(long long, long long, long long);
long long make_node_program(long long, long long);
long long make_node_logical(long long, long long, long long);
long long create_parser_state(long long);
long long get_state_tokens(long long);
long long get_state_pos(long long);
long long set_state_pos(long long, long long);
long long get_eof_token();
long long peek_token(long long);
long long advance_token(long long);
long long expect_token_type(long long, long long);
long long get_token_precedence(long long);
long long parse_program(long long);
long long parse_function(long long);
long long parse_block(long long);
long long parse_statement(long long);
long long parse_expr(long long, long long);
long long map_get(long long, long long, long long);
long long map_put(long long, long long, long long, long long);
long long string_concat(long long, long long);
long long int_to_string(long long);
long long escape_string(long long);
long long join_strings(long long);
long long create_codegen_state();
long long emit(long long, long long);
long long add_string_literal(long long, long long);
long long get_new_label(long long, long long);
long long analyze_return_types(long long, long long);
long long collect_var_types(long long, long long, long long, long long);
long long determine_ret_type(long long, long long, long long, long long);
long long infer_type(long long, long long, long long, long long);
long long gen_function(long long, long long);
long long gen_statement(long long, long long, long long, long long);
long long gen_expr(long long, long long, long long, long long);
long long get_c_runtime_source();
long long get_c_main_source();
long long generate_c(long long);
long long get_file_stem(long long);
long long get_file_dir(long long);
long long contains_string(long long, long long);
long long resolve_import_path(long long, long long);
long long parse_all_modules(long long, long long, long long);
long long _main();

long long create_token(long long type, long long value, long long line, long long col) {
    long long tok = 0;
    long long ok = 0;
    long long ret_val = 0;

    {
        long long tmp_val = create_list();
        free_list(tok);
        tok = tmp_val;
    }
    ok = append_list(tok, type);
    ok = append_list(tok, value);
    ok = append_list(tok, line);
    ok = append_list(tok, col);
    ret_val = tok;
    tok = 0;
    goto L_cleanup;
L_cleanup:
    free_list(tok);
    return ret_val;
}

long long get_token_type(long long tok) {
    long long ret_val = 0;

    ret_val = get_list(tok, 0);
    goto L_cleanup;
L_cleanup:
    return ret_val;
}

long long get_token_value(long long tok) {
    long long ret_val = 0;

    ret_val = get_list(tok, 1);
    goto L_cleanup;
L_cleanup:
    return ret_val;
}

long long get_token_line(long long tok) {
    long long ret_val = 0;

    ret_val = get_list(tok, 2);
    goto L_cleanup;
L_cleanup:
    return ret_val;
}

long long get_token_col(long long tok) {
    long long ret_val = 0;

    ret_val = get_list(tok, 3);
    goto L_cleanup;
L_cleanup:
    return ret_val;
}

long long match_next_word(long long source, long long start_pos, long long next_word) {
    long long p = 0;
    long long s_len = 0;
    long long loop = 0;
    long long ch = 0;
    long long nw_len = 0;
    long long idx = 0;
    long long matches = 0;
    long long ch1 = 0;
    long long ch2 = 0;
    long long next_ch = 0;
    long long is_id_part = 0;
    long long ret_val = 0;

    p = start_pos;
    s_len = string_length((char*)source);
    loop = 1;
    while (((p < s_len) && (loop == 1))) {
    ch = get_character((char*)source, p);
    if (((ch == 32) || (ch == 9))) {
    p = (p + 1);
    } else {
    loop = 0;
    }
    }
    nw_len = string_length((char*)next_word);
    if (((p + nw_len) > s_len)) {
    ret_val = 0;
    goto L_cleanup;
    }
    idx = 0;
    matches = 1;
    while (((idx < nw_len) && (matches == 1))) {
    ch1 = get_character((char*)source, (p + idx));
    ch2 = get_character((char*)next_word, idx);
    if ((ch1 != ch2)) {
    matches = 0;
    }
    idx = (idx + 1);
    }
    if ((matches == 1)) {
    next_ch = get_character((char*)source, (p + nw_len));
    is_id_part = (((((next_ch > 96) && (next_ch < 123)) || ((next_ch > 64) && (next_ch < 91))) || ((next_ch > 47) && (next_ch < 58))) || (next_ch == 95));
    if ((is_id_part == 0)) {
    ret_val = (p + nw_len);
    goto L_cleanup;
    }
    }
    ret_val = 0;
    goto L_cleanup;
L_cleanup:
    return ret_val;
}

long long tokenize_source(long long source) {
    long long tokens = 0;
    long long source_len = 0;
    long long pos = 0;
    long long current_line = 0;
    long long current_col = 0;
    long long indent_stack = 0;
    long long ok = 0;
    long long at_line_start = 0;
    long long spaces = 0;
    long long space_loop = 0;
    long long ch = 0;
    long long next_ch = 0;
    long long stack_len = 0;
    long long last_indent = 0;
    long long tok = 0;
    long long loop_dedent = 0;
    long long s_len = 0;
    long long top_indent = 0;
    long long popped = 0;
    long long dummy = 0;
    long long c = 0;
    long long tokens_len = 0;
    long long should_emit_nl = 0;
    long long last_tok = 0;
    long long num_start = 0;
    long long num_len = 0;
    long long num_str = 0;
    long long is_id_start = 0;
    long long id_start = 0;
    long long id_loop = 0;
    long long is_id_part = 0;
    long long id_len = 0;
    long long id_str = 0;
    long long tok_type = 0;
    long long is_multi_phrase = 0;
    long long next_p = 0;
    long long next_p2 = 0;
    long long next_p3 = 0;
    long long start_col = 0;
    long long str_chars = 0;
    long long str_loop = 0;
    long long closed = 0;
    long long esc_ch = 0;
    long long str_val = 0;
    long long sym_type = 0;
    long long sym_val = 0;
    long long sym_len = 0;
    long long next_c = 0;
    long long ret_val = 0;

    {
        long long tmp_val = create_list();
        free_list(tokens);
        tokens = tmp_val;
    }
    source_len = string_length((char*)source);
    pos = 0;
    current_line = 1;
    current_col = 1;
    {
        long long tmp_val = create_list();
        free_list(indent_stack);
        indent_stack = tmp_val;
    }
    ok = append_list(indent_stack, 0);
    at_line_start = 1;
    while ((pos < source_len)) {
    if ((at_line_start == 1)) {
    spaces = 0;
    space_loop = 1;
    while (((pos < source_len) && (space_loop == 1))) {
    ch = get_character((char*)source, pos);
    if ((ch == 32)) {
    spaces = (spaces + 1);
    pos = (pos + 1);
    current_col = (current_col + 1);
    } else {
    if ((ch == 9)) {
    spaces = (spaces + 4);
    pos = (pos + 1);
    current_col = (current_col + 4);
    } else {
    space_loop = 0;
    }
    }
    }
    next_ch = get_character((char*)source, pos);
    if (((((next_ch != 10) && (next_ch != 13)) && (next_ch != 35)) && (pos < source_len))) {
    stack_len = length_list(indent_stack);
    last_indent = get_list(indent_stack, (stack_len - 1));
    if ((spaces > last_indent)) {
    ok = append_list(indent_stack, spaces);
    tok = (create_token(29, (long long)"INDENT", current_line, current_col) + 0);
    ok = append_list(tokens, tok);
    } else {
    if ((spaces < last_indent)) {
    loop_dedent = 1;
    while ((loop_dedent == 1)) {
    s_len = length_list(indent_stack);
    top_indent = get_list(indent_stack, (s_len - 1));
    if ((spaces < top_indent)) {
    popped = pop_list(indent_stack);
    tok = (create_token(30, (long long)"DEDENT", current_line, current_col) + 0);
    ok = append_list(tokens, tok);
    } else {
    loop_dedent = 0;
    }
    }
    }
    }
    }
    at_line_start = 0;
    }
    if ((pos > (source_len - 1))) {
    dummy = 0;
    } else {
    c = get_character((char*)source, pos);
    if (((c == 32) || (c == 9))) {
    pos = (pos + 1);
    current_col = (current_col + 1);
    } else {
    if (((c == 10) || (c == 13))) {
    pos = (pos + 1);
    if (((c == 13) && (get_character((char*)source, pos) == 10))) {
    pos = (pos + 1);
    }
    tokens_len = length_list(tokens);
    should_emit_nl = 1;
    if ((tokens_len > 0)) {
    last_tok = get_list(tokens, (tokens_len - 1));
    if ((get_token_type(last_tok) == 28)) {
    should_emit_nl = 0;
    }
    }
    if (((should_emit_nl == 1) && (tokens_len > 0))) {
    tok = (create_token(28, (long long)"\n", current_line, current_col) + 0);
    ok = append_list(tokens, tok);
    }
    current_line = (current_line + 1);
    current_col = 1;
    at_line_start = 1;
    } else {
    if ((c == 35)) {
    pos = (pos + 1);
    while ((((pos < source_len) && (get_character((char*)source, pos) != 10)) && (get_character((char*)source, pos) != 13))) {
    pos = (pos + 1);
    }
    } else {
    if (((c > 47) && (c < 58))) {
    num_start = pos;
    while ((((pos < source_len) && (get_character((char*)source, pos) > 47)) && (get_character((char*)source, pos) < 58))) {
    pos = (pos + 1);
    current_col = (current_col + 1);
    }
    num_len = (pos - num_start);
    num_str = ((long long)substring((char*)source, num_start, num_len) + 0);
    tok = (create_token(25, num_str, current_line, (current_col - num_len)) + 0);
    ok = append_list(tokens, tok);
    } else {
    is_id_start = ((((c > 96) && (c < 123)) || ((c > 64) && (c < 91))) || (c == 95));
    if (is_id_start) {
    id_start = pos;
    id_loop = 1;
    while (((pos < source_len) && (id_loop == 1))) {
    ch = get_character((char*)source, pos);
    is_id_part = (((((ch > 96) && (ch < 123)) || ((ch > 64) && (ch < 91))) || ((ch > 47) && (ch < 58))) || (ch == 95));
    if (is_id_part) {
    pos = (pos + 1);
    current_col = (current_col + 1);
    } else {
    id_loop = 0;
    }
    }
    id_len = (pos - id_start);
    id_str = ((long long)substring((char*)source, id_start, id_len) + 0);
    tok_type = 27;
    is_multi_phrase = 0;
    if ((strcmp((char*)(long long)"multiplied", (char*)id_str) == 0)) {
    next_p = match_next_word(source, pos, (long long)"by");
    if ((next_p > 0)) {
    tok_type = 14;
    id_str = (long long)"*";
    current_col = (current_col + (next_p - pos));
    pos = next_p;
    is_multi_phrase = 1;
    }
    }
    if ((is_multi_phrase == 0)) {
    if ((strcmp((char*)(long long)"divided", (char*)id_str) == 0)) {
    next_p = match_next_word(source, pos, (long long)"by");
    if ((next_p > 0)) {
    tok_type = 15;
    id_str = (long long)"/";
    current_col = (current_col + (next_p - pos));
    pos = next_p;
    is_multi_phrase = 1;
    }
    }
    }
    if ((is_multi_phrase == 0)) {
    if ((strcmp((char*)(long long)"is", (char*)id_str) == 0)) {
    next_p = match_next_word(source, pos, (long long)"not");
    if ((next_p > 0)) {
    next_p2 = match_next_word(source, next_p, (long long)"equal");
    if ((next_p2 > 0)) {
    next_p3 = match_next_word(source, next_p2, (long long)"to");
    if ((next_p3 > 0)) {
    tok_type = 19;
    id_str = (long long)"!=";
    current_col = (current_col + (next_p3 - pos));
    pos = next_p3;
    is_multi_phrase = 1;
    }
    }
    }
    if ((is_multi_phrase == 0)) {
    next_p = match_next_word(source, pos, (long long)"less");
    if ((next_p > 0)) {
    next_p2 = match_next_word(source, next_p, (long long)"than");
    if ((next_p2 > 0)) {
    tok_type = 16;
    id_str = (long long)"<";
    current_col = (current_col + (next_p2 - pos));
    pos = next_p2;
    is_multi_phrase = 1;
    }
    }
    }
    if ((is_multi_phrase == 0)) {
    next_p = match_next_word(source, pos, (long long)"greater");
    if ((next_p > 0)) {
    next_p2 = match_next_word(source, next_p, (long long)"than");
    if ((next_p2 > 0)) {
    tok_type = 17;
    id_str = (long long)">";
    current_col = (current_col + (next_p2 - pos));
    pos = next_p2;
    is_multi_phrase = 1;
    }
    }
    }
    if ((is_multi_phrase == 0)) {
    next_p = match_next_word(source, pos, (long long)"equal");
    if ((next_p > 0)) {
    next_p2 = match_next_word(source, next_p, (long long)"to");
    if ((next_p2 > 0)) {
    tok_type = 18;
    id_str = (long long)"==";
    current_col = (current_col + (next_p2 - pos));
    pos = next_p2;
    is_multi_phrase = 1;
    }
    }
    }
    }
    }
    if ((is_multi_phrase == 0)) {
    if ((strcmp((char*)(long long)"and", (char*)id_str) == 0)) {
    next_p = match_next_word(source, pos, (long long)"also");
    if ((next_p > 0)) {
    tok_type = 20;
    id_str = (long long)"&&";
    current_col = (current_col + (next_p - pos));
    pos = next_p;
    is_multi_phrase = 1;
    }
    }
    }
    if ((is_multi_phrase == 0)) {
    if ((strcmp((char*)(long long)"or", (char*)id_str) == 0)) {
    next_p = match_next_word(source, pos, (long long)"else");
    if ((next_p > 0)) {
    tok_type = 21;
    id_str = (long long)"||";
    current_col = (current_col + (next_p - pos));
    pos = next_p;
    is_multi_phrase = 1;
    }
    }
    }
    if ((is_multi_phrase == 0)) {
    if ((strcmp((char*)(long long)"define", (char*)id_str) == 0)) {
    tok_type = 1;
    }
    if ((strcmp((char*)(long long)"set", (char*)id_str) == 0)) {
    tok_type = 2;
    }
    if ((strcmp((char*)(long long)"to", (char*)id_str) == 0)) {
    tok_type = 3;
    }
    if ((strcmp((char*)(long long)"if", (char*)id_str) == 0)) {
    tok_type = 4;
    }
    if ((strcmp((char*)(long long)"else", (char*)id_str) == 0)) {
    tok_type = 5;
    }
    if ((strcmp((char*)(long long)"return", (char*)id_str) == 0)) {
    tok_type = 6;
    }
    if ((strcmp((char*)(long long)"display", (char*)id_str) == 0)) {
    tok_type = 7;
    }
    if ((strcmp((char*)(long long)"repeat", (char*)id_str) == 0)) {
    tok_type = 8;
    }
    if ((strcmp((char*)(long long)"while", (char*)id_str) == 0)) {
    tok_type = 9;
    }
    if ((strcmp((char*)(long long)"import", (char*)id_str) == 0)) {
    tok_type = 32;
    }
    if ((strcmp((char*)(long long)"with", (char*)id_str) == 0)) {
    tok_type = 10;
    }
    if ((strcmp((char*)(long long)"and", (char*)id_str) == 0)) {
    tok_type = 11;
    }
    if ((strcmp((char*)(long long)"plus", (char*)id_str) == 0)) {
    tok_type = 12;
    }
    if ((strcmp((char*)(long long)"minus", (char*)id_str) == 0)) {
    tok_type = 13;
    }
    if ((strcmp((char*)(long long)"equals", (char*)id_str) == 0)) {
    tok_type = 18;
    }
    }
    tok = (create_token(tok_type, id_str, current_line, (current_col - id_len)) + 0);
    ok = append_list(tokens, tok);
    } else {
    if ((c == 34)) {
    start_col = current_col;
    pos = (pos + 1);
    current_col = (current_col + 1);
    {
        long long tmp_val = create_list();
        free_list(str_chars);
        str_chars = tmp_val;
    }
    str_loop = 1;
    closed = 0;
    while (((pos < source_len) && (str_loop == 1))) {
    ch = get_character((char*)source, pos);
    if ((ch == 34)) {
    closed = 1;
    str_loop = 0;
    pos = (pos + 1);
    current_col = (current_col + 1);
    } else {
    if ((ch == 92)) {
    pos = (pos + 1);
    current_col = (current_col + 1);
    if ((pos < source_len)) {
    esc_ch = get_character((char*)source, pos);
    if ((esc_ch == 110)) {
    ok = append_list(str_chars, 10);
    } else {
    if ((esc_ch == 116)) {
    ok = append_list(str_chars, 9);
    } else {
    if ((esc_ch == 114)) {
    ok = append_list(str_chars, 13);
    } else {
    if ((esc_ch == 34)) {
    ok = append_list(str_chars, 34);
    } else {
    if ((esc_ch == 92)) {
    ok = append_list(str_chars, 92);
    } else {
    ok = append_list(str_chars, 92);
    ok = append_list(str_chars, esc_ch);
    }
    }
    }
    }
    }
    pos = (pos + 1);
    current_col = (current_col + 1);
    } else {
    printf("%s\n", (char*)(long long)"Lexer Error: Unterminated string literal at escape sequence");
    str_loop = 0;
    }
    } else {
    if (((ch == 10) || (ch == 13))) {
    printf("%s\n", (char*)(long long)"Lexer Error: Unterminated string literal");
    str_loop = 0;
    } else {
    ok = append_list(str_chars, ch);
    pos = (pos + 1);
    current_col = (current_col + 1);
    }
    }
    }
    }
    if ((closed == 0)) {
    printf("%s\n", (char*)(long long)"Lexer Error: Unterminated string literal");
    ret_val = 0;
    goto L_cleanup;
    }
    str_val = (long long)string_from_list(str_chars);
    tok = (create_token(26, str_val, current_line, start_col) + 0);
    ok = append_list(tokens, tok);
    } else {
    sym_type = 0;
    sym_val = (long long)"";
    sym_len = 1;
    next_c = get_character((char*)source, (pos + 1));
    if (((c == 61) && (next_c == 61))) {
    sym_type = 18;
    sym_val = (long long)"==";
    sym_len = 2;
    }
    if (((c == 33) && (next_c == 61))) {
    sym_type = 19;
    sym_val = (long long)"!=";
    sym_len = 2;
    }
    if (((c == 38) && (next_c == 38))) {
    sym_type = 20;
    sym_val = (long long)"&&";
    sym_len = 2;
    }
    if (((c == 124) && (next_c == 124))) {
    sym_type = 21;
    sym_val = (long long)"||";
    sym_len = 2;
    }
    if ((sym_type == 0)) {
    if ((c == 58)) {
    sym_type = 22;
    sym_val = (long long)":";
    }
    if ((c == 40)) {
    sym_type = 23;
    sym_val = (long long)"(";
    }
    if ((c == 41)) {
    sym_type = 24;
    sym_val = (long long)")";
    }
    if ((c == 43)) {
    sym_type = 12;
    sym_val = (long long)"+";
    }
    if ((c == 45)) {
    sym_type = 13;
    sym_val = (long long)"-";
    }
    if ((c == 42)) {
    sym_type = 14;
    sym_val = (long long)"*";
    }
    if ((c == 47)) {
    sym_type = 15;
    sym_val = (long long)"/";
    }
    if ((c == 60)) {
    sym_type = 16;
    sym_val = (long long)"<";
    }
    if ((c == 62)) {
    sym_type = 17;
    sym_val = (long long)">";
    }
    }
    if ((sym_type > 0)) {
    tok = (create_token(sym_type, sym_val, current_line, current_col) + 0);
    ok = append_list(tokens, tok);
    pos = (pos + sym_len);
    current_col = (current_col + sym_len);
    } else {
    printf("%s\n", (char*)(long long)"Lexer Error: Unknown symbol character code:");
    printf("%lld\n", c);
    pos = (pos + 1);
    current_col = (current_col + 1);
    }
    }
    }
    }
    }
    }
    }
    }
    }
    stack_len = length_list(indent_stack);
    while ((stack_len > 1)) {
    tok = (create_token(30, (long long)"DEDENT", current_line, current_col) + 0);
    ok = append_list(tokens, tok);
    popped = pop_list(indent_stack);
    stack_len = (stack_len - 1);
    }
    tok = (create_token(31, (long long)"EOF", current_line, current_col) + 0);
    ok = append_list(tokens, tok);
    ret_val = tokens;
    tokens = 0;
    goto L_cleanup;
L_cleanup:
    free_list(tokens);
    free_list(indent_stack);
    free_list(str_chars);
    return ret_val;
}

long long parse_int(long long s) {
    long long val = 0;
    long long len = 0;
    long long idx = 0;
    long long ch = 0;
    long long digit = 0;
    long long ret_val = 0;

    val = 0;
    len = string_length((char*)s);
    idx = 0;
    while ((idx < len)) {
    ch = get_character((char*)s, idx);
    digit = (ch - 48);
    val = ((val * 10) + digit);
    idx = (idx + 1);
    }
    ret_val = val;
    goto L_cleanup;
L_cleanup:
    return ret_val;
}

long long make_node_int(long long val) {
    long long node = 0;
    long long ok = 0;
    long long ret_val = 0;

    {
        long long tmp_val = create_list();
        free_list(node);
        node = tmp_val;
    }
    ok = append_list(node, 1);
    ok = append_list(node, val);
    ret_val = node;
    node = 0;
    goto L_cleanup;
L_cleanup:
    free_list(node);
    return ret_val;
}

long long make_node_str(long long val) {
    long long node = 0;
    long long ok = 0;
    long long ret_val = 0;

    {
        long long tmp_val = create_list();
        free_list(node);
        node = tmp_val;
    }
    ok = append_list(node, 2);
    ok = append_list(node, val);
    ret_val = node;
    node = 0;
    goto L_cleanup;
L_cleanup:
    free_list(node);
    return ret_val;
}

long long make_node_ident(long long name) {
    long long node = 0;
    long long ok = 0;
    long long ret_val = 0;

    {
        long long tmp_val = create_list();
        free_list(node);
        node = tmp_val;
    }
    ok = append_list(node, 3);
    ok = append_list(node, name);
    ret_val = node;
    node = 0;
    goto L_cleanup;
L_cleanup:
    free_list(node);
    return ret_val;
}

long long make_node_binary(long long left, long long op, long long right) {
    long long node = 0;
    long long ok = 0;
    long long ret_val = 0;

    {
        long long tmp_val = create_list();
        free_list(node);
        node = tmp_val;
    }
    ok = append_list(node, 4);
    ok = append_list(node, left);
    ok = append_list(node, op);
    ok = append_list(node, right);
    ret_val = node;
    node = 0;
    goto L_cleanup;
L_cleanup:
    free_list(node);
    return ret_val;
}

long long make_node_comp(long long left, long long op, long long right) {
    long long node = 0;
    long long ok = 0;
    long long ret_val = 0;

    {
        long long tmp_val = create_list();
        free_list(node);
        node = tmp_val;
    }
    ok = append_list(node, 5);
    ok = append_list(node, left);
    ok = append_list(node, op);
    ok = append_list(node, right);
    ret_val = node;
    node = 0;
    goto L_cleanup;
L_cleanup:
    free_list(node);
    return ret_val;
}

long long make_node_call(long long name, long long args) {
    long long node = 0;
    long long ok = 0;
    long long ret_val = 0;

    {
        long long tmp_val = create_list();
        free_list(node);
        node = tmp_val;
    }
    ok = append_list(node, 6);
    ok = append_list(node, name);
    ok = append_list(node, args);
    ret_val = node;
    node = 0;
    goto L_cleanup;
L_cleanup:
    free_list(node);
    return ret_val;
}

long long make_node_set(long long var, long long expr) {
    long long node = 0;
    long long ok = 0;
    long long ret_val = 0;

    {
        long long tmp_val = create_list();
        free_list(node);
        node = tmp_val;
    }
    ok = append_list(node, 7);
    ok = append_list(node, var);
    ok = append_list(node, expr);
    ret_val = node;
    node = 0;
    goto L_cleanup;
L_cleanup:
    free_list(node);
    return ret_val;
}

long long make_node_return(long long expr) {
    long long node = 0;
    long long ok = 0;
    long long ret_val = 0;

    {
        long long tmp_val = create_list();
        free_list(node);
        node = tmp_val;
    }
    ok = append_list(node, 8);
    ok = append_list(node, expr);
    ret_val = node;
    node = 0;
    goto L_cleanup;
L_cleanup:
    free_list(node);
    return ret_val;
}

long long make_node_display(long long expr) {
    long long node = 0;
    long long ok = 0;
    long long ret_val = 0;

    {
        long long tmp_val = create_list();
        free_list(node);
        node = tmp_val;
    }
    ok = append_list(node, 9);
    ok = append_list(node, expr);
    ret_val = node;
    node = 0;
    goto L_cleanup;
L_cleanup:
    free_list(node);
    return ret_val;
}

long long make_node_if(long long cond, long long then_b, long long else_b) {
    long long node = 0;
    long long ok = 0;
    long long ret_val = 0;

    {
        long long tmp_val = create_list();
        free_list(node);
        node = tmp_val;
    }
    ok = append_list(node, 10);
    ok = append_list(node, cond);
    ok = append_list(node, then_b);
    ok = append_list(node, else_b);
    ret_val = node;
    node = 0;
    goto L_cleanup;
L_cleanup:
    free_list(node);
    return ret_val;
}

long long make_node_repeat_while(long long cond, long long body) {
    long long node = 0;
    long long ok = 0;
    long long ret_val = 0;

    {
        long long tmp_val = create_list();
        free_list(node);
        node = tmp_val;
    }
    ok = append_list(node, 11);
    ok = append_list(node, cond);
    ok = append_list(node, body);
    ret_val = node;
    node = 0;
    goto L_cleanup;
L_cleanup:
    free_list(node);
    return ret_val;
}

long long make_node_func(long long name, long long params, long long body) {
    long long node = 0;
    long long ok = 0;
    long long ret_val = 0;

    {
        long long tmp_val = create_list();
        free_list(node);
        node = tmp_val;
    }
    ok = append_list(node, 12);
    ok = append_list(node, name);
    ok = append_list(node, params);
    ok = append_list(node, body);
    ret_val = node;
    node = 0;
    goto L_cleanup;
L_cleanup:
    free_list(node);
    return ret_val;
}

long long make_node_program(long long imports, long long funcs) {
    long long node = 0;
    long long ok = 0;
    long long ret_val = 0;

    {
        long long tmp_val = create_list();
        free_list(node);
        node = tmp_val;
    }
    ok = append_list(node, 13);
    ok = append_list(node, imports);
    ok = append_list(node, funcs);
    ret_val = node;
    node = 0;
    goto L_cleanup;
L_cleanup:
    free_list(node);
    return ret_val;
}

long long make_node_logical(long long left, long long op, long long right) {
    long long node = 0;
    long long ok = 0;
    long long ret_val = 0;

    {
        long long tmp_val = create_list();
        free_list(node);
        node = tmp_val;
    }
    ok = append_list(node, 14);
    ok = append_list(node, left);
    ok = append_list(node, op);
    ok = append_list(node, right);
    ret_val = node;
    node = 0;
    goto L_cleanup;
L_cleanup:
    free_list(node);
    return ret_val;
}

long long create_parser_state(long long tokens) {
    long long state = 0;
    long long ok = 0;
    long long ret_val = 0;

    {
        long long tmp_val = create_list();
        free_list(state);
        state = tmp_val;
    }
    ok = append_list(state, tokens);
    ok = append_list(state, 0);
    ret_val = state;
    state = 0;
    goto L_cleanup;
L_cleanup:
    free_list(state);
    return ret_val;
}

long long get_state_tokens(long long state) {
    long long ret_val = 0;

    ret_val = get_list(state, 0);
    goto L_cleanup;
L_cleanup:
    return ret_val;
}

long long get_state_pos(long long state) {
    long long ret_val = 0;

    ret_val = get_list(state, 1);
    goto L_cleanup;
L_cleanup:
    return ret_val;
}

long long set_state_pos(long long state, long long new_pos) {
    long long ret_val = 0;

    ret_val = set_list(state, 1, new_pos);
    goto L_cleanup;
L_cleanup:
    return ret_val;
}

long long get_eof_token() {
    long long tok = 0;
    long long ret_val = 0;

    tok = (create_token(31, (long long)"EOF", 1, 1) + 0);
    ret_val = tok;
    goto L_cleanup;
L_cleanup:
    return ret_val;
}

long long peek_token(long long state) {
    long long tokens = 0;
    long long pos = 0;
    long long tokens_len = 0;
    long long ret_val = 0;

    tokens = get_state_tokens(state);
    pos = get_state_pos(state);
    tokens_len = length_list(tokens);
    if ((pos < tokens_len)) {
    ret_val = get_list(tokens, pos);
    goto L_cleanup;
    }
    ret_val = get_eof_token();
    goto L_cleanup;
L_cleanup:
    return ret_val;
}

long long advance_token(long long state) {
    long long tokens = 0;
    long long pos = 0;
    long long tokens_len = 0;
    long long tok = 0;
    long long ok = 0;
    long long ret_val = 0;

    tokens = get_state_tokens(state);
    pos = get_state_pos(state);
    tokens_len = length_list(tokens);
    if ((pos < tokens_len)) {
    tok = get_list(tokens, pos);
    ok = set_state_pos(state, (pos + 1));
    ret_val = tok;
    goto L_cleanup;
    }
    ret_val = get_eof_token();
    goto L_cleanup;
L_cleanup:
    return ret_val;
}

long long expect_token_type(long long state, long long expected_type) {
    long long tok = 0;
    long long actual_type = 0;
    long long ret_val = 0;

    tok = advance_token(state);
    actual_type = get_token_type(tok);
    if ((actual_type == expected_type)) {
    ret_val = 1;
    goto L_cleanup;
    }
    printf("%s\n", (char*)(long long)"Parser Error: Expected token type:");
    printf("%lld\n", expected_type);
    printf("%s\n", (char*)(long long)"Found type:");
    printf("%lld\n", actual_type);
    printf("%s\n", (char*)(long long)"At line:");
    printf("%lld\n", get_token_line(tok));
    ret_val = 0;
    goto L_cleanup;
L_cleanup:
    return ret_val;
}

long long get_token_precedence(long long tok) {
    long long t = 0;
    long long ret_val = 0;

    t = get_token_type(tok);
    if ((t == 21)) {
    ret_val = 1;
    goto L_cleanup;
    }
    if ((t == 20)) {
    ret_val = 2;
    goto L_cleanup;
    }
    if (((((t == 16) || (t == 17)) || (t == 18)) || (t == 19))) {
    ret_val = 3;
    goto L_cleanup;
    }
    if (((t == 12) || (t == 13))) {
    ret_val = 4;
    goto L_cleanup;
    }
    if (((t == 14) || (t == 15))) {
    ret_val = 5;
    goto L_cleanup;
    }
    if ((t == 23)) {
    ret_val = 6;
    goto L_cleanup;
    }
    ret_val = 0;
    goto L_cleanup;
L_cleanup:
    return ret_val;
}

long long parse_program(long long state) {
    long long imports = 0;
    long long funcs = 0;
    long long loop = 0;
    long long tok = 0;
    long long t = 0;
    long long dummy = 0;
    long long tok_path = 0;
    long long path_type = 0;
    long long path_val = 0;
    long long ok = 0;
    long long func = 0;
    long long prog = 0;
    long long ret_val = 0;

    imports = (create_list() + 0);
    funcs = (create_list() + 0);
    loop = 1;
    while ((loop == 1)) {
    tok = peek_token(state);
    t = get_token_type(tok);
    if ((t == 31)) {
    loop = 0;
    } else {
    if ((t == 28)) {
    dummy = advance_token(state);
    } else {
    if ((t == 32)) {
    dummy = advance_token(state);
    tok_path = advance_token(state);
    path_type = get_token_type(tok_path);
    if ((path_type == 26)) {
    path_val = get_token_value(tok_path);
    ok = append_list(imports, path_val);
    } else {
    printf("%s\n", (char*)(long long)"Parser Error: Expected string literal after import at line:");
    printf("%lld\n", get_token_line(tok_path));
    loop = 0;
    }
    } else {
    if ((t == 1)) {
    func = (parse_function(state) + 0);
    ok = append_list(funcs, func);
    } else {
    printf("%s\n", (char*)(long long)"Parser Error: Unexpected token at top level:");
    printf("%lld\n", t);
    loop = 0;
    }
    }
    }
    }
    }
    prog = (make_node_program(imports, funcs) + 0);
    ret_val = prog;
    goto L_cleanup;
L_cleanup:
    return ret_val;
}

long long parse_function(long long state) {
    long long ok = 0;
    long long tok_name = 0;
    long long name = 0;
    long long params = 0;
    long long next_tok = 0;
    long long dummy = 0;
    long long tok_param = 0;
    long long loop = 0;
    long long tok_and = 0;
    long long tok_nl = 0;
    long long body = 0;
    long long ret_val = 0;

    ok = expect_token_type(state, 1);
    tok_name = advance_token(state);
    name = get_token_value(tok_name);
    params = (create_list() + 0);
    next_tok = peek_token(state);
    if ((get_token_type(next_tok) == 10)) {
    dummy = advance_token(state);
    tok_param = advance_token(state);
    ok = append_list(params, get_token_value(tok_param));
    loop = 1;
    while ((loop == 1)) {
    tok_and = peek_token(state);
    if ((get_token_type(tok_and) == 11)) {
    dummy = advance_token(state);
    tok_param = advance_token(state);
    ok = append_list(params, get_token_value(tok_param));
    } else {
    loop = 0;
    }
    }
    }
    ok = expect_token_type(state, 22);
    tok_nl = peek_token(state);
    if ((get_token_type(tok_nl) == 28)) {
    dummy = advance_token(state);
    }
    body = (parse_block(state) + 0);
    ret_val = make_node_func(name, params, body);
    goto L_cleanup;
L_cleanup:
    return ret_val;
}

long long parse_block(long long state) {
    long long ok = 0;
    long long statements = 0;
    long long loop = 0;
    long long tok = 0;
    long long t = 0;
    long long dummy = 0;
    long long stmt = 0;
    long long tok_dedent = 0;
    long long ret_val = 0;

    ok = expect_token_type(state, 29);
    statements = (create_list() + 0);
    loop = 1;
    while ((loop == 1)) {
    tok = peek_token(state);
    t = get_token_type(tok);
    if (((t == 30) || (t == 31))) {
    loop = 0;
    } else {
    if ((t == 28)) {
    dummy = advance_token(state);
    } else {
    stmt = (parse_statement(state) + 0);
    ok = append_list(statements, stmt);
    }
    }
    }
    tok_dedent = peek_token(state);
    if ((get_token_type(tok_dedent) == 30)) {
    dummy = advance_token(state);
    } else {
    printf("%s\n", (char*)(long long)"Parser Error: Expected DEDENT, found:");
    printf("%lld\n", get_token_type(tok_dedent));
    }
    ret_val = statements;
    goto L_cleanup;
L_cleanup:
    return ret_val;
}

long long parse_statement(long long state) {
    long long tok = 0;
    long long t = 0;
    long long tok_var = 0;
    long long var_name = 0;
    long long ok = 0;
    long long expr = 0;
    long long next_t = 0;
    long long dummy = 0;
    long long cond = 0;
    long long then_branch = 0;
    long long else_branch = 0;
    long long next_t2 = 0;
    long long next_t3 = 0;
    long long body = 0;
    long long ret_val = 0;

    tok = advance_token(state);
    t = get_token_type(tok);
    if ((t == 2)) {
    tok_var = advance_token(state);
    var_name = get_token_value(tok_var);
    ok = expect_token_type(state, 3);
    expr = (parse_expr(state, 0) + 0);
    next_t = peek_token(state);
    if ((get_token_type(next_t) == 28)) {
    dummy = advance_token(state);
    }
    ret_val = make_node_set(var_name, expr);
    goto L_cleanup;
    } else {
    if ((t == 4)) {
    cond = (parse_expr(state, 0) + 0);
    ok = expect_token_type(state, 22);
    next_t = peek_token(state);
    if ((get_token_type(next_t) == 28)) {
    dummy = advance_token(state);
    }
    then_branch = (parse_block(state) + 0);
    else_branch = 0;
    next_t2 = peek_token(state);
    if ((get_token_type(next_t2) == 5)) {
    dummy = advance_token(state);
    ok = expect_token_type(state, 22);
    next_t3 = peek_token(state);
    if ((get_token_type(next_t3) == 28)) {
    dummy = advance_token(state);
    }
    else_branch = (parse_block(state) + 0);
    }
    ret_val = make_node_if(cond, then_branch, else_branch);
    goto L_cleanup;
    } else {
    if (((t == 8) || (t == 9))) {
    if ((t == 8)) {
    ok = expect_token_type(state, 9);
    }
    cond = (parse_expr(state, 0) + 0);
    ok = expect_token_type(state, 22);
    next_t = peek_token(state);
    if ((get_token_type(next_t) == 28)) {
    dummy = advance_token(state);
    }
    body = (parse_block(state) + 0);
    ret_val = make_node_repeat_while(cond, body);
    goto L_cleanup;
    } else {
    if ((t == 6)) {
    expr = (parse_expr(state, 0) + 0);
    next_t = peek_token(state);
    if ((get_token_type(next_t) == 28)) {
    dummy = advance_token(state);
    }
    ret_val = make_node_return(expr);
    goto L_cleanup;
    } else {
    if ((t == 7)) {
    expr = (parse_expr(state, 0) + 0);
    next_t = peek_token(state);
    if ((get_token_type(next_t) == 28)) {
    dummy = advance_token(state);
    }
    ret_val = make_node_display(expr);
    goto L_cleanup;
    } else {
    printf("%s\n", (char*)(long long)"Parser Error: Unexpected statement start token type:");
    printf("%lld\n", t);
    ret_val = 0;
    goto L_cleanup;
    }
    }
    }
    }
    }
L_cleanup:
    return ret_val;
}

long long parse_expr(long long state, long long precedence) {
    long long tok = 0;
    long long t = 0;
    long long left = 0;
    long long val = 0;
    long long name = 0;
    long long next_tok = 0;
    long long dummy = 0;
    long long args = 0;
    long long next_tok2 = 0;
    long long arg = 0;
    long long ok = 0;
    long long loop_args = 0;
    long long next_tok3 = 0;
    long long climbing = 0;
    long long next_prec = 0;
    long long op_tok = 0;
    long long op_type = 0;
    long long is_math = 0;
    long long op = 0;
    long long right_prec = 0;
    long long right = 0;
    long long is_comp = 0;
    long long is_logical = 0;
    long long ret_val = 0;

    tok = advance_token(state);
    t = get_token_type(tok);
    left = 0;
    if ((t == 25)) {
    val = parse_int(get_token_value(tok));
    left = (make_node_int(val) + 0);
    } else {
    if ((t == 26)) {
    left = (make_node_str(get_token_value(tok)) + 0);
    } else {
    if ((t == 27)) {
    name = get_token_value(tok);
    next_tok = peek_token(state);
    if ((get_token_type(next_tok) == 23)) {
    dummy = advance_token(state);
    args = (create_list() + 0);
    next_tok2 = peek_token(state);
    if ((get_token_type(next_tok2) != 24)) {
    arg = (parse_expr(state, 0) + 0);
    ok = append_list(args, arg);
    loop_args = 1;
    while ((loop_args == 1)) {
    next_tok3 = peek_token(state);
    if ((get_token_type(next_tok3) == 11)) {
    dummy = advance_token(state);
    arg = (parse_expr(state, 0) + 0);
    ok = append_list(args, arg);
    } else {
    loop_args = 0;
    }
    }
    }
    ok = expect_token_type(state, 24);
    left = (make_node_call(name, args) + 0);
    } else {
    left = (make_node_ident(name) + 0);
    }
    } else {
    if ((t == 23)) {
    left = (parse_expr(state, 0) + 0);
    ok = expect_token_type(state, 24);
    } else {
    printf("%s\n", (char*)(long long)"Parser Error: Expected expression, found token type:");
    printf("%lld\n", t);
    }
    }
    }
    }
    climbing = 1;
    while ((climbing == 1)) {
    next_tok = peek_token(state);
    next_prec = get_token_precedence(next_tok);
    if ((precedence < next_prec)) {
    op_tok = advance_token(state);
    op_type = get_token_type(op_tok);
    is_math = ((((op_type == 12) || (op_type == 13)) || (op_type == 14)) || (op_type == 15));
    if ((is_math == 1)) {
    op = 0;
    if ((op_type == 12)) {
    op = 1;
    }
    if ((op_type == 13)) {
    op = 2;
    }
    if ((op_type == 14)) {
    op = 3;
    }
    if ((op_type == 15)) {
    op = 4;
    }
    right_prec = get_token_precedence(op_tok);
    right = (parse_expr(state, right_prec) + 0);
    left = (make_node_binary(left, op, right) + 0);
    } else {
    is_comp = ((((op_type == 16) || (op_type == 17)) || (op_type == 18)) || (op_type == 19));
    if ((is_comp == 1)) {
    op = 0;
    if ((op_type == 16)) {
    op = 1;
    }
    if ((op_type == 17)) {
    op = 2;
    }
    if ((op_type == 18)) {
    op = 3;
    }
    if ((op_type == 19)) {
    op = 4;
    }
    right_prec = get_token_precedence(op_tok);
    right = (parse_expr(state, right_prec) + 0);
    left = (make_node_comp(left, op, right) + 0);
    } else {
    is_logical = ((op_type == 20) || (op_type == 21));
    if ((is_logical == 1)) {
    op = 0;
    if ((op_type == 20)) {
    op = 1;
    }
    if ((op_type == 21)) {
    op = 2;
    }
    right_prec = get_token_precedence(op_tok);
    right = (parse_expr(state, right_prec) + 0);
    left = (make_node_logical(left, op, right) + 0);
    }
    }
    }
    } else {
    climbing = 0;
    }
    }
    ret_val = left;
    goto L_cleanup;
L_cleanup:
    return ret_val;
}

long long map_get(long long keys, long long values, long long key) {
    long long key_str = 0;
    long long len = 0;
    long long idx = 0;
    long long k = 0;
    long long ret_val = 0;

    key_str = string_concat(key, (long long)"");
    len = length_list(keys);
    idx = 0;
    while ((idx < len)) {
    k = get_list(keys, idx);
    if ((strcmp((char*)key_str, (char*)k) == 0)) {
    ret_val = get_list(values, idx);
    goto L_cleanup;
    }
    idx = (idx + 1);
    }
    ret_val = 0;
    goto L_cleanup;
L_cleanup:
    return ret_val;
}

long long map_put(long long keys, long long values, long long key, long long val) {
    long long key_str = 0;
    long long len = 0;
    long long idx = 0;
    long long found = 0;
    long long k = 0;
    long long ok = 0;
    long long ret_val = 0;

    key_str = string_concat(key, (long long)"");
    len = length_list(keys);
    idx = 0;
    found = 0;
    while (((idx < len) && (found == 0))) {
    k = get_list(keys, idx);
    if ((strcmp((char*)key_str, (char*)k) == 0)) {
    ok = set_list(values, idx, val);
    found = 1;
    }
    idx = (idx + 1);
    }
    if ((found == 0)) {
    ok = append_list(keys, key);
    ok = append_list(values, val);
    }
    ret_val = 0;
    goto L_cleanup;
L_cleanup:
    return ret_val;
}

long long string_concat(long long s1, long long s2) {
    long long lst = 0;
    long long len1 = 0;
    long long idx = 0;
    long long ok = 0;
    long long len2 = 0;
    long long idx2 = 0;
    long long res = 0;
    long long ret_val = 0;

    {
        long long tmp_val = create_list();
        free_list(lst);
        lst = tmp_val;
    }
    len1 = string_length((char*)s1);
    idx = 0;
    while ((idx < len1)) {
    ok = append_list(lst, get_character((char*)s1, idx));
    idx = (idx + 1);
    }
    len2 = string_length((char*)s2);
    idx2 = 0;
    while ((idx2 < len2)) {
    ok = append_list(lst, get_character((char*)s2, idx2));
    idx2 = (idx2 + 1);
    }
    res = (long long)string_from_list(lst);
    ret_val = res;
    goto L_cleanup;
L_cleanup:
    free_list(lst);
    return ret_val;
}

long long int_to_string(long long n) {
    long long lst = 0;
    long long temp = 0;
    long long digits = 0;
    long long digit = 0;
    long long ok = 0;
    long long len = 0;
    long long d = 0;
    long long res = 0;
    long long ret_val = 0;

    if ((n == 0)) {
    ret_val = (long long)"0";
    goto L_cleanup;
    }
    {
        long long tmp_val = create_list();
        free_list(lst);
        lst = tmp_val;
    }
    temp = n;
    {
        long long tmp_val = create_list();
        free_list(digits);
        digits = tmp_val;
    }
    while ((temp > 0)) {
    digit = (temp - ((temp / 10) * 10));
    ok = append_list(digits, (digit + 48));
    temp = (temp / 10);
    }
    len = length_list(digits);
    while ((len > 0)) {
    d = pop_list(digits);
    ok = append_list(lst, d);
    len = (len - 1);
    }
    res = (long long)string_from_list(lst);
    ret_val = res;
    goto L_cleanup;
L_cleanup:
    free_list(lst);
    free_list(digits);
    return ret_val;
}

long long escape_string(long long s) {
    long long lst = 0;
    long long len = 0;
    long long idx = 0;
    long long ch = 0;
    long long ok = 0;
    long long res = 0;
    long long ret_val = 0;

    {
        long long tmp_val = create_list();
        free_list(lst);
        lst = tmp_val;
    }
    len = string_length((char*)s);
    idx = 0;
    while ((idx < len)) {
    ch = get_character((char*)s, idx);
    if ((ch == 92)) {
    ok = append_list(lst, 92);
    ok = append_list(lst, 92);
    } else {
    if ((ch == 34)) {
    ok = append_list(lst, 92);
    ok = append_list(lst, 34);
    } else {
    if ((ch == 10)) {
    ok = append_list(lst, 92);
    ok = append_list(lst, 110);
    } else {
    if ((ch == 9)) {
    ok = append_list(lst, 92);
    ok = append_list(lst, 116);
    } else {
    if ((ch == 13)) {
    ok = append_list(lst, 92);
    ok = append_list(lst, 114);
    } else {
    ok = append_list(lst, ch);
    }
    }
    }
    }
    }
    idx = (idx + 1);
    }
    res = (long long)string_from_list(lst);
    ret_val = res;
    goto L_cleanup;
L_cleanup:
    free_list(lst);
    return ret_val;
}

long long join_strings(long long lines) {
    long long lst = 0;
    long long len = 0;
    long long idx = 0;
    long long line = 0;
    long long line_len = 0;
    long long c_idx = 0;
    long long ok = 0;
    long long res = 0;
    long long ret_val = 0;

    {
        long long tmp_val = create_list();
        free_list(lst);
        lst = tmp_val;
    }
    len = length_list(lines);
    idx = 0;
    while ((idx < len)) {
    line = get_list(lines, idx);
    line_len = string_length((char*)line);
    c_idx = 0;
    while ((c_idx < line_len)) {
    ok = append_list(lst, get_character((char*)line, c_idx));
    c_idx = (c_idx + 1);
    }
    idx = (idx + 1);
    }
    res = (long long)string_from_list(lst);
    ret_val = res;
    goto L_cleanup;
L_cleanup:
    free_list(lst);
    return ret_val;
}

long long create_codegen_state() {
    long long state = 0;
    long long ok = 0;
    long long ret_val = 0;

    {
        long long tmp_val = create_list();
        free_list(state);
        state = tmp_val;
    }
    ok = append_list(state, (create_list() + 0));
    ok = append_list(state, (create_list() + 0));
    ok = append_list(state, 0);
    ok = append_list(state, (create_list() + 0));
    ok = append_list(state, (create_list() + 0));
    ok = append_list(state, 0);
    ret_val = state;
    state = 0;
    goto L_cleanup;
L_cleanup:
    free_list(state);
    return ret_val;
}

long long emit(long long state, long long line) {
    long long lines = 0;
    long long ok = 0;
    long long ret_val = 0;

    lines = get_list(state, 0);
    ok = append_list(lines, line);
    ret_val = 0;
    goto L_cleanup;
L_cleanup:
    return ret_val;
}

long long add_string_literal(long long state, long long s) {
    long long s_str = 0;
    long long lits = 0;
    long long len = 0;
    long long idx = 0;
    long long val = 0;
    long long ok = 0;
    long long ret_val = 0;

    s_str = string_concat(s, (long long)"");
    lits = get_list(state, 1);
    len = length_list(lits);
    idx = 0;
    while ((idx < len)) {
    val = get_list(lits, idx);
    if ((strcmp((char*)s_str, (char*)val) == 0)) {
    ret_val = idx;
    goto L_cleanup;
    }
    idx = (idx + 1);
    }
    ok = append_list(lits, s);
    ret_val = len;
    goto L_cleanup;
L_cleanup:
    return ret_val;
}

long long get_new_label(long long state, long long prefix) {
    long long count = 0;
    long long next_count = 0;
    long long ok = 0;
    long long num_str = 0;
    long long label_half = 0;
    long long label = 0;
    long long final_label = 0;
    long long ret_val = 0;

    count = get_list(state, 2);
    next_count = (count + 1);
    ok = set_list(state, 2, next_count);
    num_str = int_to_string(count);
    label_half = string_concat((long long)"L_", prefix);
    label = string_concat(label_half, (long long)"_");
    final_label = string_concat(label, num_str);
    ret_val = final_label;
    goto L_cleanup;
L_cleanup:
    return ret_val;
}

long long analyze_return_types(long long state, long long program) {
    long long keys = 0;
    long long values = 0;
    long long ok = 0;
    long long funcs = 0;
    long long funcs_len = 0;
    long long pass = 0;
    long long idx = 0;
    long long func = 0;
    long long name = 0;
    long long params = 0;
    long long body = 0;
    long long var_keys = 0;
    long long var_values = 0;
    long long p_len = 0;
    long long p_idx = 0;
    long long p_name = 0;
    long long ret_t = 0;
    long long ret_val = 0;

    keys = get_list(state, 3);
    values = get_list(state, 4);
    ok = map_put(keys, values, (long long)"read_file_content", 3);
    ok = map_put(keys, values, (long long)"create_list", 4);
    ok = map_put(keys, values, (long long)"ep_md5", 3);
    ok = map_put(keys, values, (long long)"ep_sha256", 3);
    ok = map_put(keys, values, (long long)"ep_net_connect", 1);
    ok = map_put(keys, values, (long long)"ep_net_listen", 1);
    ok = map_put(keys, values, (long long)"ep_net_accept", 1);
    ok = map_put(keys, values, (long long)"ep_net_send", 1);
    ok = map_put(keys, values, (long long)"ep_net_recv", 3);
    ok = map_put(keys, values, (long long)"ep_net_close", 1);
    ok = map_put(keys, values, (long long)"append_list", 1);
    ok = map_put(keys, values, (long long)"get_list", 1);
    ok = map_put(keys, values, (long long)"set_list", 1);
    ok = map_put(keys, values, (long long)"length_list", 1);
    ok = map_put(keys, values, (long long)"string_length", 1);
    ok = map_put(keys, values, (long long)"get_character", 1);
    ok = map_put(keys, values, (long long)"display_string", 1);
    ok = map_put(keys, values, (long long)"get_argument_count", 1);
    ok = map_put(keys, values, (long long)"get_argument", 2);
    ok = map_put(keys, values, (long long)"write_file_content", 1);
    ok = map_put(keys, values, (long long)"run_command", 1);
    ok = map_put(keys, values, (long long)"substring", 3);
    ok = map_put(keys, values, (long long)"string_from_list", 3);
    ok = map_put(keys, values, (long long)"pop_list", 1);
    funcs = get_list(program, 2);
    funcs_len = length_list(funcs);
    pass = 0;
    while ((pass < 3)) {
    idx = 0;
    while ((idx < funcs_len)) {
    func = get_list(funcs, idx);
    name = get_list(func, 1);
    params = get_list(func, 2);
    body = get_list(func, 3);
    var_keys = (create_list() + 0);
    var_values = (create_list() + 0);
    p_len = length_list(params);
    p_idx = 0;
    while ((p_idx < p_len)) {
    p_name = get_list(params, p_idx);
    ok = map_put(var_keys, var_values, p_name, 1);
    p_idx = (p_idx + 1);
    }
    ok = collect_var_types(state, body, var_keys, var_values);
    ret_t = determine_ret_type(state, body, var_keys, var_values);
    if ((ret_t == 0)) {
    ret_t = 1;
    }
    ok = map_put(keys, values, name, ret_t);
    idx = (idx + 1);
    }
    pass = (pass + 1);
    }
    ret_val = 0;
    goto L_cleanup;
L_cleanup:
    return ret_val;
}

long long collect_var_types(long long state, long long stmts, long long var_keys, long long var_values) {
    long long len = 0;
    long long idx = 0;
    long long stmt = 0;
    long long type = 0;
    long long var_name = 0;
    long long expr = 0;
    long long t = 0;
    long long ok = 0;
    long long then_b = 0;
    long long else_b = 0;
    long long body = 0;
    long long ret_val = 0;

    len = length_list(stmts);
    idx = 0;
    while ((idx < len)) {
    stmt = get_list(stmts, idx);
    type = get_list(stmt, 0);
    if ((type == 7)) {
    var_name = get_list(stmt, 1);
    expr = get_list(stmt, 2);
    t = infer_type(state, expr, var_keys, var_values);
    ok = map_put(var_keys, var_values, var_name, t);
    } else {
    if ((type == 10)) {
    then_b = get_list(stmt, 2);
    ok = collect_var_types(state, then_b, var_keys, var_values);
    else_b = get_list(stmt, 3);
    if ((else_b != 0)) {
    ok = collect_var_types(state, else_b, var_keys, var_values);
    }
    } else {
    if ((type == 11)) {
    body = get_list(stmt, 2);
    ok = collect_var_types(state, body, var_keys, var_values);
    }
    }
    }
    idx = (idx + 1);
    }
    ret_val = 0;
    goto L_cleanup;
L_cleanup:
    return ret_val;
}

long long determine_ret_type(long long state, long long stmts, long long var_keys, long long var_values) {
    long long len = 0;
    long long idx = 0;
    long long stmt = 0;
    long long type = 0;
    long long expr = 0;
    long long then_b = 0;
    long long ret_t = 0;
    long long else_b = 0;
    long long ret_t2 = 0;
    long long body = 0;
    long long ret_val = 0;

    len = length_list(stmts);
    idx = 0;
    while ((idx < len)) {
    stmt = get_list(stmts, idx);
    type = get_list(stmt, 0);
    if ((type == 8)) {
    expr = get_list(stmt, 1);
    ret_val = infer_type(state, expr, var_keys, var_values);
    goto L_cleanup;
    } else {
    if ((type == 10)) {
    then_b = get_list(stmt, 2);
    ret_t = determine_ret_type(state, then_b, var_keys, var_values);
    if ((ret_t != 0)) {
    ret_val = ret_t;
    goto L_cleanup;
    }
    else_b = get_list(stmt, 3);
    if ((else_b != 0)) {
    ret_t2 = determine_ret_type(state, else_b, var_keys, var_values);
    if ((ret_t2 != 0)) {
    ret_val = ret_t2;
    goto L_cleanup;
    }
    }
    } else {
    if ((type == 11)) {
    body = get_list(stmt, 2);
    ret_t = determine_ret_type(state, body, var_keys, var_values);
    if ((ret_t != 0)) {
    ret_val = ret_t;
    goto L_cleanup;
    }
    }
    }
    }
    idx = (idx + 1);
    }
    ret_val = 0;
    goto L_cleanup;
L_cleanup:
    return ret_val;
}

long long infer_type(long long state, long long expr, long long var_keys, long long var_values) {
    long long type = 0;
    long long name = 0;
    long long t = 0;
    long long func_keys = 0;
    long long func_values = 0;
    long long ret_val = 0;

    type = get_list(expr, 0);
    if ((type == 1)) {
    ret_val = 1;
    goto L_cleanup;
    }
    if ((type == 2)) {
    ret_val = 2;
    goto L_cleanup;
    }
    if ((type == 3)) {
    name = get_list(expr, 1);
    t = map_get(var_keys, var_values, name);
    if ((t != 0)) {
    ret_val = t;
    goto L_cleanup;
    }
    ret_val = 1;
    goto L_cleanup;
    }
    if ((((type == 4) || (type == 5)) || (type == 14))) {
    ret_val = 1;
    goto L_cleanup;
    }
    if ((type == 6)) {
    name = get_list(expr, 1);
    func_keys = get_list(state, 3);
    func_values = get_list(state, 4);
    t = map_get(func_keys, func_values, name);
    if ((t != 0)) {
    ret_val = t;
    goto L_cleanup;
    }
    ret_val = 1;
    goto L_cleanup;
    }
    ret_val = 1;
    goto L_cleanup;
L_cleanup:
    return ret_val;
}

long long gen_function(long long state, long long func) {
    long long name = 0;
    long long params = 0;
    long long body = 0;
    long long func_keys = 0;
    long long func_values = 0;
    long long ret_t = 0;
    long long ok = 0;
    long long var_types_keys = 0;
    long long var_types_values = 0;
    long long p_len = 0;
    long long p_idx = 0;
    long long p_name = 0;
    long long c_name = 0;
    long long header = 0;
    long long num_vars = 0;
    long long idx = 0;
    long long var_name = 0;
    long long is_param = 0;
    long long p_i = 0;
    long long decl = 0;
    long long body_len = 0;
    long long stmt = 0;
    long long t = 0;
    long long cleanup_line = 0;
    long long ret_val = 0;

    name = get_list(func, 1);
    params = get_list(func, 2);
    body = get_list(func, 3);
    func_keys = get_list(state, 3);
    func_values = get_list(state, 4);
    ret_t = map_get(func_keys, func_values, name);
    ok = set_list(state, 5, ret_t);
    var_types_keys = (create_list() + 0);
    var_types_values = (create_list() + 0);
    p_len = length_list(params);
    p_idx = 0;
    while ((p_idx < p_len)) {
    p_name = get_list(params, p_idx);
    ok = map_put(var_types_keys, var_types_values, p_name, 1);
    p_idx = (p_idx + 1);
    }
    ok = collect_var_types(state, body, var_types_keys, var_types_values);
    c_name = name;
    if ((strcmp((char*)(long long)"main", (char*)name) == 0)) {
    c_name = (long long)"_main";
    }
    header = (long long)"long long ";
    header = string_concat(header, c_name);
    header = string_concat(header, (long long)"(");
    p_idx = 0;
    while ((p_idx < p_len)) {
    p_name = get_list(params, p_idx);
    header = string_concat(header, (long long)"long long ");
    header = string_concat(header, p_name);
    if ((p_idx < (p_len - 1))) {
    header = string_concat(header, (long long)", ");
    }
    p_idx = (p_idx + 1);
    }
    header = string_concat(header, (long long)") {\n");
    ok = emit(state, header);
    num_vars = length_list(var_types_keys);
    idx = 0;
    while ((idx < num_vars)) {
    var_name = get_list(var_types_keys, idx);
    is_param = 0;
    p_i = 0;
    while ((p_i < p_len)) {
    p_name = get_list(params, p_i);
    if ((strcmp((char*)string_concat(var_name, (long long)""), (char*)string_concat(p_name, (long long)"")) == 0)) {
    is_param = 1;
    }
    p_i = (p_i + 1);
    }
    if ((is_param == 0)) {
    decl = (long long)"    long long ";
    decl = string_concat(decl, var_name);
    decl = string_concat(decl, (long long)" = 0;\n");
    ok = emit(state, decl);
    }
    idx = (idx + 1);
    }
    ok = emit(state, (long long)"    long long ret_val = 0;\n\n");
    body_len = length_list(body);
    idx = 0;
    while ((idx < body_len)) {
    stmt = get_list(body, idx);
    ok = gen_statement(state, stmt, var_types_keys, var_types_values);
    idx = (idx + 1);
    }
    ok = emit(state, (long long)"L_cleanup:\n");
    idx = 0;
    while ((idx < num_vars)) {
    var_name = get_list(var_types_keys, idx);
    is_param = 0;
    p_i = 0;
    while ((p_i < p_len)) {
    p_name = get_list(params, p_i);
    if ((strcmp((char*)string_concat(var_name, (long long)""), (char*)string_concat(p_name, (long long)"")) == 0)) {
    is_param = 1;
    }
    p_i = (p_i + 1);
    }
    if ((is_param == 0)) {
    t = map_get(var_types_keys, var_types_values, var_name);
    if ((t == 4)) {
    cleanup_line = (long long)"    free_list(";
    cleanup_line = string_concat(cleanup_line, var_name);
    cleanup_line = string_concat(cleanup_line, (long long)");\n");
    ok = emit(state, cleanup_line);
    }
    }
    idx = (idx + 1);
    }
    ok = emit(state, (long long)"    return ret_val;\n}\n\n");
    ret_val = 0;
    goto L_cleanup;
L_cleanup:
    return ret_val;
}

long long gen_statement(long long state, long long stmt, long long var_keys, long long var_values) {
    long long type = 0;
    long long name = 0;
    long long expr = 0;
    long long t = 0;
    long long expr_str = 0;
    long long ok = 0;
    long long line = 0;
    long long expr_type = 0;
    long long null_line = 0;
    long long cond = 0;
    long long then_b = 0;
    long long else_b = 0;
    long long cond_str = 0;
    long long t_len = 0;
    long long t_idx = 0;
    long long s = 0;
    long long e_len = 0;
    long long e_idx = 0;
    long long body = 0;
    long long b_len = 0;
    long long b_idx = 0;
    long long ret_val = 0;

    type = get_list(stmt, 0);
    if ((type == 7)) {
    name = get_list(stmt, 1);
    expr = get_list(stmt, 2);
    t = map_get(var_keys, var_values, name);
    expr_str = gen_expr(state, expr, var_keys, var_values);
    if ((t == 4)) {
    ok = emit(state, (long long)"    {\n");
    line = (long long)"        long long tmp_val = ";
    line = string_concat(line, expr_str);
    line = string_concat(line, (long long)";\n");
    ok = emit(state, line);
    line = (long long)"        free_list(";
    line = string_concat(line, name);
    line = string_concat(line, (long long)");\n");
    ok = emit(state, line);
    line = (long long)"        ";
    line = string_concat(line, name);
    line = string_concat(line, (long long)" = tmp_val;\n");
    ok = emit(state, line);
    ok = emit(state, (long long)"    }\n");
    } else {
    line = (long long)"    ";
    line = string_concat(line, name);
    line = string_concat(line, (long long)" = ");
    line = string_concat(line, expr_str);
    line = string_concat(line, (long long)";\n");
    ok = emit(state, line);
    }
    ret_val = 0;
    goto L_cleanup;
    }
    if ((type == 8)) {
    expr = get_list(stmt, 1);
    expr_str = gen_expr(state, expr, var_keys, var_values);
    line = (long long)"    ret_val = ";
    line = string_concat(line, expr_str);
    line = string_concat(line, (long long)";\n");
    ok = emit(state, line);
    expr_type = get_list(expr, 0);
    if ((expr_type == 3)) {
    name = get_list(expr, 1);
    t = map_get(var_keys, var_values, name);
    if ((t == 4)) {
    null_line = (long long)"    ";
    null_line = string_concat(null_line, name);
    null_line = string_concat(null_line, (long long)" = 0;\n");
    ok = emit(state, null_line);
    }
    }
    ok = emit(state, (long long)"    goto L_cleanup;\n");
    ret_val = 0;
    goto L_cleanup;
    }
    if ((type == 9)) {
    expr = get_list(stmt, 1);
    t = infer_type(state, expr, var_keys, var_values);
    expr_str = gen_expr(state, expr, var_keys, var_values);
    if (((t == 2) || (t == 3))) {
    line = (long long)"    printf(\"%s\\n\", (char*)";
    line = string_concat(line, expr_str);
    line = string_concat(line, (long long)");\n");
    ok = emit(state, line);
    } else {
    line = (long long)"    printf(\"%lld\\n\", ";
    line = string_concat(line, expr_str);
    line = string_concat(line, (long long)");\n");
    ok = emit(state, line);
    }
    ret_val = 0;
    goto L_cleanup;
    }
    if ((type == 10)) {
    cond = get_list(stmt, 1);
    then_b = get_list(stmt, 2);
    else_b = get_list(stmt, 3);
    cond_str = gen_expr(state, cond, var_keys, var_values);
    line = (long long)"    if (";
    line = string_concat(line, cond_str);
    line = string_concat(line, (long long)") {\n");
    ok = emit(state, line);
    t_len = length_list(then_b);
    t_idx = 0;
    while ((t_idx < t_len)) {
    s = get_list(then_b, t_idx);
    ok = gen_statement(state, s, var_keys, var_values);
    t_idx = (t_idx + 1);
    }
    if ((else_b != 0)) {
    ok = emit(state, (long long)"    } else {\n");
    e_len = length_list(else_b);
    e_idx = 0;
    while ((e_idx < e_len)) {
    s = get_list(else_b, e_idx);
    ok = gen_statement(state, s, var_keys, var_values);
    e_idx = (e_idx + 1);
    }
    ok = emit(state, (long long)"    }\n");
    } else {
    ok = emit(state, (long long)"    }\n");
    }
    ret_val = 0;
    goto L_cleanup;
    }
    if ((type == 11)) {
    cond = get_list(stmt, 1);
    body = get_list(stmt, 2);
    cond_str = gen_expr(state, cond, var_keys, var_values);
    line = (long long)"    while (";
    line = string_concat(line, cond_str);
    line = string_concat(line, (long long)") {\n");
    ok = emit(state, line);
    b_len = length_list(body);
    b_idx = 0;
    while ((b_idx < b_len)) {
    s = get_list(body, b_idx);
    ok = gen_statement(state, s, var_keys, var_values);
    b_idx = (b_idx + 1);
    }
    ok = emit(state, (long long)"    }\n");
    ret_val = 0;
    goto L_cleanup;
    }
    ret_val = 0;
    goto L_cleanup;
L_cleanup:
    return ret_val;
}

long long gen_expr(long long state, long long expr, long long var_keys, long long var_values) {
    long long type = 0;
    long long val = 0;
    long long escaped = 0;
    long long res = 0;
    long long name = 0;
    long long left = 0;
    long long op = 0;
    long long right = 0;
    long long left_str = 0;
    long long right_str = 0;
    long long op_str = 0;
    long long lt = 0;
    long long is_string = 0;
    long long cmp_op = 0;
    long long args = 0;
    long long args_len = 0;
    long long formatted_args = 0;
    long long idx = 0;
    long long arg = 0;
    long long arg_val = 0;
    long long casted = 0;
    long long ok = 0;
    long long args_str_list = 0;
    long long arg_item = 0;
    long long args_joined = 0;
    long long call_str = 0;
    long long ret_val = 0;

    type = get_list(expr, 0);
    if ((type == 1)) {
    val = get_list(expr, 1);
    ret_val = int_to_string(val);
    goto L_cleanup;
    }
    if ((type == 2)) {
    val = get_list(expr, 1);
    escaped = escape_string(val);
    res = (long long)"(long long)\"";
    res = string_concat(res, escaped);
    res = string_concat(res, (long long)"\"");
    ret_val = res;
    goto L_cleanup;
    }
    if ((type == 3)) {
    name = get_list(expr, 1);
    ret_val = name;
    goto L_cleanup;
    }
    if ((type == 4)) {
    left = get_list(expr, 1);
    op = get_list(expr, 2);
    right = get_list(expr, 3);
    left_str = gen_expr(state, left, var_keys, var_values);
    right_str = gen_expr(state, right, var_keys, var_values);
    op_str = (long long)"";
    if ((op == 1)) {
    op_str = (long long)"+";
    }
    if ((op == 2)) {
    op_str = (long long)"-";
    }
    if ((op == 3)) {
    op_str = (long long)"*";
    }
    if ((op == 4)) {
    op_str = (long long)"/";
    }
    res = (long long)"(";
    res = string_concat(res, left_str);
    res = string_concat(res, (long long)" ");
    res = string_concat(res, op_str);
    res = string_concat(res, (long long)" ");
    res = string_concat(res, right_str);
    res = string_concat(res, (long long)")");
    ret_val = res;
    goto L_cleanup;
    }
    if ((type == 5)) {
    left = get_list(expr, 1);
    op = get_list(expr, 2);
    right = get_list(expr, 3);
    left_str = gen_expr(state, left, var_keys, var_values);
    right_str = gen_expr(state, right, var_keys, var_values);
    lt = infer_type(state, left, var_keys, var_values);
    is_string = 0;
    if (((lt == 2) || (lt == 3))) {
    is_string = 1;
    }
    if ((is_string == 1)) {
    cmp_op = (long long)"";
    if ((op == 1)) {
    cmp_op = (long long)"< 0";
    }
    if ((op == 2)) {
    cmp_op = (long long)"> 0";
    }
    if ((op == 3)) {
    cmp_op = (long long)"== 0";
    }
    if ((op == 4)) {
    cmp_op = (long long)"!= 0";
    }
    res = (long long)"(strcmp((char*)";
    res = string_concat(res, left_str);
    res = string_concat(res, (long long)", (char*)");
    res = string_concat(res, right_str);
    res = string_concat(res, (long long)") ");
    res = string_concat(res, cmp_op);
    res = string_concat(res, (long long)")");
    ret_val = res;
    goto L_cleanup;
    } else {
    op_str = (long long)"";
    if ((op == 1)) {
    op_str = (long long)"<";
    }
    if ((op == 2)) {
    op_str = (long long)">";
    }
    if ((op == 3)) {
    op_str = (long long)"==";
    }
    if ((op == 4)) {
    op_str = (long long)"!=";
    }
    res = (long long)"(";
    res = string_concat(res, left_str);
    res = string_concat(res, (long long)" ");
    res = string_concat(res, op_str);
    res = string_concat(res, (long long)" ");
    res = string_concat(res, right_str);
    res = string_concat(res, (long long)")");
    ret_val = res;
    goto L_cleanup;
    }
    }
    if ((type == 14)) {
    left = get_list(expr, 1);
    op = get_list(expr, 2);
    right = get_list(expr, 3);
    left_str = gen_expr(state, left, var_keys, var_values);
    right_str = gen_expr(state, right, var_keys, var_values);
    op_str = (long long)"";
    if ((op == 1)) {
    op_str = (long long)"&&";
    }
    if ((op == 2)) {
    op_str = (long long)"||";
    }
    res = (long long)"(";
    res = string_concat(res, left_str);
    res = string_concat(res, (long long)" ");
    res = string_concat(res, op_str);
    res = string_concat(res, (long long)" ");
    res = string_concat(res, right_str);
    res = string_concat(res, (long long)")");
    ret_val = res;
    goto L_cleanup;
    }
    if ((type == 6)) {
    name = get_list(expr, 1);
    args = get_list(expr, 2);
    args_len = length_list(args);
    {
        long long tmp_val = create_list();
        free_list(formatted_args);
        formatted_args = tmp_val;
    }
    idx = 0;
    while ((idx < args_len)) {
    arg = get_list(args, idx);
    arg_val = gen_expr(state, arg, var_keys, var_values);
    casted = arg_val;
    if ((((((((strcmp((char*)(long long)"read_file_content", (char*)name) == 0) || (strcmp((char*)(long long)"string_length", (char*)name) == 0)) || (strcmp((char*)(long long)"display_string", (char*)name) == 0)) || (strcmp((char*)(long long)"run_command", (char*)name) == 0)) || (strcmp((char*)(long long)"ep_md5", (char*)name) == 0)) || (strcmp((char*)(long long)"ep_sha256", (char*)name) == 0)) || (strcmp((char*)(long long)"ep_net_connect", (char*)name) == 0))) {
    if ((idx == 0)) {
    casted = string_concat((long long)"(char*)", arg_val);
    }
    } else {
    if (((strcmp((char*)(long long)"get_character", (char*)name) == 0) || (strcmp((char*)(long long)"substring", (char*)name) == 0))) {
    if ((idx == 0)) {
    casted = string_concat((long long)"(char*)", arg_val);
    }
    } else {
    if ((strcmp((char*)(long long)"write_file_content", (char*)name) == 0)) {
    if (((idx == 0) || (idx == 1))) {
    casted = string_concat((long long)"(char*)", arg_val);
    }
    } else {
    if ((strcmp((char*)(long long)"ep_net_send", (char*)name) == 0)) {
    if ((idx == 1)) {
    casted = string_concat((long long)"(char*)", arg_val);
    }
    }
    }
    }
    }
    ok = append_list(formatted_args, casted);
    idx = (idx + 1);
    }
    {
        long long tmp_val = create_list();
        free_list(args_str_list);
        args_str_list = tmp_val;
    }
    idx = 0;
    while ((idx < args_len)) {
    arg_item = get_list(formatted_args, idx);
    ok = append_list(args_str_list, arg_item);
    if ((idx < (args_len - 1))) {
    ok = append_list(args_str_list, (long long)", ");
    }
    idx = (idx + 1);
    }
    args_joined = join_strings(args_str_list);
    call_str = name;
    call_str = string_concat(call_str, (long long)"(");
    call_str = string_concat(call_str, args_joined);
    call_str = string_concat(call_str, (long long)")");
    if ((((((((strcmp((char*)(long long)"read_file_content", (char*)name) == 0) || (strcmp((char*)(long long)"get_argument", (char*)name) == 0)) || (strcmp((char*)(long long)"substring", (char*)name) == 0)) || (strcmp((char*)(long long)"string_from_list", (char*)name) == 0)) || (strcmp((char*)(long long)"ep_net_recv", (char*)name) == 0)) || (strcmp((char*)(long long)"ep_md5", (char*)name) == 0)) || (strcmp((char*)(long long)"ep_sha256", (char*)name) == 0))) {
    res = (long long)"(long long)";
    res = string_concat(res, call_str);
    ret_val = res;
    goto L_cleanup;
    } else {
    ret_val = call_str;
    goto L_cleanup;
    }
    }
    ret_val = (long long)"";
    goto L_cleanup;
L_cleanup:
    free_list(formatted_args);
    free_list(args_str_list);
    return ret_val;
}

long long get_c_runtime_source() {
    long long lines = 0;
    long long ok = 0;
    long long ret_val = 0;

    {
        long long tmp_val = create_list();
        free_list(lines);
        lines = tmp_val;
    }
    ok = append_list(lines, (long long)"#include <stdio.h>\n");
    ok = append_list(lines, (long long)"#include <stdlib.h>\n");
    ok = append_list(lines, (long long)"#include <string.h>\n");
    ok = append_list(lines, (long long)"#include <sys/socket.h>\n");
    ok = append_list(lines, (long long)"#include <netinet/in.h>\n");
    ok = append_list(lines, (long long)"#include <arpa/inet.h>\n");
    ok = append_list(lines, (long long)"#include <unistd.h>\n");
    ok = append_list(lines, (long long)"#include <netdb.h>\n\n");
    ok = append_list(lines, (long long)"long long ep_net_connect(const char* host, long long port) {\n");
    ok = append_list(lines, (long long)"    int sockfd = socket(AF_INET, SOCK_STREAM, 0);\n");
    ok = append_list(lines, (long long)"    if (sockfd < 0) return -1;\n");
    ok = append_list(lines, (long long)"    struct hostent* server = gethostbyname(host);\n");
    ok = append_list(lines, (long long)"    if (!server) {\n");
    ok = append_list(lines, (long long)"        close(sockfd);\n");
    ok = append_list(lines, (long long)"        return -1;\n");
    ok = append_list(lines, (long long)"    }\n");
    ok = append_list(lines, (long long)"    struct sockaddr_in serv_addr;\n");
    ok = append_list(lines, (long long)"    memset(&serv_addr, 0, sizeof(serv_addr));\n");
    ok = append_list(lines, (long long)"    serv_addr.sin_family = AF_INET;\n");
    ok = append_list(lines, (long long)"    memcpy(&serv_addr.sin_addr.s_addr, server->h_addr_list[0], server->h_length);\n");
    ok = append_list(lines, (long long)"    serv_addr.sin_port = htons(port);\n");
    ok = append_list(lines, (long long)"    if (connect(sockfd, (struct sockaddr*)&serv_addr, sizeof(serv_addr)) < 0) {\n");
    ok = append_list(lines, (long long)"        close(sockfd);\n");
    ok = append_list(lines, (long long)"        return -1;\n");
    ok = append_list(lines, (long long)"    }\n");
    ok = append_list(lines, (long long)"    return sockfd;\n");
    ok = append_list(lines, (long long)"}\n\n");
    ok = append_list(lines, (long long)"long long ep_net_listen(long long port) {\n");
    ok = append_list(lines, (long long)"    int sockfd = socket(AF_INET, SOCK_STREAM, 0);\n");
    ok = append_list(lines, (long long)"    if (sockfd < 0) return -1;\n");
    ok = append_list(lines, (long long)"    int opt = 1;\n");
    ok = append_list(lines, (long long)"    setsockopt(sockfd, SOL_SOCKET, SO_REUSEADDR, &opt, sizeof(opt));\n");
    ok = append_list(lines, (long long)"    struct sockaddr_in serv_addr;\n");
    ok = append_list(lines, (long long)"    memset(&serv_addr, 0, sizeof(serv_addr));\n");
    ok = append_list(lines, (long long)"    serv_addr.sin_family = AF_INET;\n");
    ok = append_list(lines, (long long)"    serv_addr.sin_addr.s_addr = INADDR_ANY;\n");
    ok = append_list(lines, (long long)"    serv_addr.sin_port = htons(port);\n");
    ok = append_list(lines, (long long)"    if (bind(sockfd, (struct sockaddr*)&serv_addr, sizeof(serv_addr)) < 0) {\n");
    ok = append_list(lines, (long long)"        close(sockfd);\n");
    ok = append_list(lines, (long long)"        return -1;\n");
    ok = append_list(lines, (long long)"    }\n");
    ok = append_list(lines, (long long)"    if (listen(sockfd, 10) < 0) {\n");
    ok = append_list(lines, (long long)"        close(sockfd);\n");
    ok = append_list(lines, (long long)"        return -1;\n");
    ok = append_list(lines, (long long)"    }\n");
    ok = append_list(lines, (long long)"    return sockfd;\n");
    ok = append_list(lines, (long long)"}\n\n");
    ok = append_list(lines, (long long)"long long ep_net_accept(long long server_fd) {\n");
    ok = append_list(lines, (long long)"    struct sockaddr_in cli_addr;\n");
    ok = append_list(lines, (long long)"    socklen_t clilen = sizeof(cli_addr);\n");
    ok = append_list(lines, (long long)"    int newsockfd = accept((int)server_fd, (struct sockaddr*)&cli_addr, &clilen);\n");
    ok = append_list(lines, (long long)"    return newsockfd;\n");
    ok = append_list(lines, (long long)"}\n\n");
    ok = append_list(lines, (long long)"long long ep_net_send(long long fd, const char* data) {\n");
    ok = append_list(lines, (long long)"    if (!data) return 0;\n");
    ok = append_list(lines, (long long)"    return send((int)fd, data, strlen(data), 0);\n");
    ok = append_list(lines, (long long)"}\n\n");
    ok = append_list(lines, (long long)"char* ep_net_recv(long long fd, long long max_len) {\n");
    ok = append_list(lines, (long long)"    char* buf = malloc(max_len + 1);\n");
    ok = append_list(lines, (long long)"    if (!buf) {\n");
    ok = append_list(lines, (long long)"        char* empty = malloc(1);\n");
    ok = append_list(lines, (long long)"        if (empty) empty[0] = '\\0';\n");
    ok = append_list(lines, (long long)"        return empty;\n");
    ok = append_list(lines, (long long)"    }\n");
    ok = append_list(lines, (long long)"    ssize_t n = recv((int)fd, buf, max_len, 0);\n");
    ok = append_list(lines, (long long)"    if (n < 0) n = 0;\n");
    ok = append_list(lines, (long long)"    buf[n] = '\\0';\n");
    ok = append_list(lines, (long long)"    return buf;\n");
    ok = append_list(lines, (long long)"}\n\n");
    ok = append_list(lines, (long long)"long long ep_net_close(long long fd) {\n");
    ok = append_list(lines, (long long)"    return close((int)fd);\n");
    ok = append_list(lines, (long long)"}\n\n");
    ok = append_list(lines, (long long)"#define ROTRIGHT(word,bits) (((word) >> (bits)) | ((word) << (32-(bits))))\n");
    ok = append_list(lines, (long long)"#define CH(x,y,z) (((x) & (y)) ^ (~(x) & (z)))\n");
    ok = append_list(lines, (long long)"#define MAJ(x,y,z) (((x) & (y)) ^ ((x) & (z)) ^ ((y) & (z)))\n");
    ok = append_list(lines, (long long)"#define EP0(x) (ROTRIGHT(x,2) ^ ROTRIGHT(x,13) ^ ROTRIGHT(x,22))\n");
    ok = append_list(lines, (long long)"#define EP1(x) (ROTRIGHT(x,6) ^ ROTRIGHT(x,11) ^ ROTRIGHT(x,25))\n");
    ok = append_list(lines, (long long)"#define SIG0(x) (ROTRIGHT(x,7) ^ ROTRIGHT(x,18) ^ ((x) >> 3))\n");
    ok = append_list(lines, (long long)"#define SIG1(x) (ROTRIGHT(x,17) ^ ROTRIGHT(x,19) ^ ((x) >> 10))\n\n");
    ok = append_list(lines, (long long)"typedef struct {\n");
    ok = append_list(lines, (long long)"    unsigned char data[64];\n");
    ok = append_list(lines, (long long)"    unsigned int datalen;\n");
    ok = append_list(lines, (long long)"    unsigned long long bitlen;\n");
    ok = append_list(lines, (long long)"    unsigned int state[8];\n");
    ok = append_list(lines, (long long)"} EP_SHA256_CTX;\n\n");
    ok = append_list(lines, (long long)"static const unsigned int sha256_k[64] = {\n");
    ok = append_list(lines, (long long)"    0x428a2f98,0x71374491,0xb5c0fbcf,0xe9b5dba5,0x3956c25b,0x59f111f1,0x923f82a4,0xab1c5ed5,\n");
    ok = append_list(lines, (long long)"    0xd807aa98,0x12835b01,0x243185be,0x550c7dc3,0x72be5d74,0x80deb1fe,0x9bdc06a7,0xc19bf174,\n");
    ok = append_list(lines, (long long)"    0xe49b69c1,0xefbe4786,0x0fc19dc6,0x240ca1cc,0x2de92c6f,0x4a7484aa,0x5cb0a9dc,0x76f988da,\n");
    ok = append_list(lines, (long long)"    0x983e5152,0xa831c66d,0xb00327c8,0xbf597fc7,0xc6e00bf3,0xd5a79147,0x06ca6351,0x14292967,\n");
    ok = append_list(lines, (long long)"    0x27b70a85,0x2e1b2138,0x4d2c6dfc,0x53380d13,0x650a7354,0x766a0abb,0x81c2c92e,0x92722c85,\n");
    ok = append_list(lines, (long long)"    0xa2bfe8a1,0xa81a664b,0xc24b8b70,0xc76c51a3,0xd192e819,0xd6990624,0xf40e3585,0x106aa070,\n");
    ok = append_list(lines, (long long)"    0x19a4c116,0x1e376c08,0x2748774c,0x34b0bcb5,0x391c0cb3,0x4ed8aa4a,0x5b9cca4f,0x682e6ff3,\n");
    ok = append_list(lines, (long long)"    0x748f82ee,0x78a5636f,0x84c87814,0x8cc70208,0x90befffa,0xa4506ceb,0xbef9a3f7,0xc67178f2\n");
    ok = append_list(lines, (long long)"};\n\n");
    ok = append_list(lines, (long long)"void ep_sha256_transform(EP_SHA256_CTX *ctx, const unsigned char *data) {\n");
    ok = append_list(lines, (long long)"    unsigned int a, b, c, d, e, f, g, h, i, j, t1, t2, m[64];\n");
    ok = append_list(lines, (long long)"    for (i = 0, j = 0; i < 16; ++i, j += 4)\n");
    ok = append_list(lines, (long long)"        m[i] = (data[j] << 24) | (data[j + 1] << 16) | (data[j + 2] << 8) | (data[j + 3]);\n");
    ok = append_list(lines, (long long)"    for ( ; i < 64; ++i)\n");
    ok = append_list(lines, (long long)"        m[i] = SIG1(m[i - 2]) + m[i - 7] + SIG0(m[i - 15]) + m[i - 16];\n");
    ok = append_list(lines, (long long)"    a = ctx->state[0]; b = ctx->state[1]; c = ctx->state[2]; d = ctx->state[3];\n");
    ok = append_list(lines, (long long)"    e = ctx->state[4]; f = ctx->state[5]; g = ctx->state[6]; h = ctx->state[7];\n");
    ok = append_list(lines, (long long)"    for (i = 0; i < 64; ++i) {\n");
    ok = append_list(lines, (long long)"        t1 = h + EP1(e) + CH(e,f,g) + sha256_k[i] + m[i];\n");
    ok = append_list(lines, (long long)"        t2 = EP0(a) + MAJ(a,b,c);\n");
    ok = append_list(lines, (long long)"        h = g; g = f; f = e; e = d + t1; d = c; c = b; b = a; a = t1 + t2;\n");
    ok = append_list(lines, (long long)"    }\n");
    ok = append_list(lines, (long long)"    ctx->state[0] += a; ctx->state[1] += b; ctx->state[2] += c; ctx->state[3] += d;\n");
    ok = append_list(lines, (long long)"    ctx->state[4] += e; ctx->state[5] += f; ctx->state[6] += g; ctx->state[7] += h;\n");
    ok = append_list(lines, (long long)"}\n\n");
    ok = append_list(lines, (long long)"void ep_sha256_init(EP_SHA256_CTX *ctx) {\n");
    ok = append_list(lines, (long long)"    ctx->datalen = 0; ctx->bitlen = 0;\n");
    ok = append_list(lines, (long long)"    ctx->state[0] = 0x6a09e667; ctx->state[1] = 0xbb67ae85; ctx->state[2] = 0x3c6ef372; ctx->state[3] = 0xa54ff53a;\n");
    ok = append_list(lines, (long long)"    ctx->state[4] = 0x510e527f; ctx->state[5] = 0x9b05688c; ctx->state[6] = 0x1f83d9ab; ctx->state[7] = 0x5be0cd19;\n");
    ok = append_list(lines, (long long)"}\n\n");
    ok = append_list(lines, (long long)"void ep_sha256_update(EP_SHA256_CTX *ctx, const unsigned char *data, size_t len) {\n");
    ok = append_list(lines, (long long)"    for (size_t i = 0; i < len; ++i) {\n");
    ok = append_list(lines, (long long)"        ctx->data[ctx->datalen] = data[i];\n");
    ok = append_list(lines, (long long)"        ctx->datalen++;\n");
    ok = append_list(lines, (long long)"        if (ctx->datalen == 64) {\n");
    ok = append_list(lines, (long long)"            ep_sha256_transform(ctx, ctx->data);\n");
    ok = append_list(lines, (long long)"            ctx->bitlen += 512;\n");
    ok = append_list(lines, (long long)"            ctx->datalen = 0;\n");
    ok = append_list(lines, (long long)"        }\n");
    ok = append_list(lines, (long long)"    }\n");
    ok = append_list(lines, (long long)"}\n\n");
    ok = append_list(lines, (long long)"void ep_sha256_final(EP_SHA256_CTX *ctx, unsigned char *hash) {\n");
    ok = append_list(lines, (long long)"    unsigned int i = ctx->datalen;\n");
    ok = append_list(lines, (long long)"    if (ctx->datalen < 56) {\n");
    ok = append_list(lines, (long long)"        ctx->data[i++] = 0x80;\n");
    ok = append_list(lines, (long long)"        while (i < 56) ctx->data[i++] = 0x00;\n");
    ok = append_list(lines, (long long)"    } else {\n");
    ok = append_list(lines, (long long)"        ctx->data[i++] = 0x80;\n");
    ok = append_list(lines, (long long)"        while (i < 64) ctx->data[i++] = 0x00;\n");
    ok = append_list(lines, (long long)"        ep_sha256_transform(ctx, ctx->data);\n");
    ok = append_list(lines, (long long)"        memset(ctx->data, 0, 56);\n");
    ok = append_list(lines, (long long)"    }\n");
    ok = append_list(lines, (long long)"    ctx->bitlen += ctx->datalen * 8;\n");
    ok = append_list(lines, (long long)"    ctx->data[63] = ctx->bitlen; ctx->data[62] = ctx->bitlen >> 8;\n");
    ok = append_list(lines, (long long)"    ctx->data[61] = ctx->bitlen >> 16; ctx->data[60] = ctx->bitlen >> 24;\n");
    ok = append_list(lines, (long long)"    ctx->data[59] = ctx->bitlen >> 32; ctx->data[58] = ctx->bitlen >> 40;\n");
    ok = append_list(lines, (long long)"    ctx->data[57] = ctx->bitlen >> 48; ctx->data[56] = ctx->bitlen >> 56;\n");
    ok = append_list(lines, (long long)"    ep_sha256_transform(ctx, ctx->data);\n");
    ok = append_list(lines, (long long)"    for (i = 0; i < 4; ++i) {\n");
    ok = append_list(lines, (long long)"        hash[i]      = (ctx->state[0] >> (24 - i * 8)) & 0x000000ff;\n");
    ok = append_list(lines, (long long)"        hash[i + 4]  = (ctx->state[1] >> (24 - i * 8)) & 0x000000ff;\n");
    ok = append_list(lines, (long long)"        hash[i + 8]  = (ctx->state[2] >> (24 - i * 8)) & 0x000000ff;\n");
    ok = append_list(lines, (long long)"        hash[i + 12] = (ctx->state[3] >> (24 - i * 8)) & 0x000000ff;\n");
    ok = append_list(lines, (long long)"        hash[i + 16] = (ctx->state[4] >> (24 - i * 8)) & 0x000000ff;\n");
    ok = append_list(lines, (long long)"        hash[i + 20] = (ctx->state[5] >> (24 - i * 8)) & 0x000000ff;\n");
    ok = append_list(lines, (long long)"        hash[i + 24] = (ctx->state[6] >> (24 - i * 8)) & 0x000000ff;\n");
    ok = append_list(lines, (long long)"        hash[i + 28] = (ctx->state[7] >> (24 - i * 8)) & 0x000000ff;\n");
    ok = append_list(lines, (long long)"    }\n");
    ok = append_list(lines, (long long)"}\n\n");
    ok = append_list(lines, (long long)"char* ep_sha256(const char* s) {\n");
    ok = append_list(lines, (long long)"    if (!s) s = \"\";\n");
    ok = append_list(lines, (long long)"    EP_SHA256_CTX ctx;\n");
    ok = append_list(lines, (long long)"    ep_sha256_init(&ctx);\n");
    ok = append_list(lines, (long long)"    ep_sha256_update(&ctx, (const unsigned char*)s, strlen(s));\n");
    ok = append_list(lines, (long long)"    unsigned char hash[32];\n");
    ok = append_list(lines, (long long)"    ep_sha256_final(&ctx, hash);\n");
    ok = append_list(lines, (long long)"    char* result = malloc(65);\n");
    ok = append_list(lines, (long long)"    if (result) {\n");
    ok = append_list(lines, (long long)"        for (int i = 0; i < 32; i++) {\n");
    ok = append_list(lines, (long long)"            sprintf(result + (i * 2), \"%02x\", hash[i]);\n");
    ok = append_list(lines, (long long)"        }\n");
    ok = append_list(lines, (long long)"        result[64] = '\\0';\n");
    ok = append_list(lines, (long long)"    }\n");
    ok = append_list(lines, (long long)"    return result;\n");
    ok = append_list(lines, (long long)"}\n\n");
    ok = append_list(lines, (long long)"typedef struct {\n");
    ok = append_list(lines, (long long)"    unsigned int count[2];\n");
    ok = append_list(lines, (long long)"    unsigned int state[4];\n");
    ok = append_list(lines, (long long)"    unsigned char buffer[64];\n");
    ok = append_list(lines, (long long)"} EP_MD5_CTX;\n\n");
    ok = append_list(lines, (long long)"#define F(x,y,z) (((x) & (y)) | (~(x) & (z)))\n");
    ok = append_list(lines, (long long)"#define G(x,y,z) (((x) & (z)) | ((y) & ~(z)))\n");
    ok = append_list(lines, (long long)"#define H(x,y,z) ((x) ^ (y) ^ (z))\n");
    ok = append_list(lines, (long long)"#define I(x,y,z) ((y) ^ ((x) | ~(z)))\n");
    ok = append_list(lines, (long long)"#define ROTATE_LEFT(x,n) (((x) << (n)) | ((x) >> (32-(n))))\n\n");
    ok = append_list(lines, (long long)"#define FF(a,b,c,d,x,s,ac) { (a) += F((b),(c),(d)) + (x) + (ac); (a) = ROTATE_LEFT((a),(s)); (a) += (b); }\n");
    ok = append_list(lines, (long long)"#define GG(a,b,c,d,x,s,ac) { (a) += G((b),(c),(d)) + (x) + (ac); (a) = ROTATE_LEFT((a),(s)); (a) += (b); }\n");
    ok = append_list(lines, (long long)"#define HH(a,b,c,d,x,s,ac) { (a) += H((b),(c),(d)) + (x) + (ac); (a) = ROTATE_LEFT((a),(s)); (a) += (b); }\n");
    ok = append_list(lines, (long long)"#define II(a,b,c,d,x,s,ac) { (a) += I((b),(c),(d)) + (x) + (ac); (a) = ROTATE_LEFT((a),(s)); (a) += (b); }\n\n");
    ok = append_list(lines, (long long)"void ep_md5_init(EP_MD5_CTX *ctx) {\n");
    ok = append_list(lines, (long long)"    ctx->count[0] = ctx->count[1] = 0;\n");
    ok = append_list(lines, (long long)"    ctx->state[0] = 0x67452301;\n");
    ok = append_list(lines, (long long)"    ctx->state[1] = 0xefcdab89;\n");
    ok = append_list(lines, (long long)"    ctx->state[2] = 0x98badcfe;\n");
    ok = append_list(lines, (long long)"    ctx->state[3] = 0x10325476;\n");
    ok = append_list(lines, (long long)"}\n\n");
    ok = append_list(lines, (long long)"void ep_md5_transform(unsigned int state[4], const unsigned char block[64]) {\n");
    ok = append_list(lines, (long long)"    unsigned int a = state[0], b = state[1], c = state[2], d = state[3], x[16];\n");
    ok = append_list(lines, (long long)"    for (int i = 0, j = 0; i < 16; i++, j += 4)\n");
    ok = append_list(lines, (long long)"        x[i] = (block[j]) | (block[j+1] << 8) | (block[j+2] << 16) | (block[j+3] << 24);\n");
    ok = append_list(lines, (long long)"    FF(a, b, c, d, x[0], 7, 0xd76aa478); FF(d, a, b, c, x[1], 12, 0xe8c7b756); FF(c, d, a, b, x[2], 17, 0x242070db); FF(b, c, d, a, x[3], 22, 0xc1bdceee);\n");
    ok = append_list(lines, (long long)"    FF(a, b, c, d, x[4], 7, 0xf57c0faf); FF(d, a, b, c, x[5], 12, 0x4787c62a); FF(c, d, a, b, x[6], 17, 0xa8304613); FF(b, c, d, a, x[7], 22, 0xfd469501);\n");
    ok = append_list(lines, (long long)"    FF(a, b, c, d, x[8], 7, 0x698098d8); FF(d, a, b, c, x[9], 12, 0x8b44f7af); FF(c, d, a, b, x[10], 17, 0xffff5bb1); FF(b, c, d, a, x[11], 22, 0x895cd7be);\n");
    ok = append_list(lines, (long long)"    FF(a, b, c, d, x[12], 7, 0x6b901122); FF(d, a, b, c, x[13], 12, 0xfd987193); FF(c, d, a, b, x[14], 17, 0xa679438e); FF(b, c, d, a, x[15], 22, 0x49b40821);\n");
    ok = append_list(lines, (long long)"    GG(a, b, c, d, x[1], 5, 0xf61e2562); GG(d, a, b, c, x[6], 9, 0xc040b340); GG(c, d, a, b, x[11], 14, 0x265e5a51); GG(b, c, d, a, x[0], 20, 0xe9b6c7aa);\n");
    ok = append_list(lines, (long long)"    GG(a, b, c, d, x[5], 5, 0xd62f105d); GG(d, a, b, c, x[10], 9, 0x02441453); GG(c, d, a, b, x[15], 14, 0xd8a1e681); GG(b, c, d, a, x[4], 20, 0xe7d3fbc8);\n");
    ok = append_list(lines, (long long)"    GG(a, b, c, d, x[9], 5, 0x21e1cde6); GG(d, a, b, c, x[14], 9, 0xc33707d6); GG(c, d, a, b, x[3], 14, 0xf4d50d87); GG(b, c, d, a, x[8], 20, 0x455a14ed);\n");
    ok = append_list(lines, (long long)"    GG(a, b, c, d, x[13], 5, 0xa9e3e905); GG(d, a, b, c, x[2], 9, 0xfcefa3f8); GG(c, d, a, b, x[7], 14, 0x676f02d9); GG(b, c, d, a, x[12], 20, 0x8d2a4c8a);\n");
    ok = append_list(lines, (long long)"    HH(a, b, c, d, x[5], 4, 0xfffa3942); HH(d, a, b, c, x[8], 11, 0x8771f681); HH(c, d, a, b, x[11], 16, 0x6d9d6122); HH(b, c, d, a, x[14], 23, 0xfde5380c);\n");
    ok = append_list(lines, (long long)"    HH(a, b, c, d, x[1], 4, 0xa4beea44); HH(d, a, b, c, x[4], 11, 0x4bdecfa9); HH(c, d, a, b, x[7], 16, 0xf6bb4b60); HH(b, c, d, a, x[10], 23, 0xbebfbc70);\n");
    ok = append_list(lines, (long long)"    HH(a, b, c, d, x[13], 4, 0x289b7ec6); HH(d, a, b, c, x[0], 11, 0xeaa127fa); HH(c, d, a, b, x[3], 16, 0xd4ef3085); HH(b, c, d, a, x[6], 23, 0x04881d05);\n");
    ok = append_list(lines, (long long)"    HH(a, b, c, d, x[9], 4, 0xd9d4d039); HH(d, a, b, c, x[12], 11, 0xe6db99e5); HH(c, d, a, b, x[15], 16, 0x1fa27cf8); HH(b, c, d, a, x[2], 23, 0xc4ac5665);\n");
    ok = append_list(lines, (long long)"    II(a, b, c, d, x[0], 6, 0xf4292244); II(d, a, b, c, x[7], 10, 0x432aff97); II(c, d, a, b, x[14], 15, 0xab9423a7); II(b, c, d, a, x[5], 21, 0xfc93a039);\n");
    ok = append_list(lines, (long long)"    II(a, b, c, d, x[12], 6, 0x655b59c3); II(d, a, b, c, x[3], 10, 0x8f0ccc92); II(c, d, a, b, x[10], 15, 0xffeff47d); II(b, c, d, a, x[1], 21, 0x85845dd1);\n");
    ok = append_list(lines, (long long)"    II(a, b, c, d, x[8], 6, 0x6fa87e4f); II(d, a, b, c, x[15], 10, 0xfe2ce6e0); II(c, d, a, b, x[6], 15, 0xa3014314); II(b, c, d, a, x[13], 21, 0x4e0811a1);\n");
    ok = append_list(lines, (long long)"    II(a, b, c, d, x[4], 6, 0xf7537e82); II(d, a, b, c, x[11], 10, 0xbd3af235); II(c, d, a, b, x[2], 15, 0x2ad7d2bb); II(b, c, d, a, x[9], 21, 0xeb86d391);\n");
    ok = append_list(lines, (long long)"    state[0] += a; state[1] += b; state[2] += c; state[3] += d;\n");
    ok = append_list(lines, (long long)"}\n\n");
    ok = append_list(lines, (long long)"void ep_md5_update(EP_MD5_CTX *ctx, const unsigned char *input, size_t input_len) {\n");
    ok = append_list(lines, (long long)"    unsigned int i = 0, index = (ctx->count[0] >> 3) & 0x3F, part_len = 64 - index;\n");
    ok = append_list(lines, (long long)"    ctx->count[0] += input_len << 3;\n");
    ok = append_list(lines, (long long)"    if (ctx->count[0] < (input_len << 3)) ctx->count[1]++;\n");
    ok = append_list(lines, (long long)"    ctx->count[1] += input_len >> 29;\n");
    ok = append_list(lines, (long long)"    if (input_len >= part_len) {\n");
    ok = append_list(lines, (long long)"        memcpy(&ctx->buffer[index], input, part_len);\n");
    ok = append_list(lines, (long long)"        ep_md5_transform(ctx->state, ctx->buffer);\n");
    ok = append_list(lines, (long long)"        for (i = part_len; i + 63 < input_len; i += 64)\n");
    ok = append_list(lines, (long long)"            ep_md5_transform(ctx->state, &input[i]);\n");
    ok = append_list(lines, (long long)"        index = 0;\n");
    ok = append_list(lines, (long long)"    }\n");
    ok = append_list(lines, (long long)"    memcpy(&ctx->buffer[index], &input[i], input_len - i);\n");
    ok = append_list(lines, (long long)"}\n\n");
    ok = append_list(lines, (long long)"void ep_md5_final(EP_MD5_CTX *ctx, unsigned char digest[16]) {\n");
    ok = append_list(lines, (long long)"    unsigned char bits[8];\n");
    ok = append_list(lines, (long long)"    bits[0] = ctx->count[0]; bits[1] = ctx->count[0] >> 8; bits[2] = ctx->count[0] >> 16; bits[3] = ctx->count[0] >> 24;\n");
    ok = append_list(lines, (long long)"    bits[4] = ctx->count[1]; bits[5] = ctx->count[1] >> 8; bits[6] = ctx->count[1] >> 16; bits[7] = ctx->count[1] >> 24;\n");
    ok = append_list(lines, (long long)"    unsigned int index = (ctx->count[0] >> 3) & 0x3F, pad_len = (index < 56) ? (56 - index) : (120 - index);\n");
    ok = append_list(lines, (long long)"    unsigned char padding[64];\n");
    ok = append_list(lines, (long long)"    memset(padding, 0, 64); padding[0] = 0x80;\n");
    ok = append_list(lines, (long long)"    ep_md5_update(ctx, padding, pad_len);\n");
    ok = append_list(lines, (long long)"    ep_md5_update(ctx, bits, 8);\n");
    ok = append_list(lines, (long long)"    for (int i = 0; i < 4; i++) {\n");
    ok = append_list(lines, (long long)"        digest[i*4]     = ctx->state[i];\n");
    ok = append_list(lines, (long long)"        digest[i*4 + 1] = ctx->state[i] >> 8;\n");
    ok = append_list(lines, (long long)"        digest[i*4 + 2] = ctx->state[i] >> 16;\n");
    ok = append_list(lines, (long long)"        digest[i*4 + 3] = ctx->state[i] >> 24;\n");
    ok = append_list(lines, (long long)"    }\n");
    ok = append_list(lines, (long long)"}\n\n");
    ok = append_list(lines, (long long)"char* ep_md5(const char* s) {\n");
    ok = append_list(lines, (long long)"    if (!s) s = \"\";\n");
    ok = append_list(lines, (long long)"    EP_MD5_CTX ctx;\n");
    ok = append_list(lines, (long long)"    ep_md5_init(&ctx);\n");
    ok = append_list(lines, (long long)"    ep_md5_update(&ctx, (const unsigned char*)s, strlen(s));\n");
    ok = append_list(lines, (long long)"    unsigned char hash[16];\n");
    ok = append_list(lines, (long long)"    ep_md5_final(&ctx, hash);\n");
    ok = append_list(lines, (long long)"    char* result = malloc(33);\n");
    ok = append_list(lines, (long long)"    if (result) {\n");
    ok = append_list(lines, (long long)"        for (int i = 0; i < 16; i++) {\n");
    ok = append_list(lines, (long long)"            sprintf(result + (i * 2), \"%02x\", hash[i]);\n");
    ok = append_list(lines, (long long)"        }\n");
    ok = append_list(lines, (long long)"        result[32] = '\\0';\n");
    ok = append_list(lines, (long long)"    }\n");
    ok = append_list(lines, (long long)"    return result;\n");
    ok = append_list(lines, (long long)"}\n\n");
    ok = append_list(lines, (long long)"char* read_file_content(const char* filepath) {\n");
    ok = append_list(lines, (long long)"    char mode[3];\n");
    ok = append_list(lines, (long long)"    mode[0] = 'r';\n");
    ok = append_list(lines, (long long)"    mode[1] = 'b';\n");
    ok = append_list(lines, (long long)"    mode[2] = '\\0';\n");
    ok = append_list(lines, (long long)"    FILE* f = fopen(filepath, mode);\n");
    ok = append_list(lines, (long long)"    if (!f) {\n");
    ok = append_list(lines, (long long)"        char* empty = malloc(1);\n");
    ok = append_list(lines, (long long)"        if (empty) empty[0] = '\\0';\n");
    ok = append_list(lines, (long long)"        return empty;\n");
    ok = append_list(lines, (long long)"    }\n");
    ok = append_list(lines, (long long)"    fseek(f, 0, SEEK_END);\n");
    ok = append_list(lines, (long long)"    long size = ftell(f);\n");
    ok = append_list(lines, (long long)"    fseek(f, 0, SEEK_SET);\n");
    ok = append_list(lines, (long long)"    char* buf = malloc(size + 1);\n");
    ok = append_list(lines, (long long)"    if (!buf) {\n");
    ok = append_list(lines, (long long)"        fclose(f);\n");
    ok = append_list(lines, (long long)"        char* empty = malloc(1);\n");
    ok = append_list(lines, (long long)"        if (empty) empty[0] = '\\0';\n");
    ok = append_list(lines, (long long)"        return empty;\n");
    ok = append_list(lines, (long long)"    }\n");
    ok = append_list(lines, (long long)"    size_t read_bytes = fread(buf, 1, size, f);\n");
    ok = append_list(lines, (long long)"    buf[read_bytes] = '\\0';\n");
    ok = append_list(lines, (long long)"    fclose(f);\n");
    ok = append_list(lines, (long long)"    return buf;\n");
    ok = append_list(lines, (long long)"}\n\n");
    ok = append_list(lines, (long long)"long long string_length(const char* s) {\n");
    ok = append_list(lines, (long long)"    if (!s) return 0;\n");
    ok = append_list(lines, (long long)"    return strlen(s);\n");
    ok = append_list(lines, (long long)"}\n\n");
    ok = append_list(lines, (long long)"long long get_character(const char* s, long long index) {\n");
    ok = append_list(lines, (long long)"    if (!s) return 0;\n");
    ok = append_list(lines, (long long)"    long long len = strlen(s);\n");
    ok = append_list(lines, (long long)"    if (index < 0 || index >= len) return 0;\n");
    ok = append_list(lines, (long long)"    return (unsigned char)s[index];\n");
    ok = append_list(lines, (long long)"}\n\n");
    ok = append_list(lines, (long long)"typedef struct {\n");
    ok = append_list(lines, (long long)"    long long* data;\n");
    ok = append_list(lines, (long long)"    long long capacity;\n");
    ok = append_list(lines, (long long)"    long long length;\n");
    ok = append_list(lines, (long long)"} EpList;\n\n");
    ok = append_list(lines, (long long)"long long create_list(void) {\n");
    ok = append_list(lines, (long long)"    EpList* list = malloc(sizeof(EpList));\n");
    ok = append_list(lines, (long long)"    if (!list) return 0;\n");
    ok = append_list(lines, (long long)"    list->capacity = 4;\n");
    ok = append_list(lines, (long long)"    list->length = 0;\n");
    ok = append_list(lines, (long long)"    list->data = malloc(list->capacity * sizeof(long long));\n");
    ok = append_list(lines, (long long)"    return (long long)list;\n");
    ok = append_list(lines, (long long)"}\n\n");
    ok = append_list(lines, (long long)"long long append_list(long long list_ptr, long long value) {\n");
    ok = append_list(lines, (long long)"    EpList* list = (EpList*)list_ptr;\n");
    ok = append_list(lines, (long long)"    if (!list) return 0;\n");
    ok = append_list(lines, (long long)"    if (list->length >= list->capacity) {\n");
    ok = append_list(lines, (long long)"        list->capacity *= 2;\n");
    ok = append_list(lines, (long long)"        list->data = realloc(list->data, list->capacity * sizeof(long long));\n");
    ok = append_list(lines, (long long)"    }\n");
    ok = append_list(lines, (long long)"    list->data[list->length] = value;\n");
    ok = append_list(lines, (long long)"    list->length += 1;\n");
    ok = append_list(lines, (long long)"    return value;\n");
    ok = append_list(lines, (long long)"}\n\n");
    ok = append_list(lines, (long long)"long long get_list(long long list_ptr, long long index) {\n");
    ok = append_list(lines, (long long)"    EpList* list = (EpList*)list_ptr;\n");
    ok = append_list(lines, (long long)"    if (!list || index < 0 || index >= list->length) return 0;\n");
    ok = append_list(lines, (long long)"    return list->data[index];\n");
    ok = append_list(lines, (long long)"}\n\n");
    ok = append_list(lines, (long long)"long long set_list(long long list_ptr, long long index, long long value) {\n");
    ok = append_list(lines, (long long)"    EpList* list = (EpList*)list_ptr;\n");
    ok = append_list(lines, (long long)"    if (!list || index < 0 || index >= list->length) return 0;\n");
    ok = append_list(lines, (long long)"    list->data[index] = value;\n");
    ok = append_list(lines, (long long)"    return value;\n");
    ok = append_list(lines, (long long)"}\n\n");
    ok = append_list(lines, (long long)"long long length_list(long long list_ptr) {\n");
    ok = append_list(lines, (long long)"    EpList* list = (EpList*)list_ptr;\n");
    ok = append_list(lines, (long long)"    if (!list) return 0;\n");
    ok = append_list(lines, (long long)"    return list->length;\n");
    ok = append_list(lines, (long long)"}\n\n");
    ok = append_list(lines, (long long)"void free_list(long long list_ptr) {\n");
    ok = append_list(lines, (long long)"    EpList* list = (EpList*)list_ptr;\n");
    ok = append_list(lines, (long long)"    if (!list) return;\n");
    ok = append_list(lines, (long long)"    free(list->data);\n");
    ok = append_list(lines, (long long)"    free(list);\n");
    ok = append_list(lines, (long long)"}\n\n");
    ok = append_list(lines, (long long)"int ep_argc = 0;\n");
    ok = append_list(lines, (long long)"char** ep_argv = NULL;\n\n");
    ok = append_list(lines, (long long)"void init_ep_args(int argc, char** argv) {\n");
    ok = append_list(lines, (long long)"    ep_argc = argc;\n");
    ok = append_list(lines, (long long)"    ep_argv = argv;\n");
    ok = append_list(lines, (long long)"}\n\n");
    ok = append_list(lines, (long long)"long long get_argument_count(void) {\n");
    ok = append_list(lines, (long long)"    return ep_argc;\n");
    ok = append_list(lines, (long long)"}\n\n");
    ok = append_list(lines, (long long)"const char* get_argument(long long index) {\n");
    ok = append_list(lines, (long long)"    if (index < 0 || index >= ep_argc) {\n");
    ok = append_list(lines, (long long)"        char* empty = malloc(1);\n");
    ok = append_list(lines, (long long)"        if (empty) empty[0] = '\\0';\n");
    ok = append_list(lines, (long long)"        return empty;\n");
    ok = append_list(lines, (long long)"    }\n");
    ok = append_list(lines, (long long)"    return ep_argv[index];\n");
    ok = append_list(lines, (long long)"}\n\n");
    ok = append_list(lines, (long long)"long long write_file_content(const char* filepath, const char* content) {\n");
    ok = append_list(lines, (long long)"    char mode[3];\n");
    ok = append_list(lines, (long long)"    mode[0] = 'w';\n");
    ok = append_list(lines, (long long)"    mode[1] = 'b';\n");
    ok = append_list(lines, (long long)"    mode[2] = '\\0';\n");
    ok = append_list(lines, (long long)"    FILE* f = fopen(filepath, mode);\n");
    ok = append_list(lines, (long long)"    if (!f) return 0;\n");
    ok = append_list(lines, (long long)"    size_t len = strlen(content);\n");
    ok = append_list(lines, (long long)"    size_t written = fwrite(content, 1, len, f);\n");
    ok = append_list(lines, (long long)"    fclose(f);\n");
    ok = append_list(lines, (long long)"    return written == len ? 1 : 0;\n");
    ok = append_list(lines, (long long)"}\n\n");
    ok = append_list(lines, (long long)"long long run_command(const char* command) {\n");
    ok = append_list(lines, (long long)"    if (!command) return -1;\n");
    ok = append_list(lines, (long long)"    return system(command);\n");
    ok = append_list(lines, (long long)"}\n\n");
    ok = append_list(lines, (long long)"char* substring(const char* s, long long start, long long len) {\n");
    ok = append_list(lines, (long long)"    if (!s) {\n");
    ok = append_list(lines, (long long)"        char* empty = malloc(1);\n");
    ok = append_list(lines, (long long)"        if (empty) empty[0] = '\\0';\n");
    ok = append_list(lines, (long long)"        return empty;\n");
    ok = append_list(lines, (long long)"    }\n");
    ok = append_list(lines, (long long)"    long long total_len = strlen(s);\n");
    ok = append_list(lines, (long long)"    if (start < 0 || start >= total_len || len <= 0) {\n");
    ok = append_list(lines, (long long)"        char* empty = malloc(1);\n");
    ok = append_list(lines, (long long)"        if (empty) empty[0] = '\\0';\n");
    ok = append_list(lines, (long long)"        return empty;\n");
    ok = append_list(lines, (long long)"    }\n");
    ok = append_list(lines, (long long)"    if (start + len > total_len) {\n");
    ok = append_list(lines, (long long)"        len = total_len - start;\n");
    ok = append_list(lines, (long long)"    }\n");
    ok = append_list(lines, (long long)"    char* sub = malloc(len + 1);\n");
    ok = append_list(lines, (long long)"    if (!sub) {\n");
    ok = append_list(lines, (long long)"        char* empty = malloc(1);\n");
    ok = append_list(lines, (long long)"        if (empty) empty[0] = '\\0';\n");
    ok = append_list(lines, (long long)"        return empty;\n");
    ok = append_list(lines, (long long)"    }\n");
    ok = append_list(lines, (long long)"    strncpy(sub, s + start, len);\n");
    ok = append_list(lines, (long long)"    sub[len] = '\\0';\n");
    ok = append_list(lines, (long long)"    return sub;\n");
    ok = append_list(lines, (long long)"}\n\n");
    ok = append_list(lines, (long long)"char* string_from_list(long long list_ptr) {\n");
    ok = append_list(lines, (long long)"    EpList* list = (EpList*)list_ptr;\n");
    ok = append_list(lines, (long long)"    if (!list) {\n");
    ok = append_list(lines, (long long)"        char* empty = malloc(1);\n");
    ok = append_list(lines, (long long)"        if (empty) empty[0] = '\\0';\n");
    ok = append_list(lines, (long long)"        return empty;\n");
    ok = append_list(lines, (long long)"    }\n");
    ok = append_list(lines, (long long)"    char* s = malloc(list->length + 1);\n");
    ok = append_list(lines, (long long)"    if (!s) {\n");
    ok = append_list(lines, (long long)"        char* empty = malloc(1);\n");
    ok = append_list(lines, (long long)"        if (empty) empty[0] = '\\0';\n");
    ok = append_list(lines, (long long)"        return empty;\n");
    ok = append_list(lines, (long long)"    }\n");
    ok = append_list(lines, (long long)"    for (long long i = 0; i < list->length; i++) {\n");
    ok = append_list(lines, (long long)"        s[i] = (char)list->data[i];\n");
    ok = append_list(lines, (long long)"    }\n");
    ok = append_list(lines, (long long)"    s[list->length] = '\\0';\n");
    ok = append_list(lines, (long long)"    return s;\n");
    ok = append_list(lines, (long long)"}\n\n");
    ok = append_list(lines, (long long)"long long pop_list(long long list_ptr) {\n");
    ok = append_list(lines, (long long)"    EpList* list = (EpList*)list_ptr;\n");
    ok = append_list(lines, (long long)"    if (!list || list->length <= 0) return 0;\n");
    ok = append_list(lines, (long long)"    list->length -= 1;\n");
    ok = append_list(lines, (long long)"    return list->data[list->length];\n");
    ok = append_list(lines, (long long)"}\n\n");
    ok = append_list(lines, (long long)"long long display_string(const char* s) {\n");
    ok = append_list(lines, (long long)"    if (s) puts(s);\n");
    ok = append_list(lines, (long long)"    return 0;\n");
    ok = append_list(lines, (long long)"}\n\n");
    ret_val = join_strings(lines);
    goto L_cleanup;
L_cleanup:
    free_list(lines);
    return ret_val;
}

long long get_c_main_source() {
    long long lines = 0;
    long long ok = 0;
    long long ret_val = 0;

    {
        long long tmp_val = create_list();
        free_list(lines);
        lines = tmp_val;
    }
    ok = append_list(lines, (long long)"\n/* Bootstrapper C main */\n");
    ok = append_list(lines, (long long)"int main(int argc, char** argv) {\n");
    ok = append_list(lines, (long long)"    init_ep_args(argc, argv);\n");
    ok = append_list(lines, (long long)"    _main();\n");
    ok = append_list(lines, (long long)"    return 0;\n");
    ok = append_list(lines, (long long)"}\n");
    ret_val = join_strings(lines);
    goto L_cleanup;
L_cleanup:
    free_list(lines);
    return ret_val;
}

long long generate_c(long long program) {
    long long state = 0;
    long long ok = 0;
    long long funcs = 0;
    long long len = 0;
    long long idx = 0;
    long long func = 0;
    long long name = 0;
    long long params = 0;
    long long p_len = 0;
    long long c_name = 0;
    long long proto = 0;
    long long p_i = 0;
    long long lines = 0;
    long long c_code = 0;
    long long ret_val = 0;

    {
        long long tmp_val = create_codegen_state();
        free_list(state);
        state = tmp_val;
    }
    ok = analyze_return_types(state, program);
    ok = emit(state, get_c_runtime_source());
    funcs = get_list(program, 2);
    len = length_list(funcs);
    ok = emit(state, (long long)"\n/* User Function Prototypes */\n");
    idx = 0;
    while ((idx < len)) {
    func = get_list(funcs, idx);
    name = get_list(func, 1);
    params = get_list(func, 2);
    p_len = length_list(params);
    c_name = name;
    if ((strcmp((char*)(long long)"main", (char*)name) == 0)) {
    c_name = (long long)"_main";
    }
    proto = (long long)"long long ";
    proto = string_concat(proto, c_name);
    proto = string_concat(proto, (long long)"(");
    p_i = 0;
    while ((p_i < p_len)) {
    proto = string_concat(proto, (long long)"long long");
    if ((p_i < (p_len - 1))) {
    proto = string_concat(proto, (long long)", ");
    }
    p_i = (p_i + 1);
    }
    proto = string_concat(proto, (long long)");\n");
    ok = emit(state, proto);
    idx = (idx + 1);
    }
    ok = emit(state, (long long)"\n");
    idx = 0;
    while ((idx < len)) {
    func = get_list(funcs, idx);
    ok = gen_function(state, func);
    idx = (idx + 1);
    }
    ok = emit(state, get_c_main_source());
    lines = get_list(state, 0);
    c_code = join_strings(lines);
    ret_val = c_code;
    goto L_cleanup;
L_cleanup:
    free_list(state);
    return ret_val;
}

long long get_file_stem(long long path) {
    long long len = 0;
    long long last_slash = 0;
    long long idx = 0;
    long long ch = 0;
    long long start = 0;
    long long dot_pos = 0;
    long long idx2 = 0;
    long long stem_len = 0;
    long long stem = 0;
    long long ret_val = 0;

    len = string_length((char*)path);
    last_slash = (0 - 1);
    idx = 0;
    while ((idx < len)) {
    ch = get_character((char*)path, idx);
    if ((ch == 47)) {
    last_slash = idx;
    }
    idx = (idx + 1);
    }
    start = (last_slash + 1);
    dot_pos = len;
    idx2 = start;
    while ((idx2 < len)) {
    ch = get_character((char*)path, idx2);
    if ((ch == 46)) {
    dot_pos = idx2;
    }
    idx2 = (idx2 + 1);
    }
    stem_len = (dot_pos - start);
    stem = (long long)substring((char*)path, start, stem_len);
    ret_val = stem;
    goto L_cleanup;
L_cleanup:
    return ret_val;
}

long long get_file_dir(long long path) {
    long long len = 0;
    long long last_slash = 0;
    long long idx = 0;
    long long ch = 0;
    long long ret_val = 0;

    len = string_length((char*)path);
    last_slash = (0 - 1);
    idx = 0;
    while ((idx < len)) {
    ch = get_character((char*)path, idx);
    if ((ch == 47)) {
    last_slash = idx;
    }
    idx = (idx + 1);
    }
    if ((last_slash < 0)) {
    ret_val = (long long)"./";
    goto L_cleanup;
    }
    ret_val = (long long)substring((char*)path, 0, (last_slash + 1));
    goto L_cleanup;
L_cleanup:
    return ret_val;
}

long long contains_string(long long list, long long s) {
    long long len = 0;
    long long idx = 0;
    long long item = 0;
    long long ret_val = 0;

    len = length_list(list);
    idx = 0;
    while ((idx < len)) {
    item = get_list(list, idx);
    if ((strcmp((char*)string_concat(s, (long long)""), (char*)item) == 0)) {
    ret_val = 1;
    goto L_cleanup;
    }
    idx = (idx + 1);
    }
    ret_val = 0;
    goto L_cleanup;
L_cleanup:
    return ret_val;
}

long long resolve_import_path(long long current_file, long long import_path) {
    long long std_path = 0;
    long long std_path_ep = 0;
    long long dir = 0;
    long long resolved = 0;
    long long len = 0;
    long long ext = 0;
    long long ret_val = 0;

    if ((((((strcmp((char*)(long long)"math", (char*)import_path) == 0) || (strcmp((char*)(long long)"hash", (char*)import_path) == 0)) || (strcmp((char*)(long long)"net", (char*)import_path) == 0)) || (strcmp((char*)(long long)"json", (char*)import_path) == 0)) || (strcmp((char*)(long long)"string", (char*)import_path) == 0))) {
    std_path = string_concat((long long)"stdlib/", import_path);
    std_path_ep = string_concat(std_path, (long long)".ep");
    ret_val = std_path_ep;
    goto L_cleanup;
    }
    dir = get_file_dir(current_file);
    resolved = string_concat(dir, import_path);
    len = string_length((char*)resolved);
    if ((len > 3)) {
    ext = (long long)substring((char*)resolved, (len - 3), 3);
    if ((strcmp((char*)(long long)".ep", (char*)ext) == 0)) {
    ret_val = resolved;
    goto L_cleanup;
    }
    }
    ret_val = string_concat(resolved, (long long)".ep");
    goto L_cleanup;
L_cleanup:
    return ret_val;
}

long long parse_all_modules(long long current_file, long long parsed_files, long long all_functions) {
    long long has_parsed = 0;
    long long ok = 0;
    long long source = 0;
    long long tokens = 0;
    long long state = 0;
    long long program_ast = 0;
    long long imports = 0;
    long long funcs = 0;
    long long funcs_len = 0;
    long long f_idx = 0;
    long long func = 0;
    long long imp_len = 0;
    long long i_idx = 0;
    long long imp = 0;
    long long resolved_path = 0;
    long long status = 0;
    long long ret_val = 0;

    has_parsed = contains_string(parsed_files, current_file);
    if ((has_parsed == 1)) {
    ret_val = 0;
    goto L_cleanup;
    }
    ok = append_list(parsed_files, current_file);
    source = (long long)read_file_content((char*)current_file);
    if ((string_length((char*)source) == 0)) {
    printf("%s\n", (char*)(long long)"Compiler Error: Failed to read file or file is empty:");
    ok = display_string((char*)current_file);
    ret_val = 1;
    goto L_cleanup;
    }
    tokens = tokenize_source(source);
    {
        long long tmp_val = create_parser_state(tokens);
        free_list(state);
        state = tmp_val;
    }
    program_ast = parse_program(state);
    imports = get_list(program_ast, 1);
    funcs = get_list(program_ast, 2);
    funcs_len = length_list(funcs);
    f_idx = 0;
    while ((f_idx < funcs_len)) {
    func = get_list(funcs, f_idx);
    ok = append_list(all_functions, func);
    f_idx = (f_idx + 1);
    }
    imp_len = length_list(imports);
    i_idx = 0;
    while ((i_idx < imp_len)) {
    imp = get_list(imports, i_idx);
    resolved_path = resolve_import_path(current_file, imp);
    status = parse_all_modules(resolved_path, parsed_files, all_functions);
    if ((status != 0)) {
    ret_val = status;
    goto L_cleanup;
    }
    i_idx = (i_idx + 1);
    }
    ret_val = 0;
    goto L_cleanup;
L_cleanup:
    free_list(state);
    return ret_val;
}

long long _main() {
    long long arg_count = 0;
    long long input_path = 0;
    long long stem = 0;
    long long all_functions = 0;
    long long parsed_files = 0;
    long long status = 0;
    long long f_names = 0;
    long long all_len = 0;
    long long idx = 0;
    long long duplicate_found = 0;
    long long func = 0;
    long long name = 0;
    long long ok = 0;
    long long empty_imports = 0;
    long long program_ast = 0;
    long long c_code = 0;
    long long c_path = 0;
    long long cmd_half1 = 0;
    long long cmd_half2 = 0;
    long long compile_cmd = 0;
    long long ret_val = 0;

    arg_count = get_argument_count();
    if ((arg_count < 2)) {
    printf("%s\n", (char*)(long long)"Usage: epc <filename.ep>");
    ret_val = 1;
    goto L_cleanup;
    }
    input_path = (long long)get_argument(1);
    stem = get_file_stem(input_path);
    printf("%s\n", (char*)(long long)"[1/3] Tokenizing and Parsing...");
    {
        long long tmp_val = create_list();
        free_list(all_functions);
        all_functions = tmp_val;
    }
    {
        long long tmp_val = create_list();
        free_list(parsed_files);
        parsed_files = tmp_val;
    }
    status = parse_all_modules(input_path, parsed_files, all_functions);
    if ((status != 0)) {
    ret_val = 1;
    goto L_cleanup;
    }
    {
        long long tmp_val = create_list();
        free_list(f_names);
        f_names = tmp_val;
    }
    all_len = length_list(all_functions);
    idx = 0;
    duplicate_found = 0;
    while (((idx < all_len) && (duplicate_found == 0))) {
    func = get_list(all_functions, idx);
    name = get_list(func, 1);
    if ((contains_string(f_names, name) == 1)) {
    printf("%s\n", (char*)(long long)"Compiler Error: Function is defined multiple times:");
    ok = display_string((char*)name);
    duplicate_found = 1;
    } else {
    ok = append_list(f_names, name);
    }
    idx = (idx + 1);
    }
    if ((duplicate_found == 1)) {
    ret_val = 1;
    goto L_cleanup;
    }
    {
        long long tmp_val = create_list();
        free_list(empty_imports);
        empty_imports = tmp_val;
    }
    {
        long long tmp_val = create_list();
        free_list(program_ast);
        program_ast = tmp_val;
    }
    ok = append_list(program_ast, 13);
    ok = append_list(program_ast, empty_imports);
    ok = append_list(program_ast, all_functions);
    printf("%s\n", (char*)(long long)"[2/3] Generating C Source...");
    c_code = generate_c(program_ast);
    c_path = string_concat(stem, (long long)"_compiled.c");
    ok = write_file_content((char*)c_path, (char*)c_code);
    printf("%s\n", (char*)(long long)"[3/3] Compiling and Linking via Clang...");
    cmd_half1 = string_concat((long long)"clang ", c_path);
    cmd_half2 = string_concat(cmd_half1, (long long)" -o ");
    compile_cmd = string_concat(cmd_half2, stem);
    status = run_command((char*)compile_cmd);
    if ((status == 0)) {
    printf("%s\n", (char*)(long long)"Self-hosted compilation successful!");
    ret_val = 0;
    goto L_cleanup;
    } else {
    printf("%s\n", (char*)(long long)"Compilation failed.");
    ret_val = 1;
    goto L_cleanup;
    }
L_cleanup:
    free_list(all_functions);
    free_list(parsed_files);
    free_list(f_names);
    free_list(empty_imports);
    free_list(program_ast);
    return ret_val;
}


/* Bootstrapper C main */
int main(int argc, char** argv) {
    init_ep_args(argc, argv);
    _main();
    return 0;
}
