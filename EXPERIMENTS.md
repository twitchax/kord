# Experiments with Machine Learning Models

This document contains experiments and findings related to the machine learning models used in the Kord project.  It will be used to collate the results of different trials to eventually arrive at a "good" model.  The implications of each type of run can be learned by looking at the code for the specific builf flags.

## Constants

All experiment performed with a 24 GB RTX 4090 GPU.  All experiments used the `wgpu` backend on the host machine, and were performed over the network.

For some reason, the output always says the number of epochs is `64`, but the actual number is `16` as specified in the command line.

# Experiment Pass 1

## Experiment: Note-Binned Convolution and Full Target

### Observations

GPU Utilization around 20%.

### Command

```bash
cargo run --bin kord --no-default-features --features "cli ml_train ml_remote ml_loader_note_binned_convolution ml_target_full" --release -- -q ml train --backend ws://192.168.229.200:3000 --training-sources kord/samples/captured/ --training-sources kord/samples/slakh --training-sources sim --model-epochs 16
```

### Output

```
======================== Learner Summary ========================
Model:
KordModel {
  mha: MultiHeadAttention {d_model: 128, n_heads: 16, d_k: 8, dropout: 0.5, min_float: -10000, quiet_softmax: false, params: 66048}
  output: Linear {d_input: 128, d_output: 128, bias: true, params: 16512}
  params: 82560
}
Total Epochs: 64


| Split | Metric                         | Min.   | Epoch | Max.   | Epoch |
| ----- | ------------------------------ | ------ | ----- | ------ | ----- |
| Train | Hamming Score @ Threshold(0.5) | 95.704 | 1     | 98.927 | 21    |
| Train | Loss                           | 0.032  | 53    | 0.106  | 1     |
| Valid | Hamming Score @ Threshold(0.5) | 95.829 | 1     | 98.562 | 29    |
| Valid | Loss                           | 0.043  | 29    | 0.096  | 1     |

Deterministic accuracy: 100%
Inference accuracy: 12.5%
```

## Experiment: Frequency Pooling and Full Target

### Observations

GPU Utilization around 60%.

### Command

```bash
cargo run --bin kord --no-default-features --features "cli ml_train ml_remote ml_loader_frequency_pooled ml_target_full" --release -- -q ml train --backend ws://192.168.229.200:3000 --training-sources kord/samples/captured/ --training-sources kord/samples/slakh --training-sources sim --model-epochs 16
```

### Output

```
======================== Learner Summary ========================
Model:
KordModel {
  mha: MultiHeadAttention {d_model: 2048, n_heads: 16, d_k: 128, dropout: 0.5, min_float: -10000, quiet_softmax: false, params: 16785408}
  output: Linear {d_input: 2048, d_output: 128, bias: true, params: 262272}
  params: 17047680
}
Total Epochs: 64


| Split | Metric                         | Min.   | Epoch | Max.   | Epoch |
| ----- | ------------------------------ | ------ | ----- | ------ | ----- |
| Train | Hamming Score @ Threshold(0.5) | 95.907 | 11    | 98.927 | 21    |
| Train | Loss                           | 0.032  | 53    | 0.391  | 11    |
| Valid | Hamming Score @ Threshold(0.5) | 95.861 | 1     | 98.562 | 29    |
| Valid | Loss                           | 0.043  | 29    | 0.099  | 1     |

Deterministic accuracy: 100%
Inference accuracy: 7.9545455%
```

## Experiment: Note-Binned Convolution and Folded Target

### Observations

GPU Utilization around 15%.

### Command

```bash
cargo run --bin kord --no-default-features --features "cli ml_train ml_remote ml_loader_note_binned_convolution ml_target_folded" --release -- -q ml train --backend ws://192.168.229.200:3000 --training-sources kord/samples/captured/ --training-sources kord/samples/slakh --training-sources sim --model-epochs 16
```

### Output

```
======================== Learner Summary ========================
Model:
KordModel {
  mha: MultiHeadAttention {d_model: 128, n_heads: 16, d_k: 8, dropout: 0.5, min_float: -10000, quiet_softmax: false, params: 66048}
  output: Linear {d_input: 128, d_output: 12, bias: true, params: 1548}
  params: 67596
}
Total Epochs: 64


| Split | Metric                         | Min.   | Epoch | Max.   | Epoch |
| ----- | ------------------------------ | ------ | ----- | ------ | ----- |
| Train | Hamming Score @ Threshold(0.5) | 90.464 | 1     | 98.927 | 21    |
| Train | Loss                           | 0.032  | 53    | 0.231  | 1     |
| Valid | Hamming Score @ Threshold(0.5) | 90.524 | 14    | 98.562 | 29    |
| Valid | Loss                           | 0.043  | 29    | 0.223  | 3     |

Deterministic accuracy: 100%
Inference accuracy: 77.840904%
```

## Experiment: Frequency Pooling and Folded Target

### Observations

GPU Utilization around 60%.

### Command

