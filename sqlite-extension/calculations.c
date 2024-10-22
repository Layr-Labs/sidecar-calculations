#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include "sidecar-calculations.h"
#include "calculations.h"
SQLITE_EXTENSION_INIT1

void add_big_sqlite(sqlite3_context *context, int argc, sqlite3_value **argv) {
    if (argc != 2) {
        sqlite3_result_error(context, "add_big() requires two arguments", -1);
        return;
    }
    const char* a = (const char*)sqlite3_value_text(argv[0]);
    if (!a) {
        sqlite3_result_null(context);
        return;
    }

    const char* b = (const char*)sqlite3_value_text(argv[1]);
    if (!b) {
        sqlite3_result_null(context);
        return;
    }

    char* sum = add_big_c(a, b);
    if (!sum) {
        sqlite3_result_null(context);
        return;
    }

    sqlite3_result_text(context, sum, -1, SQLITE_TRANSIENT);
}
void subtract_big_sqlite(sqlite3_context *context, int argc, sqlite3_value **argv) {
    if (argc != 2) {
        sqlite3_result_error(context, "subtract_big() requires two arguments", -1);
        return;
    }
    const char* a = (const char*)sqlite3_value_text(argv[0]);
    if (!a) {
        sqlite3_result_null(context);
        return;
    }

    const char* b = (const char*)sqlite3_value_text(argv[1]);
    if (!b) {
        sqlite3_result_null(context);
        return;
    }

    char* diff = subtract_big_c(a, b);
    if (!diff) {
        sqlite3_result_null(context);
        return;
    }

    sqlite3_result_text(context, diff, -1, SQLITE_TRANSIENT);
}
void big_gt_sqlite(sqlite3_context *context, int argc, sqlite3_value **argv) {
    if (argc != 2) {
        sqlite3_result_error(context, "big_gt() requires two arguments", -1);
        return;
    }
    const char* a = (const char*)sqlite3_value_text(argv[0]);
    if (!a) {
        sqlite3_result_null(context);
        return;
    }

    const char* b = (const char*)sqlite3_value_text(argv[1]);
    if (!b) {
        sqlite3_result_null(context);
        return;
    }

    int gt = big_gt_c(a, b);
    sqlite3_result_int(context, gt);
}
void numeric_multiply_sqlite(sqlite3_context *context, int argc, sqlite3_value **argv) {
    if (argc != 2) {
        sqlite3_result_error(context, "numeric_multiply() requires two arguments", -1);
        return;
    }
    const char* a = (const char*)sqlite3_value_text(argv[0]);
    if (!a) {
        sqlite3_result_null(context);
        return;
    }

    const char* b = (const char*)sqlite3_value_text(argv[1]);
    if (!b) {
        sqlite3_result_null(context);
        return;
    }

    char* product = numeric_multiply_c(a, b);
    if (!product) {
        sqlite3_result_null(context);
        return;
    }

    sqlite3_result_text(context, product, -1, SQLITE_TRANSIENT);
}

typedef struct SumContext {
    char* current_sum;
} SumContext;

static void sum_big_step(sqlite3_context* context, int argc, sqlite3_value** argv) {
    SumContext* ctx = (SumContext*)sqlite3_aggregate_context(context, sizeof(SumContext));

    if (argc != 1) {
        sqlite3_result_error(context, "sum_big() requires one argument", -1);
        return;
    }

    const char* value = (const char*)sqlite3_value_text(argv[0]);
    if (!value) {
        return; // Skip NULL values
    }

    if (!ctx->current_sum) {
        ctx->current_sum = strdup(value);
    } else {
        sqlite3 *db = sqlite3_context_db_handle(context);
        char* new_sum = add_big_c(ctx->current_sum, value);
        free(ctx->current_sum);
        ctx->current_sum = new_sum;
    }
}

static void sum_big_finalize(sqlite3_context* context) {
    SumContext* ctx = (SumContext*)sqlite3_aggregate_context(context, sizeof(SumContext));

    if (ctx && ctx->current_sum) {
        sqlite3_result_text(context, ctx->current_sum, -1, SQLITE_TRANSIENT);
        free(ctx->current_sum);
    } else {
        sqlite3_result_null(context);
    }
}

// handle removing values that leave the window frame
static void sum_big_inverse(sqlite3_context* context, int argc, sqlite3_value** argv) {
    SumContext* ctx = (SumContext*)sqlite3_aggregate_context(context, sizeof(SumContext));

    if (!ctx || !ctx->current_sum) {
        return;
    }

    const char* value = (const char*)sqlite3_value_text(argv[0]);
    if (!value) {
        return; // Skip NULL values
    }

    // Subtract the value that's leaving the window
    char* new_sum = subtract_big_c(ctx->current_sum, value);
    free(ctx->current_sum);
    ctx->current_sum = new_sum;
}

