/* Builds minimp3 with the configuration mbop3 targets (Layer 3 only, no SIMD) and
 * exposes it under variant-prefixed names so i16 and f32 builds can be linked together. */
#include <stdlib.h>
#include <stddef.h>

#define CAT_(a, b) a##b
#define CAT(a, b) CAT_(a, b)
#define PREFIX(name) CAT(CAT(mbop3ref_, REF_VARIANT), CAT(_, name))

#define mp3dec_init PREFIX(mp3dec_init)
#define mp3dec_decode_frame PREFIX(mp3dec_decode_frame)
#define mp3dec_f32_to_s16 PREFIX(mp3dec_f32_to_s16)

#define MINIMP3_IMPLEMENTATION
#define MINIMP3_ONLY_MP3
#define MINIMP3_NO_SIMD
#include "minimp3.h"

void *PREFIX(new)(void)
{
    mp3dec_t *dec = calloc(1, sizeof(mp3dec_t));
    if (dec)
        mp3dec_init(dec);
    return dec;
}

void PREFIX(free)(void *dec) { free(dec); }

size_t PREFIX(decoder_size)(void) { return sizeof(mp3dec_t); }

size_t PREFIX(scratch_size)(void) { return sizeof(mp3dec_scratch_t); }

/* pcm may be NULL: minimp3 then parses the frame without decoding it. */
int PREFIX(decode)(void *dec, const unsigned char *mp3, int mp3_bytes, void *pcm, int info_out[6])
{
    /* minimp3 leaves info fields other than frame_bytes unset when no frame is found. */
    mp3dec_frame_info_t info = { 0 };
    int samples = mp3dec_decode_frame((mp3dec_t *)dec, mp3, mp3_bytes, (mp3d_sample_t *)pcm, &info);
    info_out[0] = info.frame_bytes;
    info_out[1] = info.frame_offset;
    info_out[2] = info.channels;
    info_out[3] = info.hz;
    info_out[4] = info.layer;
    info_out[5] = info.bitrate_kbps;
    return samples;
}
