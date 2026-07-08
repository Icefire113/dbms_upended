# Query Language

The query language is designed to be somewhat similar to SQL.

## Selecting

```sql
select /* Comma seperated list of columns to select, or a wildcard */
from   /* [table_name] | [table_1_name table_1_alias], [table_2_name table_2_alias], ... */
where  /* Optional where condition */

;
```

## Creating

### Tables

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

### Databases

Databases can be created like so:

```sql
create database database_name;
```

### Indexes

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
