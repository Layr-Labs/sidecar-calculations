// FFI for rust functions in sidecar-calculations
#ifndef SIDECAR_CALCULATIONS_H
#define SIDECAR_CALCULATIONS_H

#ifdef __cplusplus
extern "C" {
#endif

void free_c_string(char* s);

// Generic math functions
char* add_big_c(const char* a, const char* b);
char* subtract_big_c(const char* a, const char* b);
int big_gt_c(const char* a, const char* b);
char* numeric_multiply_c(const char* a, const char* b);

// Amazon fork calculations
char* amazon_staker_token_rewards_c(const char* sp, const char* tpd);
char* amazon_operator_token_rewards_c(const char* tsot);

// Nile fork calculations
char* nile_staker_token_rewards_c(const char* sp, const char* tpd);
char* nile_operator_token_rewards_c(const char* tsot);

// Post-nile fork calculations
char* staker_token_rewards_c(const char* sp, const char* tpd);
char* operator_token_rewards_c(const char* tsot);

char* staker_weight_c(const char* multiplier, const char* shares);
char* staker_proportion_c(const char* staker_weight, const char* total_staker_weight);

char* tokens_per_day_c(const char* a, const char* b);
char* tokens_per_day_decimal_c(const char* a, const char* b);

#ifdef __cplusplus
}
#endif

#endif // SIDECAR_CALCULATIONS_H
