import torch
from digit import Digit

model = Digit()
model.load_state_dict(torch.load("model/digit.pth", map_location="cpu"))

sample = torch.randn(1, 1, 8, 8)

trace_model = torch.jit.trace(model, sample)
trace_model.save("model/digit.jit")