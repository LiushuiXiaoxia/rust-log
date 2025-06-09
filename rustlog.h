#ifndef RUSTLOGLIB_H
#define RUSTLOGLIB_H

#ifdef __cplusplus
extern "C" {
#endif

void log_init(const char* dir);
void log_write(const char* level, const char* tag, const char* message);

#ifdef __cplusplus
}
#endif

#endif // RUSTLOGLIB_H
