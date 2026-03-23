a very simple C wrapper for the [zeekstd](https://github.com/rorosen/zeekstd) crate, albeit currently only with support for writing.

```c
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
/// `encoder` will no longer be valid after this, so like, SET IT TO NULL
/// DAMMIT.
uint64_t zs_finish(void *encoder);

/// Returns a string description of the last error (if there is one)
const char *zs_last_error();
```

licensed under [MIT](LICENSE-MIT.md) or [Apache-2.0](LICENSE-Apache.md)
