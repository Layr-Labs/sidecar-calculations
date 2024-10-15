#ifndef CALCULATIONS_H
#define CALCULATIONS_H

#include <sqlite3ext.h>

// Generic math functions
void add_big_sqlite(sqlite3_context *context, int argc, sqlite3_value **argv);
void subtract_big_sqlite(sqlite3_context *context, int argc, sqlite3_value **argv);
void big_gt_sqlite(sqlite3_context *context, int argc, sqlite3_value **argv);

// Amazon fork calculations
void amazon_staker_token_rewards_sqlite(sqlite3_context *context, int argc, sqlite3_value **argv);
void amazon_operator_token_rewards_sqlite(sqlite3_context *context, int argc, sqlite3_value **argv);

// Nile fork calculations
void nile_staker_token_rewards_sqlite(sqlite3_context *context, int argc, sqlite3_value **argv);
void nile_operator_token_rewards_sqlite(sqlite3_context *context, int argc, sqlite3_value **argv);

// Post-nile fork calculations
void staker_token_rewards_sqlite(sqlite3_context *context, int argc, sqlite3_value **argv);
void operator_token_rewards_sqlite(sqlite3_context *context, int argc, sqlite3_value **argv);

void staker_weight_sqlite(sqlite3_context *context, int argc, sqlite3_value **argv);

void tokens_per_day_sqlite(sqlite3_context *context, int argc, sqlite3_value **argv);
void tokens_per_day_decimal_sqlite(sqlite3_context *context, int argc, sqlite3_value **argv);

#endif // CALCULATIONS_H
