# init
scoreboard objectives add __array_length_temp dummy

# body
execute store result score __dpscript_temp __array_length_temp run data get storage dpscript:types/arrays/length "[output_store]/[output_path]"
data modify storage output_store "[output_path]" append from storage input_store "[input_path]"
scoreboard players add __dpscript_temp __array_length_temp 1
# TODO: What is `scale`?
execute store result storage dpscript:types/arrays/length "[output_store]/[output_path]" int 0 run scoreboard players get __dpscript_temp __array_length_temp