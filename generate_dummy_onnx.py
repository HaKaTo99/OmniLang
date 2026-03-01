import onnx
from onnx import helper, TensorProto

# X input array of length 2
X = helper.make_tensor_value_info('X', TensorProto.FLOAT, [1, 2])

# Y output array of length 2
Y = helper.make_tensor_value_info('Y', TensorProto.FLOAT, [1, 2])

# Constanta = 2.0
multiplier = helper.make_tensor('multiplier', TensorProto.FLOAT, [1], [2.0])

# Fungsi: Y = X * 2.0
mul_node = helper.make_node(
    'Mul', 
    ['X', 'multiplier'], 
    ['Y'], 
)

graph_def = helper.make_graph(
    [mul_node],
    'dummy-multiply',
    [X],
    [Y],
    [multiplier]
)

model_def = helper.make_model(graph_def, producer_name='omnilang-oracle-poc')
onnx.save(model_def, 'examples/multiply_by_two.onnx')
print("Model saved to examples/multiply_by_two.onnx")
