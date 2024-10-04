data modify storage dpscript:types/arrays/add_all temp_output append from storage dpscript:types/arrays/add_all temp_array[0]
data remove storage dpscript:types/arrays/add_all temp_array[0]

scoreboard players add __dpscript_temp __dps_add_all_array_pos_cur 1
execute if score __dpscript_temp __dps_add_all_array_pos_cur < __dpscript_temp __dps_add_all_array_length run function dpscript:__builtins/arrays/add_all_sub.pseudocode
