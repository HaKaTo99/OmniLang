import onnx
from onnx import helper, TensorProto

def main():
    # Input tensor [1, 5]
    X = helper.make_tensor_value_info('input', TensorProto.FLOAT, [1, 5])
    
    # Output tensor [1, 2]
    Y = helper.make_tensor_value_info('output', TensorProto.FLOAT, [1, 2])
    
    # Create a constant tensor that will just return [0.95, 1.0] (High Anomaly, High Severity)
    const_tensor = helper.make_tensor('const_tensor', TensorProto.FLOAT, [1, 2], [0.95, 1.0])
    
    # Node that outputs the constant tensor
    const_node = helper.make_node(
        'Constant',
        inputs=[],
        outputs=['output'],
        value=const_tensor
    )
    
    # Create the graph containing the node
    graph_def = helper.make_graph(
        [const_node],
        'dummy-anomaly-detector',
        [X],   # Graph inputs
        [Y]    # Graph outputs
    )
    
    # Create the model
    model_def = helper.make_model(graph_def, producer_name='omnilang-oracle-poc')
    
    # Save the model
    onnx_path = 'examples/ooda_loop/anomaly_detector.onnx'
    onnx.save(model_def, onnx_path)
    print(f"Model saved to {onnx_path}")

if __name__ == "__main__":
    main()
