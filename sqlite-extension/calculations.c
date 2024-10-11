#include <stdlib.h>
#include <stdio.h>
#include "sidecar-calculations.h"
#include "calculations.h"
SQLITE_EXTENSION_INIT1

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

int sqlite3_calculations_init(sqlite3 *db, char **pzErrMsg, const sqlite3_api_routines *pApi) {
    SQLITE_EXTENSION_INIT2(pApi);

    int rc;
    rc = sqlite3_create_function(db, "nile_staker_token_rewards", 2, SQLITE_UTF8 | SQLITE_DETERMINISTIC, 0, nile_staker_token_rewards_sqlite, 0, 0);
    if (rc != SQLITE_OK) {
        *pzErrMsg = sqlite3_mprintf("Failed to create function: %s", sqlite3_errmsg(db));
        return rc;
    }

    return SQLITE_OK;
}
