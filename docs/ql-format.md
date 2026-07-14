# Query Language

The query language is designed to be somewhat similar to SQL.

## Dropping

You can drop a database, index or table like so:

```sql
drop database database_name;
drop index index_name;
drop table table_name;
```

## Selecting

With a where clause:

```sql
select /* Comma seperated list of columns to select, or a wildcard */
from   /* [table_name], ... */
where  /* Optional where condition */

;
```

Without a where clause:

```sql
select /* Comma seperated list of columns to select, or a wildcard */
from   /* [table_name], ... */
;
```

Note that aliases are not supported for tables

## Creating

### Creating Tables

```sql
create table table_name
(
    column_name column_type /* [column modifiers] */,
    column_name column_type /* [column modifiers] */,
    ...
);
```

#### Column Modifiers

Column modifiers are optional types that can be added to columns.

##### Nullable: `nullable`

By default, all columns do not permit null values, adding `nullable` to a column allows for the column to hold null values. Further, typically when inserting into a row, all columns must have valid values however for nullable columns, you may omit a value and the row will default to having the value `null`.

##### Indexed: `indexed index_name`

By default, columns are not indexed (but an index can be created via `create index`), if you wish to have a column be indexed by default, add `indexed {index_name}` to the list of column modifiers.

##### Unique: `unique`

By default, columns may contain duplicate values, adding `unique` forbids this behaviour. This check is made on insertion as well as when data is loaded from a file.

#### Columm Types

The 6 basic column types are as follows, more details can be found in the [list of supported types](supported-types.md#supported-types):

- `int`
- `bigint`
- `float`
- `bigfloat`
- `string`
- `boolean`

### Creating Databases

Databases can be created like so:

```sql
create database database_name;
```

### Creating Indexes

Indexes can be created like so:

```sql
create index index_name
on table_name.column_name;
```

Note that only one column can be indexed at a time.

## Loading

Data can be loaded from a file into a table like so:

```sql
load table_name
from 'file_path';
```

Note the single quotes around the file path, they are required.

Currently, only CSV files are supported, they should not contain a header row.

## Inserting

Insert with values:

```sql
insert into table_name
values (value_1, value_2, ...);
```

Insert only specific columns:

```sql
insert into table_name (column_1, column_2, ...)
values (value_1, value_2, ...);
```

Insert multiple rows:

```sql
insert into table_name
values (value_1, value_2, ...),
      (value_1, value_2, ...),
      ...
;
```

Note that if a column is not given a value, it will default to `null` if and only if the column is nullable, if it is not nullable, the insert will fail.

## Altering

### Altering Tables

#### Adding Columns

If you wish to add a column to a table, you can do so like so:

```sql
alter table table_name
add column column_name column_type /* [column modifiers] */;
```

#### Removing Columns

If you wish to remove a column from a table, you can do so like so:

```sql
alter table table_name
drop column column_name;
```

### Altering Columns

#### Adding Column Modifiers

If you wish to add a modifier to a column, you can do so like:

```sql
alter column table_name.column_name
add modifier modifier_name;
```

Note that you cannot add the `indexed` modifier to a column after it has been created, if you wish to add an index to a column, you must do so when creating the column or via [`create index`](#creating-indexes).

#### Removing Column Modifiers

If you with to remove a modifier from a column, you can do so like:

```sql
alter column table_name.column_name
drop modifier modifier_name;
```

Note that you cannot remove the `indexed` modifier from a column after it has been created, if you wish to remove an index from a column, you must do so via [`drop index`](#dropping).

## Deleting

Removes a row from a table if the where condition is evaluated to true

```sql
delete from table_name
where condition;
```

## Updating

Sets the value of some columns to some values if the where condition is evaluated to true

With a where condition:

```sql
update table_name
set column_1 = value_1,
    column_2 = value_2,
    ...
where condition;
```

Without a where condition:

```sql
update table_name
set column_1 = value_1,
    column_2 = value_2,
    ...
```
