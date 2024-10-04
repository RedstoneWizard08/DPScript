# init
scoreboard objectives add __dps_add_all_array_length dummy
scoreboard objectives add __dps_add_all_array_pos_cur dummy
scoreboard objectives add __dps_add_all_input_len dummy
scoreboard objectives add __dps_add_all_output_len dummy

# body
execute store result score __dpscript_temp __dps_add_all_array_length run data get storage dpscript:types/arrays/length "[input_store]/[input_path]"
scoreboard players set __dpscript_temp __dps_add_all_array_pos_cur 0
data modify storage dpscript:types/arrays/add_all temp_array set from storage input_store "[input_path]"
data modify storage dpscript:types/arrays/add_all temp_output set from storage output_store "[output_path]"
function dpscript:__builtins/arrays/add_all_sub.pseudocode
data modify storage output_store "[output_path]" set from storage dpscript:types/arrays/add_all temp_output
scoreboard players reset __dpscript_temp __dps_add_all_array_length
scoreboard players reset __dpscript_temp __dps_add_all_array_pos_cur
execute store result score __dpscript_temp __dps_add_all_input_len run data get storage dpscript:types/arrays/length "[input_store]/[input_path]"
execute store result score __dpscript_temp __dps_add_all_output_len run data get storage dpscript:types/arrays/length "[output_store]/[output_path]"
scoreboard players operation __dpscript_temp __dps_add_all_output_len += __dpscript_temp __dps_add_all_input_len
# TODO: What is `scale`?
execute store result storage dpscript:types/arrays/length "[output_store]/[output_path]" int 0 run scoreboard players get __dpscript_temp __dps_add_all_output_len
scoreboard players reset __dpscript_temp __dps_add_all_input_len
scoreboard players reset __dpscript_temp __dps_add_all_output_len
