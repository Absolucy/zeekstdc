#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#ifdef __cplusplus
extern "C" {
#endif

/// The value returned by `zs_finish` if it errored.
#define ZS_FINISH_ERRORED ((uint64_t)((int64_t)-1))

/// Creates a new seekable ZSTD file to write to, with the given compression
/// level.
///
/// If successful, it returns a pointer to the encoder, to be used as the
/// `encoder` arg in other functions.
///
/// Returns a null pointer if it errored, use
/// `zs_last_error` to get an error message.
void *zs_open_file(const char *file_name, int32_t compression_level);

/// Writes to the given encoder. Simple as that.
///
/// Returns `false` if it errored, use `zs_last_error` to get an error message.
bool zs_write(void *encoder, const uint8_t *data, uintptr_t len);

/// Flushes the data in the given encoder.
///
/// Returns `false` if it errored, use `zs_last_error` to get an error message.
bool zs_flush(void *encoder);

/// Finishes up the compressed file, writing everything to the disk.
///
/// If the return value is ZS_FINISH_ERRORED, then it errored.
///
/// `encoder` will no longer be valid after this, so like, SET IT TO NULL
/// DAMMIT.
uint64_t zs_finish(void *encoder);

/// Frees an encoder without properly finishing it up.
///
/// `encoder` will no longer be valid after this, so like, SET IT TO NULL
/// DAMMIT.
void zs_free(void *encoder);

/// Returns a string description of the last error.
///
/// If there is no last error, it will be a blank string - this will never
/// return a null pointer.
const char *zs_last_error();

#ifdef __cplusplus
}
#endif
