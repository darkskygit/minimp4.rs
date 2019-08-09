#define MINIMP4_IMPLEMENTATION
#include <sys/types.h>
#include "minimp4/minimp4.h"

static size_t get_nal_size(uint8_t *buf, size_t size)
{
    size_t pos = 3;
    while ((size - pos) > 3)
    {
        if (buf[pos] == 0 && buf[pos + 1] == 0 && buf[pos + 2] == 1)
            return pos;
        if (buf[pos] == 0 && buf[pos + 1] == 0 && buf[pos + 2] == 0 && buf[pos + 3] == 1)
            return pos;
        pos++;
    }
    return size;
}

static void write_callback(int64_t offset, const void *buffer, size_t size, void *token)
{
    FILE *f = (FILE*)token;
    fseek(f, offset, SEEK_SET);
    fwrite(buffer, size, 1, f);
}

int write_mp4(char* output, int width, int height, int fps, const uint8_t *data, size_t data_size)
{
    FILE *fout = fopen(output, "wb");
    if (!fout)
    {
        printf("error: can't open output file\n");
        return 0;
    }

    int is_hevc = 0;
    int sequential_mode = 0;
    MP4E_mux_t *mux;
    mp4_h26x_writer_t mp4wr;
    mux = MP4E__open(sequential_mode, fout, write_callback);
    mp4_h26x_write_init(&mp4wr, mux, width, height, is_hevc);

    while (data_size > 0)
    {
        size_t nal_size = get_nal_size(data, data_size);
        if (!nal_size)
        {
            data += 1;
            data_size -= 1;
            continue;
        }

        mp4_h26x_write_nal(&mp4wr, data, nal_size, 90000/fps);
        data  += nal_size;
        data_size -= nal_size;
    }

    MP4E__close(mux);
    if (fout)
        fclose(fout);
}
