#include <stdlib.h>
#include <stdio.h>
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

    return SQLITE_OK;
}
