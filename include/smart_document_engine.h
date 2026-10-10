#ifndef SMART_DOCUMENT_ENGINE_H
#define SMART_DOCUMENT_ENGINE_H
#include <stdint.h>
#ifdef __cplusplus
extern "C" {
#endif
uint32_t sde_abi_version(void);
/* One-based inclusive range. 0=valid, 1=empty document, 2=invalid range. */
uint32_t sde_validate_extraction(uint32_t page_count, uint32_t start, uint32_t end);
#ifdef __cplusplus
}
#endif
#endif