```bash
cargo run --bin kord --no-default-features --features "cli ml_train ml_remote ml_loader_frequency_pooled ml_target_folded" --release -- -q ml train --backend ws://192.168.229.200:3000 --training-sources kord/samples/captured/ --training-sources kord/samples/slakh --training-sources sim --model-epochs 16
```

### Output

```
======================== Learner Summary ========================
Model:
KordModel {
  mha: MultiHeadAttention {d_model: 2048, n_heads: 16, d_k: 128, dropout: 0.5, min_float: -10000, quiet_softmax: false, params: 16785408}
  output: Linear {d_input: 2048, d_output: 12, bias: true, params: 24588}
  params: 16809996
}
Total Epochs: 64


| Split | Metric                         | Min.   | Epoch | Max.   | Epoch |
| ----- | ------------------------------ | ------ | ----- | ------ | ----- |
| Train | Hamming Score @ Threshold(0.5) | 90.686 | 1     | 98.927 | 21    |
| Train | Loss                           | 0.032  | 53    | 0.359  | 6     |
| Valid | Hamming Score @ Threshold(0.5) | 90.806 | 16    | 98.562 | 29    |
| Valid | Loss                           | 0.043  | 29    | 0.238  | 10    |

Deterministic accuracy: 100%
Inference accuracy: 76.13636%
```

# Experiment Pass 2

These all have more metrics.

## Experiment: Note-Binned Convolution and Full Target

### Command

```bash
cargo run --bin kord --no-default-features --features "cli ml_train ml_remote ml_loader_note_binned_convolution ml_target_full" --release -- -q ml train --backend ws://192.168.229.200:3000 --training-sources kord/samples/captured/ --training-sources kord/samples/slakh --training-sources sim --model-epochs 16
```

### Output

```
Deterministic accuracy: 100.00%
Inference accuracy: 11.36%
Macro accuracy (128 classes): 98.49%
Macro precision (128 classes): 19.80%
Macro recall (128 classes): 22.37%
Macro F1 (128 classes): 19.60%
Sample-wise F1: 66.22%
```

## Experiment: Frequency Pooling and Full Target

### Command

```bash
cargo run --bin kord --no-default-features --features "cli ml_train ml_remote ml_loader_frequency_pooled ml_target_full" --release -- -q ml train --backend ws://192.168.229.200:3000 --training-sources kord/samples/captured/ --training-sources kord/samples/slakh --training-sources sim --model-epochs 16
```

### Output

```
Deterministic accuracy: 100.00%
Inference accuracy: 2.84%
Macro accuracy (128 classes): 96.65%
Macro precision (128 classes): 14.50%
Macro recall (128 classes): 28.54%
Macro F1 (128 classes): 17.37%
Sample-wise F1: 50.40%
```

# Experiment Pass 3

These all have more PR-AUC metrics.

## Experiment: Note-Binned Convolution and Full Target

### Command

```bash
cargo run --bin kord --no-default-features --features "cli ml_train ml_remote ml_loader_note_binned_convolution ml_target_full" --release -- -q ml train --backend ws://192.168.229.200:3000 --training-sources kord/samples/captured/ --training-sources kord/samples/slakh --training-sources sim --model-epochs 4
```

### Output

```
Deterministic accuracy: 100.00%
Inference accuracy: 13.07%
Macro accuracy (128 classes): 98.56%
Macro precision (128 classes): 20.85%
Macro recall (128 classes): 20.91%
Macro F1 (128 classes): 19.20%
Macro PR AUC (128 classes): 25.30%
Sample-wise F1: 66.43%
```

# Experiment Pass 4

Added a bias to the final linear step.

## Experiment: Note-Binned Convolution and Full Target

### Command

```bash
cargo run --bin kord --no-default-features --features "cli ml_train ml_remote ml_loader_note_binned_convolution ml_target_full" --release -- -q ml train --backend ws://192.168.229.200:3000 --training-sources kord/samples/captured/ --training-sources kord/samples/slakh --training-sources sim --model-epochs 4
```

### Output

```
Deterministic accuracy: 100.00%
Inference accuracy: 7.39%
Macro accuracy (128 classes): 98.50%
Macro precision (128 classes): 19.76%
Macro recall (128 classes): 21.57%
Macro F1 (128 classes): 19.15%
Macro PR AUC (128 classes): 25.37%
Sample-wise F1: 66.01%
```

# Experiment Pass 5

Added threshold tuning for inference.

## Experiment: Note-Binned Convolution and Full Target

### Command

```bash
cargo run --bin kord --no-default-features --features "cli ml_train ml_remote ml_loader_note_binned_convolution ml_target_full" --release -- -q ml train --backend ws://192.168.229.200:3000 --training-sources kord/samples/captured/ --training-sources kord/samples/slakh --training-sources sim --model-epochs 4
```

### Output

```
Deterministic accuracy: 100.00%
Inference accuracy: 26.14%
Macro accuracy (128 classes): 99.12%
Macro precision (128 classes): 24.91%
Macro recall (128 classes): 27.82%
Macro F1 (128 classes): 25.49%
Macro PR AUC (128 classes): 25.09%
Sample-wise F1: 79.84%
```