#ifndef SIDECAR_CALCULATIONS_H
#define SIDECAR_CALCULATIONS_H

#ifdef __cplusplus
extern "C" {
#endif

char* nile_staker_token_rewards_c(const char* sp, const char* tpd);
void free_c_string(char* s);

#ifdef __cplusplus
}
#endif

#endif // SIDECAR_CALCULATIONS_H
