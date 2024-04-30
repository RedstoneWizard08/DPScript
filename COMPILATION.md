# Example

Here's an example of how something might compile.

## Input

```dps
#[load]
#[name = "_:load"]
function load() {
    // Pretend these aren't static values.
    let a = 1.0;
    let b = 2.0;
    
    if (a < b) {
        /tellraw(@a, [c#"A is less than B!" + nbt(component) { "color": "green" }]);
    } else {
        /tellraw(@a, [c#"A is larger than B!" + nbt(component) { "color": "red" }]);
    }
    
    // Pretend these aren't static values.
    let c = 1.0;
    let d = 2.0;
    
    // An example without an else block.
    if (c > d) {
        /tellraw(@a, [c#"C is larger than D!" + nbt(component) { "color": "green" }]);
    }
}
```

## Output

This might change in the future!

```json
# ========= data/minecraft/tags/functions/load.json =========

{"values": ["my_pack:load"]}
```

```mcfunction
# ========= data/my_pack/functions/load.mcfunction =========

# Variable ident format: `[namespace]_[function_path]_[var]`
data modify storage dpscript:builtin/stores/vars my_pack_load_a set value 1.0
data modify storage dpscript:builtin/stores/vars my_pack_load_b set value 2.0

data modify storage dpscript:builtin/stores/temp my_pack_load_a set from storage dpscript:builtin/stores/vars my_pack_load_a
data operation storage dpscript:builtin/stores/temp my_pack_load_a mul 100000000

data modify storage dpscript:builtin/stores/temp my_pack_load_b set from storage dpscript:builtin/stores/vars my_pack_load_b
data operation storage dpscript:builtin/stores/temp my_pack_load_b mul 100000000

execute store result score __temp0_lhs __dpscript_temp run data get storage dpscript:builtin/stores/temp my_pack_load_a
execute store result score __temp1_rhs __dpscript_temp run data get storage dpscript:builtin/stores/temp my_pack_load_b

# Function ident format: `[namespace]:__dpscript_gen/[function_path]/branches/[branch_index]/[true | false | join]`
execute if score __temp0_lhs __dpscript_temp < __temp1_rhs __dpscript_temp run function my_pack:__dpscript_gen/load/branches/0/true
execute unless score __temp0_lhs __dpscript_temp < __temp1_rhs __dpscript_temp run function my_pack:__dpscript_gen/load/branches/0/false

# ========= data/my_pack/functions/__dpscript_gen/load/branches/0/true.mcfunction =========

tellraw @a ["", {"text": "A is less than B!", "color": "green"}]

# Merge at the join
function my_pack:__dpscript_gen/load/branches/0/join

# ========= data/my_pack/functions/__dpscript_gen/load/branches/0/false.mcfunction =========

tellraw @a ["", {"text": "A is lerger than B!", "color": "red"}]
function my_pack:__dpscript_gen/load/branches/0/join

# ========= data/my_pack/functions/__dpscript_gen/load/branches/0/join.mcfunction =========
data modify storage dpscript:builtin/stores/vars my_pack_load_c set value 1.0
data modify storage dpscript:builtin/stores/vars my_pack_load_d set value 2.0

data modify storage dpscript:builtin/stores/temp my_pack_load_c set from storage dpscript:builtin/stores/vars my_pack_load_c
data operation storage dpscript:builtin/stores/temp my_pack_load_c mul 100000000

data modify storage dpscript:builtin/stores/temp my_pack_load_d set from storage dpscript:builtin/stores/vars my_pack_load_d
data operation storage dpscript:builtin/stores/temp my_pack_load_d mul 100000000

execute store result score __temp0_lhs __dpscript_temp run data get storage dpscript:builtin/stores/temp my_pack_load_c
execute store result score __temp1_rhs __dpscript_temp run data get storage dpscript:builtin/stores/temp my_pack_load_d

execute if score __temp0_lhs __dpscript_temp < __temp1_rhs __dpscript_temp run function my_pack:__dpscript_gen/load/branches/1/true
execute unless score __temp0_lhs __dpscript_temp < __temp1_rhs __dpscript_temp run function my_pack:__dpscript_gen/load/branches/1/false

# ========= data/my_pack/functions/__dpscript_gen/load/branches/1/true.mcfunction =========

tellraw @a ["", {"text": "C is larger than D!", "color": "green"}]
function my_pack:__dpscript_gen/load/branches/1/join

# ========= data/my_pack/functions/__dpscript_gen/load/branches/1/false.mcfunction =========

# Nothing happens in the else block, but the function exists because we need
# to merge afterwards somehow.

function my_pack:__dpscript_gen/load/branches/1/join

# ========= data/my_pack/functions/__dpscript_gen/load/branches/1/join.mcfunction =========

# Nothing happens after, but the function exists for ease of codegen.
# In a future version, I might get rid of these unnecessary functions.
```
