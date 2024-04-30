# Commands

All of this code is written using DPScript's IR. It's *like* `mcfunction`,
but some stuff is still transpiled (like variables and loop unrolling).

- `#` notation: `#var` interprets the variable as its literal value (type).
- `@` notation: `@var@` interprets the variable as an identifier.
- `[]` notation: `[var]` returns a bool that is true if the variable exists.

## Constants

```mcfunction
const DPSCRIPT_VAR_STORE = "dpscript:builtin/stores/vars";
const DPSCRIPT_TEMP_STORE = "dpscript:builtin/stores/temp";
const DPSCRIPT_DUMMY_OBJECTIVE = "__dpscript_temp";
const DPSCRIPT_FLOAT_PRECISION_MUL = 100000000;
```

## Load

```mcfunction
#! file = data/dpscript/functions/load.mcfunction

# DPScript Load Script
# This is run when the datapack is loaded, and initializes DPScript's basic functions!

# Create objective
scoreboard objectives add @DPSCRIPT_DUMMY_OBJECTIVE@ dummy

# Create stores
data merge storage @DPSCRIPT_VAR_STORE@ {}
data merge storage @DPSCRIPT_TEMP_STORE@ {}
```

## Tags

```jsonc
// data/minecraft/tags/functions/load.json
{
    "values": [
        "dpscript:load"
    ]
}
```

## Builtins

```mcfunction
macro store_temp_floats(lhs, rhs) {
    /data modify storage @DPSCRIPT_TEMP_STORE@ @lhs@ set from storage @DPSCRIPT_VAR_STORE@ @lhs@;
    /data operation storage @DPSCRIPT_TEMP_STORE@ @lhs@ mul @DPSCRIPT_FLOAT_PRECISION_MUL@;

    /data modify storage @DPSCRIPT_TEMP_STORE@ @rhs@ set from storage @DPSCRIPT_VAR_STORE@ @rhs@;
    /data operation storage @DPSCRIPT_TEMP_STORE@ @rhs@ mul @DPSCRIPT_FLOAT_PRECISION_MUL@;

    /execute store result score __temp0_lhs @DPSCRIPT_DUMMY_OBJECTIVE@ run data get storage @DPSCRIPT_TEMP_STORE@ @lhs@;
    /execute store result score __temp1_rhs @DPSCRIPT_DUMMY_OBJECTIVE@ run data get storage @DPSCRIPT_TEMP_STORE@ @rhs@;
}
```

## Components

```mcfunction
# component(variable)
{ "storage": @DPSCRIPT_VAR_STORE@, "nbt": #variable, "interpret": true }

# component(entity)
{ "selector": #entity }
```

## Function Calls

```mcfunction
# Needs:
# - function: The transpiled (output) ID of the function to call.
# - function_args: An array of arguments with names and values (strings and nbt types)
# - return_var: Where to put the function's return value (if it has one). This is optional.
# - return_out: Where the function will put its return value. This is optional.

for arg in function_args {
    let name = arg.name;
    let value = arg.value;
    
    /data merge storage @DPSCRIPT_TEMP_STORE@ { "args": { @name@: #value } };
}

/function @function@;

if [return_var] && [return_out] {
    /data modify storage @DPSCRIPT_VAR_STORE@ @return_var@ set from storage @DPSCRIPT_TEMP_STORE@ @return_out@;
}
```

## Operations

```mcfunction
# ==

!!store_temp_floats(lhs, rhs);

/execute if score __temp0_lhs @DPSCRIPT_DUMMY_OBJECTIVE@ = __temp1_rhs @DPSCRIPT_DUMMY_OBJECTIVE@ run function @branch_true_fn@;
/execute unless score __temp0_lhs @DPSCRIPT_DUMMY_OBJECTIVE@ = __temp1_rhs @DPSCRIPT_DUMMY_OBJECTIVE@ run function @branch_false_fn@;

# !=

!!store_temp_floats(lhs, rhs);

/execute unless score __temp0_lhs @DPSCRIPT_DUMMY_OBJECTIVE@ = __temp1_rhs @DPSCRIPT_DUMMY_OBJECTIVE@ run function @branch_true_fn@;
/execute if score __temp0_lhs @DPSCRIPT_DUMMY_OBJECTIVE@ = __temp1_rhs @DPSCRIPT_DUMMY_OBJECTIVE@ run function @branch_false_fn@;

# >=

!!store_temp_floats(lhs, rhs);

/execute if score __temp0_lhs @DPSCRIPT_DUMMY_OBJECTIVE@ >= __temp1_rhs @DPSCRIPT_DUMMY_OBJECTIVE@ run function @branch_true_fn@;
/execute unless score __temp0_lhs @DPSCRIPT_DUMMY_OBJECTIVE@ >= __temp1_rhs @DPSCRIPT_DUMMY_OBJECTIVE@ run function @branch_false_fn@;

# >

!!store_temp_floats(lhs, rhs);

/execute if score __temp0_lhs @DPSCRIPT_DUMMY_OBJECTIVE@ > __temp1_rhs @DPSCRIPT_DUMMY_OBJECTIVE@ run function @branch_true_fn@;
/execute unless score __temp0_lhs @DPSCRIPT_DUMMY_OBJECTIVE@ > __temp1_rhs @DPSCRIPT_DUMMY_OBJECTIVE@ run function @branch_false_fn@;

# <=

!!store_temp_floats(lhs, rhs);

/execute if score __temp0_lhs @DPSCRIPT_DUMMY_OBJECTIVE@ >= __temp1_rhs @DPSCRIPT_DUMMY_OBJECTIVE@ run function @branch_true_fn@;
/execute unless score __temp0_lhs @DPSCRIPT_DUMMY_OBJECTIVE@ <= __temp1_rhs @DPSCRIPT_DUMMY_OBJECTIVE@ run function @branch_false_fn@;

# <

!!store_temp_floats(lhs, rhs);

/execute if score __temp0_lhs @DPSCRIPT_DUMMY_OBJECTIVE@ < __temp1_rhs @DPSCRIPT_DUMMY_OBJECTIVE@ run function @branch_true_fn@;
/execute unless score __temp0_lhs @DPSCRIPT_DUMMY_OBJECTIVE@ < __temp1_rhs @DPSCRIPT_DUMMY_OBJECTIVE@ run function @branch_false_fn@;

# +

/data operation storage @DPSCRIPT_VAR_STORE@ @lhs@ add #rhs;

# -

/data operation storage @DPSCRIPT_VAR_STORE@ @lhs@ sub #rhs;

# *

/data operation storage @DPSCRIPT_VAR_STORE@ @lhs@ mul #rhs;

# /

/data operation storage @DPSCRIPT_VAR_STORE@ @lhs@ div #rhs;

# **

/data operation storage @DPSCRIPT_VAR_STORE@ @lhs@ pow #rhs;

# &

/data operation storage @DPSCRIPT_VAR_STORE@ @lhs@ and #rhs;

# |

/data operation storage @DPSCRIPT_VAR_STORE@ @lhs@ or #rhs;

# ^

/data operation storage @DPSCRIPT_VAR_STORE@ @lhs@ xor #rhs;
```
