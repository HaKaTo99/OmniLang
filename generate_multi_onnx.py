import onnx
from onnx import helper, TensorProto

# Input A and B of length 2
A = helper.make_tensor_value_info('A', TensorProto.FLOAT, [1, 2])
B = helper.make_tensor_value_info('B', TensorProto.FLOAT, [1, 2])

# Outputs Sum and Diff of length 2
Sum = helper.make_tensor_value_info('Sum', TensorProto.FLOAT, [1, 2])
Diff = helper.make_tensor_value_info('Diff', TensorProto.FLOAT, [1, 2])

# Fungsi: Sum = A + B
add_node = helper.make_node(
    'Add', 
    ['A', 'B'], 
    ['Sum'], 
)

# Fungsi: Diff = A - B
sub_node = helper.make_node(
    'Sub', 
    ['A', 'B'], 
    ['Diff'], 
)

graph_def = helper.make_graph(
    [add_node, sub_node],
    'dummy-multi-io',
    [A, B],
    [Sum, Diff]
)

model_def = helper.make_model(graph_def, producer_name='omnilang-oracle-poc-multi')
onnx.save(model_def, 'examples/multi_io.onnx')
print("Model saved to examples/multi_io.onnx")
