import torch
import torch.nn as nn
import onnx

class DummyYolo(nn.Module):
    def __init__(self):
        super(DummyYolo, self).__init__()
        # Hanya parameter dummy agar tidak kosong
        self.dummy_param = nn.Parameter(torch.ones(1))

    def forward(self, x):
        # x diasumsikan sebagai tensor [1, 10]
        batch_size = x.shape[0]
        boxes = torch.tensor([[320.0, 320.0, 100.0, 100.0, 0.95, 0.0]])
        return boxes.expand(batch_size, -1, -1)

model = DummyYolo()
model.eval()

# Dummy input
x = torch.randn(1, 10)

# Export to ONNX
torch.onnx.export(
    model, 
    (x,), 
    "dummy_yolo.onnx", 
    input_names=["image_input"], 
    output_names=["boxes_output"],
    dynamic_axes={'image_input': {0: 'batch_size'}, 'boxes_output': {0: 'batch_size'}}
)

print("Berhasil membuat dummy_yolo.onnx")
