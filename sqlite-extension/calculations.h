#ifndef CALCULATIONS_H
#define CALCULATIONS_H

#include <sqlite3ext.h>

void nile_staker_token_rewards_sqlite(sqlite3_context *context, int argc, sqlite3_value **argv);

#endif // CALCULATIONS_H