static void sum_big_value(sqlite3_context* context) {
    SumContext* ctx = (SumContext*)sqlite3_aggregate_context(context, sizeof(SumContext));

    if (ctx && ctx->current_sum) {
        sqlite3_result_text(context, ctx->current_sum, -1, SQLITE_TRANSIENT);
    } else {
        sqlite3_result_null(context);
    }
}

void amazon_staker_token_rewards_sqlite(sqlite3_context *context, int argc, sqlite3_value **argv) {
    if (argc != 2) {
        sqlite3_result_error(context, "amazon_staker_token_rewards() requires two arguments", -1);
        return;
    }
    const char* sp = (const char*)sqlite3_value_text(argv[0]);
    if (!sp) {
        sqlite3_result_null(context);
        return;
    }

    const char* tpd = (const char*)sqlite3_value_text(argv[1]);
    if (!tpd) {
        sqlite3_result_null(context);
        return;
    }

    sqlite3 *db = sqlite3_context_db_handle(context);
    char* tokens = amazon_staker_token_rewards_c(sp, tpd);
    if (!tokens) {
        sqlite3_result_null(context);
        return;
    }

    sqlite3_result_text(context, tokens, -1, SQLITE_TRANSIENT);
}
void amazon_operator_token_rewards_sqlite(sqlite3_context *context, int argc, sqlite3_value **argv) {
    if (argc != 1) {
        sqlite3_result_error(context, "amazon_operator_token_rewards() requires one argument", -1);
        return;
    }
    const char* tsot = (const char*)sqlite3_value_text(argv[0]);
    if (!tsot) {
        sqlite3_result_null(context);
        return;
    }

    char* tokens = amazon_operator_token_rewards_c(tsot);
    if (!tokens) {
        sqlite3_result_null(context);
        return;
    }

    sqlite3_result_text(context, tokens, -1, SQLITE_TRANSIENT);
}


void nile_staker_token_rewards_sqlite(sqlite3_context *context, int argc, sqlite3_value **argv) {
    if (argc != 2) {
        sqlite3_result_error(context, "amazon_staker_token_rewards() requires two arguments", -1);
        return;
    }
    const char* sp = (const char*)sqlite3_value_text(argv[0]);
    if (!sp) {
        sqlite3_result_null(context);
        return;
    }

    const char* tpd = (const char*)sqlite3_value_text(argv[1]);
    if (!tpd) {
        sqlite3_result_null(context);
        return;
    }

    sqlite3 *db = sqlite3_context_db_handle(context);
    char* tokens = nile_staker_token_rewards_c(sp, tpd);
    if (!tokens) {
        sqlite3_result_null(context);
        return;
    }

    sqlite3_result_text(context, tokens, -1, SQLITE_TRANSIENT);
}
void nile_operator_token_rewards_sqlite(sqlite3_context *context, int argc, sqlite3_value **argv) {
    if (argc != 1) {
        sqlite3_result_error(context, "amazon_operator_token_rewards() requires one argument", -1);
        return;
    }
    const char* tsot = (const char*)sqlite3_value_text(argv[0]);
    if (!tsot) {
        sqlite3_result_null(context);
        return;
    }

    char* tokens = nile_operator_token_rewards_c(tsot);
    if (!tokens) {
        sqlite3_result_null(context);
        return;
    }

    sqlite3_result_text(context, tokens, -1, SQLITE_TRANSIENT);
}


void staker_token_rewards_sqlite(sqlite3_context *context, int argc, sqlite3_value **argv) {
    if (argc != 2) {
        sqlite3_result_error(context, "staker_token_rewards() requires two arguments", -1);
        return;
    }
    const char* sp = (const char*)sqlite3_value_text(argv[0]);
    if (!sp) {
        sqlite3_result_null(context);
        return;
    }

    const char* tpd = (const char*)sqlite3_value_text(argv[1]);
    if (!tpd) {
        sqlite3_result_null(context);
        return;
    }

    sqlite3 *db = sqlite3_context_db_handle(context);
    char* tokens = staker_token_rewards_c(sp, tpd);
    if (!tokens) {
        sqlite3_result_null(context);
        return;
    }

    sqlite3_result_text(context, tokens, -1, SQLITE_TRANSIENT);
}
void operator_token_rewards_sqlite(sqlite3_context *context, int argc, sqlite3_value **argv) {
    if (argc != 1) {
        sqlite3_result_error(context, "operator_token_rewards() requires one argument", -1);
        return;
    }
    const char* tsot = (const char*)sqlite3_value_text(argv[0]);
    if (!tsot) {
        sqlite3_result_null(context);
        return;
    }

    char* tokens = operator_token_rewards_c(tsot);
    if (!tokens) {
        sqlite3_result_null(context);
        return;
    }

    sqlite3_result_text(context, tokens, -1, SQLITE_TRANSIENT);
}

