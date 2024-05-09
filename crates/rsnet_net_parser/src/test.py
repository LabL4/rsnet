import torch.nn as nn
import numpy as np

class SimpleMLP(nn.Module):
    def __init__(self):
        super().__init__()
        self.layers = nn.Sequential(
            nn.Flatten(),
            nn.Linear(28*28, 20, bias=False),
            nn.ReLU(),
            nn.Linear(20, 10, bias=False),
            # nn.ReLU(),
        )

    def forward(self, x):
        return self.layers(x)

    def nparams(self):
        npar = 0
        
        for p in model.parameters():
            npar += p.numel()
        
        return npar

# print(SimpleMLP().layers[1].bias)
# print(type(SimpleMLP().layers[1].bias))