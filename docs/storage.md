# Storage Overview

The dbms can be broken up into 1 main dbms which contains several databases
each which has several tables each with several columns.

On the file system this is represented as a directory which acts as the dbms
root, then within that directory, each folder represents a different database.

## Database storage

Each database is represented by a folder on disk, each folder must contain a
file called `db_fmt` which describes the schema of the database. This file has a
format documented [in the spec](format-spec#db_fmt-file-spec).

## Table storage

Within each database each table is also stored as a folder, the schema of the
table (and it's rows) are already described in the `db_fmt`. The actual row data
of the table is stored in sequential chunks labeled like so: `dta_{seq_id}`, and
the string pool is stored in sequential chunks labeled like so: `str_pool_{seq_id}`.