void staker_weight_sqlite(sqlite3_context *context, int argc, sqlite3_value **argv) {
    if (argc != 2) {
        sqlite3_result_error(context, "staker_weight() requires two arguments", -1);
        return;
    }
    const char* multiplier = (const char*)sqlite3_value_text(argv[0]);
    if (!multiplier) {
        sqlite3_result_null(context);
        return;
    }

    const char* shares = (const char*)sqlite3_value_text(argv[1]);
    if (!shares) {
        sqlite3_result_null(context);
        return;
    }

    char* weight = staker_weight_c(multiplier, shares);
    if (!weight) {
        sqlite3_result_null(context);
        return;
    }

    sqlite3_result_text(context, weight, -1, SQLITE_TRANSIENT);
}
void staker_proportion_sqlite(sqlite3_context *context, int argc, sqlite3_value **argv) {
    if (argc != 2) {
        sqlite3_result_error(context, "staker_proportion() requires two arguments", -1);
        return;
    }
    const char* staker_weight = (const char*)sqlite3_value_text(argv[0]);
    if (!staker_weight) {
        sqlite3_result_null(context);
        return;
    }

    const char* total_staker_weight = (const char*)sqlite3_value_text(argv[1]);
    if (!total_staker_weight) {
        sqlite3_result_null(context);
        return;
    }

    char* proportion = staker_proportion_c(staker_weight, total_staker_weight);
    if (!proportion) {
        sqlite3_result_null(context);
        return;
    }

    sqlite3_result_text(context, proportion, -1, SQLITE_TRANSIENT);
}

void tokens_per_day_sqlite(sqlite3_context *context, int argc, sqlite3_value **argv) {
    if (argc != 2) {
        sqlite3_result_error(context, "tokens_per_day() requires two arguments", -1);
        return;
    }
    const char* a = (const char*)sqlite3_value_text(argv[0]);
    if (!a) {
        sqlite3_result_null(context);
        return;
    }

    const char* b = (const char*)sqlite3_value_text(argv[1]);
    if (!b) {
        sqlite3_result_null(context);
        return;
    }

    char* tpd = tokens_per_day_c(a, b);
    if (!tpd) {
        sqlite3_result_null(context);
        return;
    }

    sqlite3_result_text(context, tpd, -1, SQLITE_TRANSIENT);
}
void tokens_per_day_decimal(sqlite3_context *context, int argc, sqlite3_value **argv) {
    if (argc != 2) {
        sqlite3_result_error(context, "tokens_per_day_decimal() requires two arguments", -1);
        return;
    }
    const char* a = (const char*)sqlite3_value_text(argv[0]);
    if (!a) {
        sqlite3_result_null(context);
        return;
    }

    const char* b = (const char*)sqlite3_value_text(argv[1]);
    if (!b) {
        sqlite3_result_null(context);
        return;
    }

    char* tpd = tokens_per_day_decimal_c(a, b);
    if (!tpd) {
        sqlite3_result_null(context);
        return;
    }

    sqlite3_result_text(context, tpd, -1, SQLITE_TRANSIENT);
}


