import torch
import torch.nn as nn
import onnx

class DummyMobileNet(nn.Module):
    def __init__(self):
        super(DummyMobileNet, self).__init__()
        # Membuat parameter dummy
        self.dummy_param = nn.Parameter(torch.ones(1))

    def forward(self, x):
        # Asumsi x adalah fitur gambar yang sudah diekstrak: tensor [1, 10]
        batch_size = x.shape[0]
        
        # Output dummy logit probabilitas kelas [Cat(0), Dog(1), Bird(2)]
        # Misal selalu memprediksi Dog dengan prob terbesar (0.85)
        # Kelas Cat = 0.05, Bird = 0.10
        logits = torch.tensor([[0.05, 0.85, 0.10]])
        
        return logits.expand(batch_size, -1)

model = DummyMobileNet()
model.eval()

# Dummy input, shape: 1 batch, 10 features (simulasi image feature extracted)
x = torch.randn(1, 10)

# Export to ONNX
torch.onnx.export(
    model, 
    (x,), 
    "examples/dummy_mobilenet.onnx", 
    input_names=["image_features"], 
    output_names=["class_probs"],
    dynamic_axes={'image_features': {0: 'batch_size'}, 'class_probs': {0: 'batch_size'}}
)

print("Berhasil membuat examples/dummy_mobilenet.onnx")
