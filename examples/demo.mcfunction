
data modify storage dpscript:builtin/stores/vars __operation_output set value {"text": "Hello, "}
data merge storage dpscript:builtin/stores/vars {"__operation_output": {"color": "blue"}}
data modify storage dpscript:builtin/stores/vars TEXT_PREFIX set from storage dpscript:builtin/stores/temp __operation_output
data modify storage dpscript:builtin/stores/vars PLAYER set value []
tellraw [] data modify storage dpscript:builtin/stores/vars set value []
data modify storage dpscript:builtin/stores/vars set from storage dpscript:builtin/stores/vars TEXT_PREFIX
data modify storage dpscript:builtin/stores/vars append from storage dpscript:builtin/stores/vars __array_item_0
data modify storage dpscript:builtin/stores/vars __operation_output set value {}
data merge storage dpscript:builtin/stores/vars {"__operation_output": {"color": "red"}}
data modify storage dpscript:builtin/stores/vars set from storage dpscript:builtin/stores/temp __operation_output
data modify storage dpscript:builtin/stores/vars append from storage dpscript:builtin/stores/vars __array_item_1
data modify storage dpscript:builtin/stores/vars append value {"text": "!"}
data modify storage dpscript:builtin/stores/vars set from storage dpscript:builtin/stores/temp __return_val
function my_pack:examples/demo.dps_say_something
data modify storage dpscript:builtin/stores/vars set from storage dpscript:builtin/stores/temp __return_val
data modify storage dpscript:builtin/stores/temp __op_lhs set value 0.1
data modify storage dpscript:builtin/stores/temp __op_rhs set value 0.2
data operation storage dpscript:builtin/stores/temp __op_lhs add dpscript:builtin/stores/temp __op_rhs
data modify storage dpscript:builtin/stores/vars __operation_output set from storage dpscript:builtin/stores/temp __op_lhs
data modify storage dpscript:builtin/stores/vars result set from storage dpscript:builtin/stores/temp __operation_output
tellraw [] data modify storage dpscript:builtin/stores/vars set value []
data modify storage dpscript:builtin/stores/vars append value {}
data modify storage dpscript:builtin/stores/vars set from storage dpscript:builtin/stores/temp __return_val
data modify storage dpscript:builtin/stores/vars __return_data set from storage dpscript:builtin/stores/vars result
data modify storage dpscript:builtin/stores/temp __return_val set from storage dpscript:builtin/stores/vars __return_data
function my_pack:examples/demo.dps_test_floats
data modify storage dpscript:builtin/stores/temp __return_val set from storage dpscript:builtin/stores/vars __return_val
data modify storage dpscript:builtin/stores/vars result set from storage dpscript:builtin/stores/temp __return_val
tellraw [] data modify storage dpscript:builtin/stores/vars set value []
data modify storage dpscript:builtin/stores/vars append value {}
data modify storage dpscript:builtin/stores/vars set from storage dpscript:builtin/stores/temp __return_val