int sqlite3_calculations_init(sqlite3 *db, char **pzErrMsg, const sqlite3_api_routines *pApi) {
    SQLITE_EXTENSION_INIT2(pApi);

    int rc;
    // Generic math functions
    rc = sqlite3_create_function(db, "add_big", 2, SQLITE_UTF8 | SQLITE_DETERMINISTIC, 0, add_big_sqlite, 0, 0);
    if (rc != SQLITE_OK) {
        *pzErrMsg = sqlite3_mprintf("Failed to create function: %s", sqlite3_errmsg(db));
        return rc;
    }

    rc = sqlite3_create_function(db, "subtract_big", 2, SQLITE_UTF8 | SQLITE_DETERMINISTIC, 0, subtract_big_sqlite, 0, 0);
    if (rc != SQLITE_OK) {
        *pzErrMsg = sqlite3_mprintf("Failed to create function: %s", sqlite3_errmsg(db));
        return rc;
    }

    rc = sqlite3_create_function(db, "big_gt", 2, SQLITE_DETERMINISTIC, 0, big_gt_sqlite, 0, 0);
    if (rc != SQLITE_OK) {
        *pzErrMsg = sqlite3_mprintf("Failed to create function: %s", sqlite3_errmsg(db));
        return rc;
    }

    rc = sqlite3_create_function(db, "numeric_multiply", 2, SQLITE_UTF8 | SQLITE_DETERMINISTIC, 0, numeric_multiply_sqlite, 0, 0);
    if (rc != SQLITE_OK) {
        *pzErrMsg = sqlite3_mprintf("Failed to create function: %s", sqlite3_errmsg(db));
        return rc;
    }

    rc = sqlite3_create_window_function(db, "sum_big", 1, SQLITE_UTF8 | SQLITE_DETERMINISTIC, NULL, sum_big_step, sum_big_finalize, sum_big_value, sum_big_inverse, NULL);
    if (rc != SQLITE_OK) {
        *pzErrMsg = sqlite3_mprintf("Failed to create function: %s", sqlite3_errmsg(db));
        return rc;
    }

    // Amazon fork calculations
    rc = sqlite3_create_function(db, "amazon_staker_token_rewards", 2, SQLITE_UTF8 | SQLITE_DETERMINISTIC, 0, amazon_staker_token_rewards_sqlite, 0, 0);
    if (rc != SQLITE_OK) {
        *pzErrMsg = sqlite3_mprintf("Failed to create function: %s", sqlite3_errmsg(db));
        return rc;
    }
    rc = sqlite3_create_function(db, "amazon_operator_token_rewards", 1, SQLITE_UTF8 | SQLITE_DETERMINISTIC, 0, amazon_operator_token_rewards_sqlite, 0, 0);
    if (rc != SQLITE_OK) {
        *pzErrMsg = sqlite3_mprintf("Failed to create function: %s", sqlite3_errmsg(db));
        return rc;
    }

    // Nile fork calculations
    rc = sqlite3_create_function(db, "nile_staker_token_rewards", 2, SQLITE_UTF8 | SQLITE_DETERMINISTIC, 0, nile_staker_token_rewards_sqlite, 0, 0);
    if (rc != SQLITE_OK) {
        *pzErrMsg = sqlite3_mprintf("Failed to create function: %s", sqlite3_errmsg(db));
        return rc;
    }
    rc = sqlite3_create_function(db, "nile_operator_token_rewards", 1, SQLITE_UTF8 | SQLITE_DETERMINISTIC, 0, nile_operator_token_rewards_sqlite, 0, 0);
    if (rc != SQLITE_OK) {
        *pzErrMsg = sqlite3_mprintf("Failed to create function: %s", sqlite3_errmsg(db));
        return rc;
    }

    // Post-nile fork calculations
    rc = sqlite3_create_function(db, "staker_token_rewards", 2, SQLITE_UTF8 | SQLITE_DETERMINISTIC, 0, staker_token_rewards_sqlite, 0, 0);
    if (rc != SQLITE_OK) {
        *pzErrMsg = sqlite3_mprintf("Failed to create function: %s", sqlite3_errmsg(db));
        return rc;
    }
    rc = sqlite3_create_function(db, "operator_token_rewards", 1, SQLITE_UTF8 | SQLITE_DETERMINISTIC, 0, operator_token_rewards_sqlite, 0, 0);
    if (rc != SQLITE_OK) {
        *pzErrMsg = sqlite3_mprintf("Failed to create function: %s", sqlite3_errmsg(db));
        return rc;
    }

    rc = sqlite3_create_function(db, "staker_weight", 2, SQLITE_UTF8 | SQLITE_DETERMINISTIC, 0, staker_weight_sqlite, 0, 0);
    if (rc != SQLITE_OK) {
        *pzErrMsg = sqlite3_mprintf("Failed to create function: %s", sqlite3_errmsg(db));
        return rc;
    }

    rc = sqlite3_create_function(db, "staker_proportion", 2, SQLITE_UTF8 | SQLITE_DETERMINISTIC, 0, staker_proportion_sqlite, 0, 0);
    if (rc != SQLITE_OK) {
        *pzErrMsg = sqlite3_mprintf("Failed to create function: %s", sqlite3_errmsg(db));
        return rc;
    }

    rc = sqlite3_create_function(db, "tokens_per_day", 2, SQLITE_UTF8 | SQLITE_DETERMINISTIC, 0, tokens_per_day_sqlite, 0, 0);
    if (rc != SQLITE_OK) {
        *pzErrMsg = sqlite3_mprintf("Failed to create function: %s", sqlite3_errmsg(db));
        return rc;
    }

    rc = sqlite3_create_function(db, "tokens_per_day_decimal", 2, SQLITE_UTF8 | SQLITE_DETERMINISTIC, 0, tokens_per_day_decimal, 0, 0);
    if (rc != SQLITE_OK) {
        *pzErrMsg = sqlite3_mprintf("Failed to create function: %s", sqlite3_errmsg(db));
        return rc;
    }

    return SQLITE_OK;
}
