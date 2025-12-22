# Experiments with Machine Learning Models

This document contains experiments and findings related to the machine learning models used in the Kord project.  It will be used to collate the results of different trials to eventually arrive at a "good" model.  The implications of each type of run can be learned by looking at the code for the specific builf flags.

Training output now prints a second metrics block that evaluates the trained model against the captured dataset (up to 1,000 samples) so that real recordings can be monitored separately from validation splits. Threshold tuning now falls back to 0.5 when a class lacks positives and clamps the learned cutoffs to the 0.05-0.95 band to avoid unusable extremes in `thresholds.json`. Captured samples are oversampled 16x during dataset construction (tunable via `--captured-oversample-factor`) so each epoch sees a heavier mix of real recordings alongside simulated material. Reports also surface per-class precision/recall highlights (including zero-support bins) to guide further data collection and balancing.

## Constants

All experiment performed with a 24 GB RTX 4090 GPU.  All experiments used the `wgpu` backend on the host machine, and were performed over the network.

For some reason, the output always says the number of epochs is `64`, but the actual number is `16` as specified in the command line.

## In-flight: Mel 1D Convolution (folded-bass target)

Tracking the new mel 1D convolution architecture (Conv5→Conv5→GELU trunk) that keeps the folded-bass target. Build/train with:

```bash
cargo run --bin kord --no-default-features --features "cli ml_train ml_tch ml_train_precision_fp32 ml_store_precision_full ml_loader_mel ml_target_folded_bass ml_model_mel_conv1d" --release -- -q ml train --backend ws://192.168.229.200:3000 --training-sources kord/samples/captured/ --training-sources kord/samples/slakh --training-sources sim --model-epochs 16
```

For inference, enable the same feature set (swap `ml_train`/`ml_tch` for `ml_infer` as needed) to keep weights/loader aligned.

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

# Experiment Pass 6

New baseline with thresholds.

## Experiment: Note-Binned Convolution and Full Target

### Command

```bash
cargo run --bin kord --no-default-features --features "cli ml_train ml_remote ml_loader_note_binned_convolution ml_target_full" --release -- -q ml train --backend ws://192.168.229.200:3000 --training-sources kord/samples/captured/ --training-sources kord/samples/slakh --training-sources sim --model-epochs 4
```

### Output

```
Validation dataset metrics (1000 samples):
Inference accuracy: 0.00%
Macro accuracy (128 classes): 90.26%
Macro precision (128 classes): 32.74%
Macro recall (128 classes): 46.29%
Macro F1 (128 classes): 34.65%
Macro PR AUC (128 classes): 33.31%
Sample-wise F1: 47.57%

Captured dataset metrics (176 samples):
Inference accuracy: 0.00%
Macro accuracy (128 classes): 91.76%
Macro precision (128 classes): 16.05%
Macro recall (128 classes): 26.94%
Macro F1 (128 classes): 18.96%
Macro PR AUC (128 classes): 25.72%
Sample-wise F1: 28.10%
```

## Experiment: Frequency Pooling and Full Target

### Command

```bash
cargo run --bin kord --no-default-features --features "cli ml_train ml_remote ml_loader_frequency_pooled ml_target_full" --release -- -q ml train --backend ws://192.168.229.200:3000 --training-sources kord/samples/captured/ --training-sources kord/samples/slakh --training-sources sim --model-epochs 4
```

### Output

```
Validation dataset metrics (1000 samples):
Inference accuracy: 0.00%
Macro accuracy (128 classes): 91.74%
Macro precision (128 classes): 33.26%
Macro recall (128 classes): 45.44%
Macro F1 (128 classes): 35.13%
Macro PR AUC (128 classes): 33.73%
Sample-wise F1: 49.90%

Captured dataset metrics (176 samples):
Inference accuracy: 0.00%
Macro accuracy (128 classes): 92.21%
Macro precision (128 classes): 13.28%
Macro recall (128 classes): 28.98%
Macro F1 (128 classes): 16.87%
Macro PR AUC (128 classes): 25.38%
Sample-wise F1: 30.51%
```

# Experiment Pass 7

Back to the baseline of the model.

```rust
let [batch_size, input_size] = input.dims();

  // Reshape to sequence format for attention
  let x = input.reshape([batch_size, 1, input_size]);

  // Apply multi-head attention
  let attn_output = self.mha.forward(MhaInput::new(x.clone(), x.clone(), x));

  // Flatten back to [batch_size, input_size]
  let x = attn_output.context.reshape([batch_size, input_size]);

  // Final linear layer to produce logits
  self.output.forward(x)
```

## Experiment: Note-Binned Convolution and Full Target

### Command

```bash
cargo run --bin kord --no-default-features --features "cli ml_train ml_remote ml_loader_note_binned_convolution ml_target_full" --release -- -q ml train --backend ws://192.168.229.200:3000 --training-sources kord/samples/captured/ --training-sources kord/samples/slakh --training-sources sim --model-epochs 4
```

### Output

```
Validation dataset metrics (1000 samples):
Inference accuracy: 0.00%
Macro accuracy (128 classes): 92.32%
Macro precision (128 classes): 35.27%
Macro recall (128 classes): 46.96%
Macro F1 (128 classes): 36.23%
Macro PR AUC (128 classes): 34.89%
Sample-wise F1: 53.39%

Captured dataset metrics (176 samples):
Inference accuracy: 0.00%
Macro accuracy (128 classes): 94.25%
Macro precision (128 classes): 16.21%
Macro recall (128 classes): 25.04%
Macro F1 (128 classes): 18.39%
Macro PR AUC (128 classes): 24.61%
Sample-wise F1: 34.92%
```

## Experiment: Frequency Pooling and Full Target

### Command

```bash
cargo run --bin kord --no-default-features --features "cli ml_train ml_remote ml_loader_frequency_pooled ml_target_full" --release -- -q ml train --backend ws://192.168.229.200:3000 --training-sources kord/samples/captured/ --training-sources kord/samples/slakh --training-sources sim --model-epochs 4
```

### Output

```
Validation dataset metrics (1000 samples):
Inference accuracy: 0.00%
Macro accuracy (128 classes): 91.81%
Macro precision (128 classes): 34.39%
Macro recall (128 classes): 45.54%
Macro F1 (128 classes): 35.61%
Macro PR AUC (128 classes): 35.24%
Sample-wise F1: 51.43%

Captured dataset metrics (176 samples):
Inference accuracy: 0.00%
Macro accuracy (128 classes): 91.97%
Macro precision (128 classes): 13.43%
Macro recall (128 classes): 28.82%
Macro F1 (128 classes): 17.31%
Macro PR AUC (128 classes): 25.05%
Sample-wise F1: 30.14%
```

# Experiment Pass 8

Fix MHA attention as per GPT-5.

```rust
let x = input.reshape([batch_size, input_size, 1]);
let h = self.embed.forward(x);
let attn = self.mha.forward(MhaInput::new(h.clone(), h.clone(), h));
let pooled = attn.context.mean_dim(1).squeeze(1);
let logits = self.output.forward(pooled);

logits
```

## Experiment: Note-Binned Convolution and Full Target

### Command

```bash
cargo run --bin kord --no-default-features --features "cli ml_train ml_remote ml_loader_note_binned_convolution ml_target_full" --release -- -q ml train --backend ws://192.168.229.200:3000 --training-sources kord/samples/captured/ --training-sources kord/samples/slakh --training-sources sim --model-epochs 4
```

### Output

```
Validation dataset metrics (1000 samples):
Inference accuracy: 0.00%
Macro accuracy (128 classes): 63.90%
Macro precision (128 classes): 7.84%
Macro recall (128 classes): 46.34%
Macro F1 (128 classes): 12.27%
Macro PR AUC (128 classes): 8.18%
Sample-wise F1: 19.53%

Captured dataset metrics (176 samples):
Inference accuracy: 0.00%
Macro accuracy (128 classes): 78.29%
Macro precision (128 classes): 1.51%
Macro recall (128 classes): 2.65%
Macro F1 (128 classes): 0.52%
Macro PR AUC (128 classes): 3.61%
Sample-wise F1: 0.33%
```

## Experiment: Frequency Pooling and Full Target

### Command

```bash
cargo run --bin kord --no-default-features --features "cli ml_train ml_remote ml_loader_frequency_pooled ml_target_full" --release -- -q ml train --backend ws://192.168.229.200:3000 --training-sources kord/samples/captured/ --training-sources kord/samples/slakh --training-sources sim --model-epochs 4
```

### Output

```
Validation dataset metrics (1000 samples):
Inference accuracy: 0.00%
Macro accuracy (128 classes): 48.73%
Macro precision (128 classes): 7.17%
Macro recall (128 classes): 59.65%
Macro F1 (128 classes): 11.50%
Macro PR AUC (128 classes): 8.34%
Sample-wise F1: 16.18%

Captured dataset metrics (176 samples):
Inference accuracy: 0.00%
Macro accuracy (128 classes): 65.88%
Macro precision (128 classes): 2.17%
Macro recall (128 classes): 9.50%
Macro F1 (128 classes): 2.81%
Macro PR AUC (128 classes): 3.14%
Sample-wise F1: 2.82%
```

# Experiment Pass 9

Back to original model, and fixed thresholds with clamping.  No `t_global`.

Now uses 500 sim samples (which turns into 120,000 data items), instead of 100 (24,000).

```rust
let [batch_size, input_size] = input.dims();

// Reshape to sequence format for attention
let x = input.reshape([batch_size, 1, input_size]);

// Apply multi-head attention
let attn_output = self.mha.forward(MhaInput::new(x.clone(), x.clone(), x));

// Flatten back to [batch_size, input_size]
let x = attn_output.context.reshape([batch_size, input_size]);

// Final linear layer to produce logits
self.output.forward(x)
```

## Experiment: Note-Binned Convolution and Full Target

### Command

```bash
cargo run --bin kord --no-default-features --features "cli ml_train ml_remote ml_loader_note_binned_convolution ml_target_full" --release -- -q ml train --backend ws://192.168.229.200:3000 --training-sources kord/samples/captured/ --training-sources kord/samples/slakh --training-sources sim --model-epochs 4
```

### Output

```
Validation dataset metrics (1000 samples):
Inference accuracy: 2.60%
Macro accuracy (128 classes): 96.52%
Macro precision (128 classes): 35.08%
Macro recall (128 classes): 39.55%
Macro F1 (128 classes): 35.92%
Macro PR AUC (128 classes): 34.77%
Sample-wise F1: 60.90%

Captured dataset metrics (176 samples):
Inference accuracy: 4.55%
Macro accuracy (128 classes): 98.13%
Macro precision (128 classes): 17.52%
Macro recall (128 classes): 23.83%
Macro F1 (128 classes): 18.91%
Macro PR AUC (128 classes): 25.75%
Sample-wise F1: 60.93%
```

## Experiment: Frequency Pooling and Full Target

### Command

```bash
cargo run --bin kord --no-default-features --features "cli ml_train ml_remote ml_loader_frequency_pooled ml_target_full" --release -- -q ml train --backend ws://192.168.229.200:3000 --training-sources kord/samples/captured/ --training-sources kord/samples/slakh --training-sources sim --model-epochs 4
```

### Output

```
Validation dataset metrics (1000 samples):
Inference accuracy: 15.10%
Macro accuracy (128 classes): 96.93%
Macro precision (128 classes): 40.04%
Macro recall (128 classes): 43.24%
Macro F1 (128 classes): 40.03%
Macro PR AUC (128 classes): 40.94%
Sample-wise F1: 71.01%

Captured dataset metrics (176 samples):
Inference accuracy: 5.68%
Macro accuracy (128 classes): 97.78%
Macro precision (128 classes): 18.51%
Macro recall (128 classes): 26.60%
Macro F1 (128 classes): 20.34%
Macro PR AUC (128 classes): 26.33%
Sample-wise F1: 60.29%
```

# Experiment Pass 9

Oversampling the captured data by a factor of 16.

## Experiment: Note-Binned Convolution and Full Target

### Command

```bash
cargo run --bin kord --no-default-features --features "cli ml_train ml_remote ml_loader_note_binned_convolution ml_target_full" --release -- -q ml train --backend ws://192.168.229.200:3000 --training-sources kord/samples/captured/ --training-sources kord/samples/slakh --training-sources sim --model-epochs 4
```

### Output

```
Validation dataset metrics (1000 samples):
Inference accuracy: 4.00%
Macro accuracy (128 classes): 96.61%
Macro precision (128 classes): 34.19%
Macro recall (128 classes): 38.42%
Macro F1 (128 classes): 34.91%
Macro PR AUC (128 classes): 35.53%
Sample-wise F1: 62.61%

Captured dataset metrics (176 samples):
Inference accuracy: 11.93%
Macro accuracy (128 classes): 98.57%
Macro precision (128 classes): 20.00%
Macro recall (128 classes): 23.04%
Macro F1 (128 classes): 20.16%
Macro PR AUC (128 classes): 25.86%
Sample-wise F1: 67.86%
```

## Experiment: Frequency Pooling and Full Target

### Command

```bash
cargo run --bin kord --no-default-features --features "cli ml_train ml_remote ml_loader_frequency_pooled ml_target_full" --release -- -q ml train --backend ws://192.168.229.200:3000 --training-sources kord/samples/captured/ --training-sources kord/samples/slakh --training-sources sim --model-epochs 4
```

### Output

```
Validation dataset metrics (1000 samples):
Inference accuracy: 18.90%
Macro accuracy (128 classes): 97.33%
Macro precision (128 classes): 44.15%
Macro recall (128 classes): 43.26%
Macro F1 (128 classes): 42.65%
Macro PR AUC (128 classes): 43.60%
Sample-wise F1: 74.73%

Captured dataset metrics (176 samples):
Inference accuracy: 10.23%
Macro accuracy (128 classes): 98.42%
Macro precision (128 classes): 20.38%
Macro recall (128 classes): 25.87%
Macro F1 (128 classes): 21.52%
Macro PR AUC (128 classes): 27.35%
Sample-wise F1: 67.10%
```

# Experiment Pass 10

Added better per-class logging to the output.

## Experiment: Note-Binned Convolution and Full Target

### Command

```bash
cargo run --bin kord --no-default-features --features "cli ml_train ml_remote ml_loader_note_binned_convolution ml_target_full" --release -- -q ml train --backend ws://192.168.229.200:3000 --training-sources kord/samples/captured/ --training-sources kord/samples/slakh --training-sources sim --model-epochs 4
```

### Output

```
Validation dataset metrics (1000 samples):
Inference accuracy: 2.70%
Macro accuracy (128 classes): 96.73%
Macro precision (128 classes): 35.54%
Macro recall (128 classes): 37.89%
Macro F1 (128 classes): 36.02%
Macro PR AUC (128 classes): 36.70%
Sample-wise F1: 62.91%
Validation class insights:
  Zero-support classes (showing 10): 0 1 2 3 4 5 6 7 8 9
  Lowest precision (10):
    class  23: support    2, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  25: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  26: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  28: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  29: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  30: support    2, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  33: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  34: support    3, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  35: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  36: support    2, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
  Lowest recall (10):
    class  23: support    2, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  25: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  26: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  28: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  29: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  30: support    2, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  33: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  34: support    3, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  35: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  36: support    2, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%

Captured dataset metrics (176 samples):
Inference accuracy: 10.23%
Macro accuracy (128 classes): 98.56%
Macro precision (128 classes): 21.33%
Macro recall (128 classes): 21.81%
Macro F1 (128 classes): 19.81%
Macro PR AUC (128 classes): 25.68%
Sample-wise F1: 66.04%
Captured class insights:
  Zero-support classes (showing 10): 100 103 99 102 45 97 98 95 106 96
  Lowest precision (10):
    class  55: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  64: support    2, predicted    1, precision   0.00%, recall   0.00%, f1   0.00%
    class  93: support    1, predicted   18, precision   5.56%, recall 100.00%, f1  10.53%
    class  58: support    1, predicted    4, precision  25.00%, recall 100.00%, f1  40.00%
    class  92: support    2, predicted    8, precision  25.00%, recall 100.00%, f1  40.00%
    class  68: support    7, predicted   27, precision  25.93%, recall 100.00%, f1  41.18%
    class  89: support   12, predicted    5, precision  40.00%, recall  16.67%, f1  23.53%
    class  71: support   10, predicted   17, precision  41.18%, recall  70.00%, f1  51.85%
    class  94: support    3, predicted    7, precision  42.86%, recall 100.00%, f1  60.00%
    class  72: support   16, predicted   15, precision  46.67%, recall  43.75%, f1  45.16%
  Lowest recall (10):
    class  55: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  64: support    2, predicted    1, precision   0.00%, recall   0.00%, f1   0.00%
    class  89: support   12, predicted    5, precision  40.00%, recall  16.67%, f1  23.53%
    class  74: support   13, predicted    4, precision  75.00%, recall  23.08%, f1  35.29%
    class  62: support    4, predicted    1, precision 100.00%, recall  25.00%, f1  40.00%
    class  86: support   19, predicted    5, precision 100.00%, recall  26.32%, f1  41.67%
    class  72: support   16, predicted   15, precision  46.67%, recall  43.75%, f1  45.16%
    class  78: support   14, predicted    8, precision 100.00%, recall  57.14%, f1  72.73%
    class  83: support   19, predicted   15, precision  73.33%, recall  57.89%, f1  64.71%
    class  84: support   25, predicted   17, precision  88.24%, recall  60.00%, f1  71.43%
```

## Experiment: Frequency Pooling and Full Target

### Command

```bash
cargo run --bin kord --no-default-features --features "cli ml_train ml_remote ml_loader_frequency_pooled ml_target_full" --release -- -q ml train --backend ws://192.168.229.200:3000 --training-sources kord/samples/captured/ --training-sources kord/samples/slakh --training-sources sim --model-epochs 4
```

### Output

```
Validation dataset metrics (1000 samples):
Inference accuracy: 21.60%
Macro accuracy (128 classes): 97.32%
Macro precision (128 classes): 44.80%
Macro recall (128 classes): 43.48%
Macro F1 (128 classes): 43.05%
Macro PR AUC (128 classes): 44.33%
Sample-wise F1: 75.13%
Validation class insights:
  Zero-support classes (showing 10): 0 1 2 3 4 5 6 7 8 9
  Lowest precision (10):
    class  21: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  26: support    2, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  27: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  28: support    2, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  29: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  30: support    3, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  33: support    2, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  34: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class 114: support    3, predicted    1, precision   0.00%, recall   0.00%, f1   0.00%
    class 115: support    1, predicted    3, precision   0.00%, recall   0.00%, f1   0.00%
  Lowest recall (10):
    class  21: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  26: support    2, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  27: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  28: support    2, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  29: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  30: support    3, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  33: support    2, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  34: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class 114: support    3, predicted    1, precision   0.00%, recall   0.00%, f1   0.00%
    class 115: support    1, predicted    3, precision   0.00%, recall   0.00%, f1   0.00%

Captured dataset metrics (176 samples):
Inference accuracy: 8.52%
Macro accuracy (128 classes): 98.36%
Macro precision (128 classes): 19.58%
Macro recall (128 classes): 26.52%
Macro F1 (128 classes): 21.00%
Macro PR AUC (128 classes): 27.51%
Sample-wise F1: 66.42%
Captured class insights:
  Zero-support classes (showing 10): 100 98 103 96 99 95 102 106 110 114
  Lowest precision (10):
    class  55: support    1, predicted    1, precision   0.00%, recall   0.00%, f1   0.00%
    class  93: support    1, predicted   13, precision   7.69%, recall 100.00%, f1  14.29%
    class  92: support    2, predicted   13, precision  15.38%, recall 100.00%, f1  26.67%
    class  94: support    3, predicted   13, precision  23.08%, recall 100.00%, f1  37.50%
    class  68: support    7, predicted   28, precision  25.00%, recall 100.00%, f1  40.00%
    class  56: support    1, predicted    3, precision  33.33%, recall 100.00%, f1  50.00%
    class  58: support    1, predicted    3, precision  33.33%, recall 100.00%, f1  50.00%
    class  71: support   10, predicted   26, precision  34.62%, recall  90.00%, f1  50.00%
    class  91: support   11, predicted   26, precision  38.46%, recall  90.91%, f1  54.05%
    class  70: support   13, predicted   25, precision  44.00%, recall  84.62%, f1  57.89%
  Lowest recall (10):
    class  55: support    1, predicted    1, precision   0.00%, recall   0.00%, f1   0.00%
    class  89: support   12, predicted    3, precision 100.00%, recall  25.00%, f1  40.00%
    class  64: support    2, predicted    1, precision 100.00%, recall  50.00%, f1  66.67%
    class  74: support   13, predicted    9, precision  77.78%, recall  53.85%, f1  63.64%
    class  88: support   11, predicted   13, precision  46.15%, recall  54.55%, f1  50.00%
    class  86: support   19, predicted   20, precision  60.00%, recall  63.16%, f1  61.54%
    class  73: support   11, predicted   17, precision  47.06%, recall  72.73%, f1  57.14%
    class  62: support    4, predicted    6, precision  50.00%, recall  75.00%, f1  60.00%
    class  66: support    8, predicted    6, precision 100.00%, recall  75.00%, f1  85.71%
    class  90: support    8, predicted    6, precision 100.00%, recall  75.00%, f1  85.71%
```

# Experiment Pass 11

Updated the model to expand the MHA width.

```rust
let [batch_size, input_size] = input.dims();

// Treat each input bin as a token, projecting to the configured attention width.
let tokens = input.reshape([batch_size, input_size, 1]);
let embedded = self.projector.forward(tokens);

// Attend over the projected tokens.
let attn_output = self.mha.forward(MhaInput::new(embedded.clone(), embedded.clone(), embedded));

// Average the attended tokens and classify.
let pooled = attn_output.context.mean_dim(1).squeeze(1);
self.output.forward(pooled)
```

## Experiment: Frequency Pooling and Full Target

### Command

```bash
cargo run --bin kord --no-default-features --features "cli ml_train ml_remote ml_loader_frequency_pooled ml_target_full" --release -- -q ml train --backend ws://192.168.229.200:3000 --training-sources kord/samples/captured/ --training-sources kord/samples/slakh --training-sources sim --model-epochs 4
```

### Output

```
Validation dataset metrics (1000 samples):
Inference accuracy: 0.00%
Macro accuracy (128 classes): 83.24%
Macro precision (128 classes): 7.32%
Macro recall (128 classes): 27.84%
Macro F1 (128 classes): 11.02%
Macro PR AUC (128 classes): 7.71%
Sample-wise F1: 18.12%
Validation class insights:
  Zero-support classes (showing 10): 0 1 2 3 4 5 6 7 8 9
  Lowest precision (10):
    class  23: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  26: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  30: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  31: support    2, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  32: support    3, predicted    2, precision   0.00%, recall   0.00%, f1   0.00%
    class  33: support    2, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  35: support    2, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  36: support    3, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  37: support    4, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  38: support    1, predicted    2, precision   0.00%, recall   0.00%, f1   0.00%
  Lowest recall (10):
    class  23: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  26: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  30: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  31: support    2, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  32: support    3, predicted    2, precision   0.00%, recall   0.00%, f1   0.00%
    class  33: support    2, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  35: support    2, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  36: support    3, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  37: support    4, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  38: support    1, predicted    2, precision   0.00%, recall   0.00%, f1   0.00%

Captured dataset metrics (176 samples):
Inference accuracy: 0.00%
Macro accuracy (128 classes): 83.36%
Macro precision (128 classes): 1.76%
Macro recall (128 classes): 11.77%
Macro F1 (128 classes): 2.98%
Macro PR AUC (128 classes): 2.33%
Sample-wise F1: 9.66%
Captured class insights:
  Zero-support classes (showing 10): 106 104 96 99 111 101 103 97 108 105
  Lowest precision (10):
    class  43: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  55: support    1, predicted   16, precision   0.00%, recall   0.00%, f1   0.00%
    class  56: support    1, predicted    5, precision   0.00%, recall   0.00%, f1   0.00%
    class  58: support    1, predicted   23, precision   0.00%, recall   0.00%, f1   0.00%
    class  59: support    3, predicted   10, precision   0.00%, recall   0.00%, f1   0.00%
    class  60: support    3, predicted   17, precision   0.00%, recall   0.00%, f1   0.00%
    class  61: support    2, predicted    6, precision   0.00%, recall   0.00%, f1   0.00%
    class  64: support    2, predicted    6, precision   0.00%, recall   0.00%, f1   0.00%
    class  88: support   11, predicted   15, precision   0.00%, recall   0.00%, f1   0.00%
    class  90: support    8, predicted   21, precision   0.00%, recall   0.00%, f1   0.00%
  Lowest recall (10):
    class  43: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  55: support    1, predicted   16, precision   0.00%, recall   0.00%, f1   0.00%
    class  56: support    1, predicted    5, precision   0.00%, recall   0.00%, f1   0.00%
    class  58: support    1, predicted   23, precision   0.00%, recall   0.00%, f1   0.00%
    class  59: support    3, predicted   10, precision   0.00%, recall   0.00%, f1   0.00%
    class  60: support    3, predicted   17, precision   0.00%, recall   0.00%, f1   0.00%
    class  61: support    2, predicted    6, precision   0.00%, recall   0.00%, f1   0.00%
    class  64: support    2, predicted    6, precision   0.00%, recall   0.00%, f1   0.00%
    class  88: support   11, predicted   15, precision   0.00%, recall   0.00%, f1   0.00%
    class  90: support    8, predicted   21, precision   0.00%, recall   0.00%, f1   0.00%
```

# Experiment Pass 12

Updated the model to use sinusoidal encoding.

```rust
let tokens = input.reshape([batch_size, input_size, 1]);
let mut embedded = self.projector.forward(tokens);

let [_, _, token_width] = embedded.dims();
let device = embedded.device();
let positional = sinusoidal_pe::<B>(input_size, token_width, &device);
let positional = positional.unsqueeze::<3>().expand([batch_size, input_size, token_width]);
embedded = embedded + positional;

// Attend over the projected tokens.
let attn_output = self.mha.forward(MhaInput::new(embedded.clone(), embedded.clone(), embedded));

// Average the attended tokens and classify.
let pooled = attn_output.context.mean_dim(1).squeeze(1);
self.output.forward(pooled)
```

## Experiment: Frequency Pooling and Full Target

### Command

```bash
cargo run --bin kord --no-default-features --features "cli ml_train ml_remote ml_loader_frequency_pooled ml_target_full" --release -- -q ml train --backend ws://192.168.229.200:3000 --training-sources kord/samples/captured/ --training-sources kord/samples/slakh --training-sources sim --model-epochs 4
```

### Output

```
Validation dataset metrics (1000 samples):
Inference accuracy: 9.80%
Macro accuracy (128 classes): 96.18%
Macro precision (128 classes): 36.21%
Macro recall (128 classes): 40.42%
Macro F1 (128 classes): 36.42%
Macro PR AUC (128 classes): 34.44%
Sample-wise F1: 57.71%
Validation class insights:
  Zero-support classes (showing 10): 0 1 2 3 4 5 6 7 8 9
  Lowest precision (10):
    class  23: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  24: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  27: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  29: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  33: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  35: support    2, predicted    2, precision   0.00%, recall   0.00%, f1   0.00%
    class 120: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  38: support    2, predicted   13, precision   7.69%, recall  50.00%, f1  13.33%
    class  34: support    3, predicted   18, precision  16.67%, recall 100.00%, f1  28.57%
    class  36: support    4, predicted   18, precision  16.67%, recall  75.00%, f1  27.27%
  Lowest recall (10):
    class  23: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  24: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  27: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  29: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  33: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  35: support    2, predicted    2, precision   0.00%, recall   0.00%, f1   0.00%
    class 120: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  39: support    6, predicted    2, precision  50.00%, recall  16.67%, f1  25.00%
    class  40: support    4, predicted    2, precision  50.00%, recall  25.00%, f1  33.33%
    class 116: support    6, predicted    5, precision  40.00%, recall  33.33%, f1  36.36%

Captured dataset metrics (176 samples):
Inference accuracy: 10.23%
Macro accuracy (128 classes): 98.15%
Macro precision (128 classes): 16.72%
Macro recall (128 classes): 14.29%
Macro F1 (128 classes): 14.07%
Macro PR AUC (128 classes): 17.53%
Sample-wise F1: 47.11%
Captured class insights:
  Zero-support classes (showing 10): 96 98 109 95 53 101 108 57 100 106
  Lowest precision (10):
    class  43: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  55: support    1, predicted    2, precision   0.00%, recall   0.00%, f1   0.00%
    class  62: support    4, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  66: support    8, predicted    1, precision   0.00%, recall   0.00%, f1   0.00%
    class  68: support    7, predicted    2, precision   0.00%, recall   0.00%, f1   0.00%
    class  92: support    2, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  93: support    1, predicted    1, precision   0.00%, recall   0.00%, f1   0.00%
    class  94: support    3, predicted   15, precision   6.67%, recall  33.33%, f1  11.11%
    class  88: support   11, predicted   10, precision  20.00%, recall  18.18%, f1  19.05%
    class  69: support   11, predicted    4, precision  25.00%, recall   9.09%, f1  13.33%
  Lowest recall (10):
    class  43: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  55: support    1, predicted    2, precision   0.00%, recall   0.00%, f1   0.00%
    class  62: support    4, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  66: support    8, predicted    1, precision   0.00%, recall   0.00%, f1   0.00%
    class  68: support    7, predicted    2, precision   0.00%, recall   0.00%, f1   0.00%
    class  92: support    2, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  93: support    1, predicted    1, precision   0.00%, recall   0.00%, f1   0.00%
    class  69: support   11, predicted    4, precision  25.00%, recall   9.09%, f1  13.33%
    class  78: support   14, predicted    4, precision  50.00%, recall  14.29%, f1  22.22%
    class  73: support   11, predicted    4, precision  50.00%, recall  18.18%, f1  26.67%
```

# Experiment Pass 13

Add scale and bias to input.

```rust
let [batch_size, input_size] = input.dims();

let tokens = input.reshape([batch_size, input_size, 1]);
let scale_raw = self.frequency_scale.val();
let bias_raw = self.frequency_bias.val();
let token_width = scale_raw.dims()[1];
let scale = scale_raw.unsqueeze::<3>().expand([batch_size, input_size, token_width]);
let bias = bias_raw.unsqueeze::<3>().expand([batch_size, input_size, token_width]);
let mut embedded = tokens * scale + bias;

let [_, _, token_width] = embedded.dims();
let device = embedded.device();
let positional = sinusoidal_pe::<B>(input_size, token_width, &device);
let positional = positional.unsqueeze::<3>().expand([batch_size, input_size, token_width]);
embedded = embedded + positional;

// Attend over the projected tokens.
let attn_output = self.mha.forward(MhaInput::new(embedded.clone(), embedded.clone(), embedded));

// Average the attended tokens and classify.
let pooled = attn_output.context.mean_dim(1).squeeze(1);
self.output.forward(pooled)
```

## Experiment: Frequency Pooling and Full Target

### Command

```bash
cargo run --bin kord --no-default-features --features "cli ml_train ml_remote ml_loader_frequency_pooled ml_target_full" --release -- -q ml train --backend ws://192.168.229.200:3000 --training-sources kord/samples/captured/ --training-sources kord/samples/slakh --training-sources sim --model-epochs 4
```

### Output

```
Validation dataset metrics (1000 samples):
Inference accuracy: 9.60%
Macro accuracy (128 classes): 96.36%
Macro precision (128 classes): 37.92%
Macro recall (128 classes): 41.33%
Macro F1 (128 classes): 38.35%
Macro PR AUC (128 classes): 38.35%
Sample-wise F1: 63.67%
Validation class insights:
  Zero-support classes (showing 10): 0 1 2 3 4 5 6 7 8 9
  Lowest precision (10):
    class  24: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  25: support    2, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  28: support    2, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  29: support    1, predicted    6, precision   0.00%, recall   0.00%, f1   0.00%
    class  30: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  31: support    3, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  32: support    1, predicted   19, precision   0.00%, recall   0.00%, f1   0.00%
    class  35: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  37: support    7, predicted    4, precision   0.00%, recall   0.00%, f1   0.00%
    class  41: support    7, predicted   31, precision  16.13%, recall  71.43%, f1  26.32%
  Lowest recall (10):
    class  24: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  25: support    2, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  28: support    2, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  29: support    1, predicted    6, precision   0.00%, recall   0.00%, f1   0.00%
    class  30: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  31: support    3, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  32: support    1, predicted   19, precision   0.00%, recall   0.00%, f1   0.00%
    class  35: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  37: support    7, predicted    4, precision   0.00%, recall   0.00%, f1   0.00%
    class  39: support    4, predicted    5, precision  20.00%, recall  25.00%, f1  22.22%

Captured dataset metrics (176 samples):
Inference accuracy: 2.84%
Macro accuracy (128 classes): 97.88%
Macro precision (128 classes): 16.04%
Macro recall (128 classes): 22.79%
Macro F1 (128 classes): 17.51%
Macro PR AUC (128 classes): 22.95%
Sample-wise F1: 56.71%
Captured class insights:
  Zero-support classes (showing 10): 99 96 95 98 102 103 107 115 100 101
  Lowest precision (10):
    class  55: support    1, predicted    1, precision   0.00%, recall   0.00%, f1   0.00%
    class  64: support    2, predicted    3, precision   0.00%, recall   0.00%, f1   0.00%
    class  93: support    1, predicted    9, precision  11.11%, recall 100.00%, f1  20.00%
    class  58: support    1, predicted    6, precision  16.67%, recall 100.00%, f1  28.57%
    class  94: support    3, predicted   16, precision  18.75%, recall 100.00%, f1  31.58%
    class  68: support    7, predicted   36, precision  19.44%, recall 100.00%, f1  32.56%
    class  60: support    3, predicted   12, precision  25.00%, recall 100.00%, f1  40.00%
    class  62: support    4, predicted    4, precision  25.00%, recall  25.00%, f1  25.00%
    class  92: support    2, predicted    8, precision  25.00%, recall 100.00%, f1  40.00%
    class  59: support    3, predicted    7, precision  28.57%, recall  66.67%, f1  40.00%
  Lowest recall (10):
    class  55: support    1, predicted    1, precision   0.00%, recall   0.00%, f1   0.00%
    class  64: support    2, predicted    3, precision   0.00%, recall   0.00%, f1   0.00%
    class  62: support    4, predicted    4, precision  25.00%, recall  25.00%, f1  25.00%
    class  89: support   12, predicted   11, precision  36.36%, recall  33.33%, f1  34.78%
    class  88: support   11, predicted    6, precision  66.67%, recall  36.36%, f1  47.06%
    class  86: support   19, predicted   14, precision  57.14%, recall  42.11%, f1  48.48%
    class  91: support   11, predicted    6, precision  83.33%, recall  45.45%, f1  58.82%
    class  74: support   13, predicted    9, precision  66.67%, recall  46.15%, f1  54.55%
    class  79: support   26, predicted   20, precision  65.00%, recall  50.00%, f1  56.52%
    class  76: support   19, predicted   18, precision  55.56%, recall  52.63%, f1  54.05%
```

# Experiment Pass 14

Complete new model with CQT.

```rust
let [batch_size, _] = input.dims();

// Project the dense FFT spectrum into logarithmically spaced energy bands and ensure numerical stability.
let projected = self.log_projection.forward(input).abs();
let log_tokens = projected.add_scalar(1.0e-6).log();

// Treat each band as a sequence token and project into the transformer width.
let mut tokens = self.embed.forward(log_tokens.reshape([batch_size, self.token_count, 1]));

let device = tokens.device();
let positional = sinusoidal_pe::<B>(self.token_count, MODEL_DIM, &device)
    .unsqueeze::<3>()
    .expand([batch_size, self.token_count, MODEL_DIM]);
tokens = tokens + positional;

// Transformer block.
let attn_input = self.norm_attn.forward(tokens.clone());
let attn = self.mha.forward(MhaInput::new(attn_input.clone(), attn_input.clone(), attn_input));
let tokens = tokens + self.attn_dropout.forward(attn.context);

let ffn_input = self.norm_ffn.forward(tokens.clone());
let ff = self.ffn_in.forward(ffn_input);
let ff = Gelu::new().forward(ff);
let ff = self.ffn_out.forward(ff);
let tokens = tokens + ff;

let pooled = tokens.mean_dim(1).squeeze(1);
self.output.forward(pooled)
```

## Experiment: Frequency Pooling and Full Target

### Command

```bash
cargo run --bin kord --no-default-features --features "cli ml_train ml_remote ml_loader_frequency_pooled ml_target_full" --release -- -q ml train --backend ws://192.168.229.200:3000 --training-sources kord/samples/captured/ --training-sources kord/samples/slakh --training-sources sim --model-epochs 4
```

### Output

```
Validation dataset metrics (1000 samples):
Inference accuracy: 6.00%
Macro accuracy (128 classes): 96.37%
Macro precision (128 classes): 34.83%
Macro recall (128 classes): 40.34%
Macro F1 (128 classes): 35.78%
Macro PR AUC (128 classes): 34.11%
Sample-wise F1: 57.75%
Validation class insights:
  Zero-support classes (showing 10): 0 1 2 3 4 5 6 7 8 9
  Lowest precision (10):
    class  22: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  23: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  27: support    1, predicted    7, precision   0.00%, recall   0.00%, f1   0.00%
    class  34: support    3, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  37: support    3, predicted    7, precision   0.00%, recall   0.00%, f1   0.00%
    class  32: support    2, predicted   43, precision   2.33%, recall  50.00%, f1   4.44%
    class  41: support    4, predicted   27, precision   7.41%, recall  50.00%, f1  12.90%
    class  38: support    7, predicted    7, precision  14.29%, recall  14.29%, f1  14.29%
    class  39: support    5, predicted   14, precision  14.29%, recall  40.00%, f1  21.05%
    class  36: support    3, predicted   13, precision  15.38%, recall  66.67%, f1  25.00%
  Lowest recall (10):
    class  22: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  23: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  27: support    1, predicted    7, precision   0.00%, recall   0.00%, f1   0.00%
    class  34: support    3, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  37: support    3, predicted    7, precision   0.00%, recall   0.00%, f1   0.00%
    class  38: support    7, predicted    7, precision  14.29%, recall  14.29%, f1  14.29%
    class  52: support   12, predicted    3, precision  66.67%, recall  16.67%, f1  26.67%
    class  40: support    5, predicted    4, precision  25.00%, recall  20.00%, f1  22.22%
    class 115: support    9, predicted    4, precision  50.00%, recall  22.22%, f1  30.77%
    class  57: support   27, predicted   15, precision  46.67%, recall  25.93%, f1  33.33%

Captured dataset metrics (176 samples):
Inference accuracy: 3.41%
Macro accuracy (128 classes): 97.27%
Macro precision (128 classes): 12.22%
Macro recall (128 classes): 20.80%
Macro F1 (128 classes): 14.06%
Macro PR AUC (128 classes): 15.45%
Sample-wise F1: 46.08%
Captured class insights:
  Zero-support classes (showing 10): 102 105 97 104 100 98 99 103 95 101
  Lowest precision (10):
    class  43: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  93: support    1, predicted   25, precision   4.00%, recall 100.00%, f1   7.69%
    class  56: support    1, predicted    9, precision  11.11%, recall 100.00%, f1  20.00%
    class  92: support    2, predicted   17, precision  11.76%, recall 100.00%, f1  21.05%
    class  55: support    1, predicted    5, precision  20.00%, recall 100.00%, f1  33.33%
    class  58: support    1, predicted    5, precision  20.00%, recall 100.00%, f1  33.33%
    class  59: support    3, predicted    8, precision  25.00%, recall  66.67%, f1  36.36%
    class  61: support    2, predicted    4, precision  25.00%, recall  50.00%, f1  33.33%
    class  64: support    2, predicted    4, precision  25.00%, recall  50.00%, f1  33.33%
    class  68: support    7, predicted   27, precision  25.93%, recall 100.00%, f1  41.18%
  Lowest recall (10):
    class  43: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  69: support   11, predicted    5, precision  80.00%, recall  36.36%, f1  50.00%
    class  72: support   16, predicted   16, precision  37.50%, recall  37.50%, f1  37.50%
    class  86: support   19, predicted   12, precision  66.67%, recall  42.11%, f1  51.61%
    class  78: support   14, predicted   19, precision  31.58%, recall  42.86%, f1  36.36%
    class  73: support   11, predicted    9, precision  55.56%, recall  45.45%, f1  50.00%
    class  74: support   13, predicted   15, precision  40.00%, recall  46.15%, f1  42.86%
    class  84: support   25, predicted   22, precision  54.55%, recall  48.00%, f1  51.06%
    class  61: support    2, predicted    4, precision  25.00%, recall  50.00%, f1  33.33%
    class  62: support    4, predicted    6, precision  33.33%, recall  50.00%, f1  40.00%
```

# Experiment Pass 15

Added a second note-binned projection.

```rust
let [batch_size, _] = input.dims();

// Project the dense FFT spectrum into logarithmically spaced energy bands and ensure numerical stability.
let projected = self.log_projection.forward(input.clone()).abs();
let log_tokens = projected.add_scalar(1.0e-6).log();

// Derive note-binned features from the same spectrum for an auxiliary token stream.
let note_raw = self.note_projection.forward(input).abs();
let note_tokens = note_raw.add_scalar(1.0e-6).log();

// Treat each band as a sequence token and project into the transformer width.
let freq_tokens = log_tokens.reshape([batch_size, self.token_count - NOTE_TOKEN_COUNT, 1]);
let note_tokens = note_tokens.reshape([batch_size, NOTE_TOKEN_COUNT, 1]);
let cat_tokens = Tensor::cat(vec![freq_tokens, note_tokens], 1);
let mut tokens = self.embed.forward(cat_tokens);

let device = tokens.device();
let positional = sinusoidal_pe::<B>(self.token_count, MODEL_DIM, &device)
    .unsqueeze::<3>()
    .expand([batch_size, self.token_count, MODEL_DIM]);
tokens = tokens + positional;

// Transformer block.
let attn_input = self.norm_attn.forward(tokens.clone());
let attn = self.mha.forward(MhaInput::new(attn_input.clone(), attn_input.clone(), attn_input));
let tokens = tokens + self.attn_dropout.forward(attn.context);

let ffn_input = self.norm_ffn.forward(tokens.clone());
let ff = self.ffn_in.forward(ffn_input);
let ff = Gelu::new().forward(ff);
let ff = self.ffn_out.forward(ff);
let tokens = tokens + ff;

let pooled = tokens.mean_dim(1).squeeze(1);
self.output.forward(pooled)
```

## Experiment: Frequency Pooling and Full Target

### Command

```bash
cargo run --bin kord --no-default-features --features "cli ml_train ml_remote ml_loader_frequency_pooled ml_target_full" --release -- -q ml train --backend ws://192.168.229.200:3000 --training-sources kord/samples/captured/ --training-sources kord/samples/slakh --training-sources sim --model-epochs 4
```

### Output

```
Validation dataset metrics (1000 samples):
Inference accuracy: 4.80%
Macro accuracy (128 classes): 96.12%
Macro precision (128 classes): 35.60%
Macro recall (128 classes): 44.22%
Macro F1 (128 classes): 37.05%
Macro PR AUC (128 classes): 35.62%
Sample-wise F1: 55.18%
Validation class insights:
  Zero-support classes (showing 10): 0 1 2 3 4 5 6 7 8 9
  Lowest precision (10):
    class  20: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  26: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  27: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  31: support    2, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  34: support    2, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class 122: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class 123: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  37: support    2, predicted   23, precision   4.35%, recall  50.00%, f1   8.00%
    class  33: support    1, predicted   14, precision   7.14%, recall 100.00%, f1  13.33%
    class  39: support    5, predicted   43, precision   9.30%, recall  80.00%, f1  16.67%
  Lowest recall (10):
    class  20: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  26: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  27: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  31: support    2, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  34: support    2, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class 122: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class 123: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  40: support    8, predicted    4, precision  25.00%, recall  12.50%, f1  16.67%
    class  42: support    5, predicted    9, precision  11.11%, recall  20.00%, f1  14.29%
    class 113: support   22, predicted    5, precision 100.00%, recall  22.73%, f1  37.04%

Captured dataset metrics (176 samples):
Inference accuracy: 1.14%
Macro accuracy (128 classes): 97.20%
Macro precision (128 classes): 12.17%
Macro recall (128 classes): 17.14%
Macro F1 (128 classes): 13.20%
Macro PR AUC (128 classes): 15.37%
Sample-wise F1: 39.52%
Captured class insights:
  Zero-support classes (showing 10): 102 103 98 96 97 99 95 100 101 110
  Lowest precision (10):
    class  43: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  55: support    1, predicted    4, precision   0.00%, recall   0.00%, f1   0.00%
    class  64: support    2, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  93: support    1, predicted    9, precision   0.00%, recall   0.00%, f1   0.00%
    class  92: support    2, predicted   21, precision   9.52%, recall 100.00%, f1  17.39%
    class  62: support    4, predicted    7, precision  14.29%, recall  25.00%, f1  18.18%
    class  68: support    7, predicted   27, precision  18.52%, recall  71.43%, f1  29.41%
    class  56: support    1, predicted    5, precision  20.00%, recall 100.00%, f1  33.33%
    class  60: support    3, predicted    5, precision  20.00%, recall  33.33%, f1  25.00%
    class  70: support   13, predicted   10, precision  20.00%, recall  15.38%, f1  17.39%
  Lowest recall (10):
    class  43: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  55: support    1, predicted    4, precision   0.00%, recall   0.00%, f1   0.00%
    class  64: support    2, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  93: support    1, predicted    9, precision   0.00%, recall   0.00%, f1   0.00%
    class  70: support   13, predicted   10, precision  20.00%, recall  15.38%, f1  17.39%
    class  62: support    4, predicted    7, precision  14.29%, recall  25.00%, f1  18.18%
    class  76: support   19, predicted   12, precision  50.00%, recall  31.58%, f1  38.71%
    class  60: support    3, predicted    5, precision  20.00%, recall  33.33%, f1  25.00%
    class  89: support   12, predicted    9, precision  44.44%, recall  33.33%, f1  38.10%
    class  78: support   14, predicted   15, precision  33.33%, recall  35.71%, f1  34.48%
```

# Experiment Pass 16

Swapped the auxiliary note stream to use log(1 + x) and normalized it with a new LayerNorm before attention so the concatenated tokens share a stable scale (model.rs). This implements the earlier plan to soften the note-token values instead of pushing them far negative and to keep per-token magnitudes balanced ahead of the transformer block.

```rust
let [batch_size, _] = input.dims();

// Project the dense FFT spectrum into logarithmically spaced energy bands and ensure numerical stability.
let projected = self.log_projection.forward(input.clone()).abs();
let log_tokens = projected.add_scalar(1.0e-6).log();

// Derive note-binned features from the same spectrum for an auxiliary token stream.
let note_raw = self.note_projection.forward(input).abs();
let note_tokens = note_raw.add_scalar(1.0).log();

// Treat each band as a sequence token and project into the transformer width.
let freq_tokens = log_tokens.reshape([batch_size, self.token_count - NOTE_TOKEN_COUNT, 1]);
let note_tokens = note_tokens.reshape([batch_size, NOTE_TOKEN_COUNT, 1]);
let cat_tokens = Tensor::cat(vec![freq_tokens, note_tokens], 1);
let mut tokens = self.embed.forward(cat_tokens);
tokens = self.token_norm.forward(tokens);

let device = tokens.device();
let positional = sinusoidal_pe::<B>(self.token_count, MODEL_DIM, &device)
    .unsqueeze::<3>()
    .expand([batch_size, self.token_count, MODEL_DIM]);
tokens = tokens + positional;

// Transformer block.
let attn_input = self.norm_attn.forward(tokens.clone());
let attn = self.mha.forward(MhaInput::new(attn_input.clone(), attn_input.clone(), attn_input));
let tokens = tokens + self.attn_dropout.forward(attn.context);

let ffn_input = self.norm_ffn.forward(tokens.clone());
let ff = self.ffn_in.forward(ffn_input);
let ff = Gelu::new().forward(ff);
let ff = self.ffn_out.forward(ff);
let tokens = tokens + ff;

let pooled = tokens.mean_dim(1).squeeze(1);
self.output.forward(pooled)
```

## Experiment: Frequency Pooling and Full Target

### Command

```bash
cargo run --bin kord --no-default-features --features "cli ml_train ml_remote ml_loader_frequency_pooled ml_target_full" --release -- -q ml train --backend ws://192.168.229.200:3000 --training-sources kord/samples/captured/ --training-sources kord/samples/slakh --training-sources sim --model-epochs 4
```

### Output

```
Validation dataset metrics (1000 samples):
Inference accuracy: 8.80%
Macro accuracy (128 classes): 96.31%
Macro precision (128 classes): 36.67%
Macro recall (128 classes): 39.21%
Macro F1 (128 classes): 36.55%
Macro PR AUC (128 classes): 36.02%
Sample-wise F1: 59.51%
Validation class insights:
  Zero-support classes (showing 10): 0 1 2 3 4 5 6 7 8 9
  Lowest precision (10):
    class  23: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  24: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  25: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  26: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  31: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  33: support    2, predicted   37, precision   0.00%, recall   0.00%, f1   0.00%
    class  34: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  36: support    4, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  37: support    5, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  38: support    3, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
  Lowest recall (10):
    class  23: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  24: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  25: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  26: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  31: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  33: support    2, predicted   37, precision   0.00%, recall   0.00%, f1   0.00%
    class  34: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  36: support    4, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  37: support    5, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  38: support    3, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%

Captured dataset metrics (176 samples):
Inference accuracy: 6.25%
Macro accuracy (128 classes): 97.86%
Macro precision (128 classes): 13.73%
Macro recall (128 classes): 18.06%
Macro F1 (128 classes): 14.00%
Macro PR AUC (128 classes): 16.57%
Sample-wise F1: 53.92%
Captured class insights:
  Zero-support classes (showing 10): 96 101 100 97 106 98 104 95 99 50
  Lowest precision (10):
    class  43: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  55: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  56: support    1, predicted    3, precision   0.00%, recall   0.00%, f1   0.00%
    class  59: support    3, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  60: support    3, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  61: support    2, predicted    1, precision   0.00%, recall   0.00%, f1   0.00%
    class  64: support    2, predicted    1, precision   0.00%, recall   0.00%, f1   0.00%
    class  58: support    1, predicted   14, precision   7.14%, recall 100.00%, f1  13.33%
    class  93: support    1, predicted   13, precision   7.69%, recall 100.00%, f1  14.29%
    class  65: support    6, predicted    6, precision  16.67%, recall  16.67%, f1  16.67%
  Lowest recall (10):
    class  43: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  55: support    1, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  56: support    1, predicted    3, precision   0.00%, recall   0.00%, f1   0.00%
    class  59: support    3, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  60: support    3, predicted    0, precision   0.00%, recall   0.00%, f1   0.00%
    class  61: support    2, predicted    1, precision   0.00%, recall   0.00%, f1   0.00%
    class  64: support    2, predicted    1, precision   0.00%, recall   0.00%, f1   0.00%
    class  71: support   10, predicted    1, precision 100.00%, recall  10.00%, f1  18.18%
    class  65: support    6, predicted    6, precision  16.67%, recall  16.67%, f1  16.67%
    class  89: support   12, predicted    6, precision  66.67%, recall  33.33%, f1  44.44%
```

# Experiment Pass 17

Trying a new target that is a 12-bin one-hot coupled with a 12-bin multi-hot.  First 12 is to denote "bass" or "slash" note, and second 12 is to denote all other note content.  This is to help the model focus on bass detection while still learning full note context.  If this works well, we can have the "chord guesser" guess based off the bass note and the collection of all notes heard.

## Experiment: Frequency Pooling and Folded Bass Target

### Command

```bash
cargo run --bin kord --no-default-features --features "cli ml_train ml_remote ml_loader_frequency_pooled ml_target_folded_bass" --release -- -q ml train --backend ws://192.168.229.200:3000 --training-sources kord/samples/captured/ --training-sources kord/samples/slakh --training-sources sim --model-epochs 4
```

### Output

```
Validation dataset metrics (1000 samples):
Inference accuracy: 26.50%
Macro accuracy (24 classes): 93.33%
Macro precision (24 classes): 73.54%
Macro recall (24 classes): 78.82%
Macro F1 (24 classes): 75.64%
Macro PR AUC (24 classes): 80.89%
Sample-wise F1: 83.01%
Validation class insights:
  Lowest precision (10):
    class  11: support   78, predicted  112, precision  50.89%, recall  73.08%, f1  60.00%
    class  10: support   71, predicted   96, precision  52.08%, recall  70.42%, f1  59.88%
    class   3: support   51, predicted   81, precision  53.09%, recall  84.31%, f1  65.15%
    class   8: support   70, predicted  106, precision  53.77%, recall  81.43%, f1  64.77%
    class   7: support  104, predicted  130, precision  55.38%, recall  69.23%, f1  61.54%
    class   4: support   97, predicted  111, precision  55.86%, recall  63.92%, f1  59.62%
    class   6: support   70, predicted   94, precision  57.45%, recall  77.14%, f1  65.85%
    class   5: support   98, predicted  107, precision  57.94%, recall  63.27%, f1  60.49%
    class   1: support   59, predicted   60, precision  61.67%, recall  62.71%, f1  62.18%
    class   2: support   95, predicted   87, precision  67.82%, recall  62.11%, f1  64.84%
  Lowest recall (10):
    class   2: support   95, predicted   87, precision  67.82%, recall  62.11%, f1  64.84%
    class   1: support   59, predicted   60, precision  61.67%, recall  62.71%, f1  62.18%
    class   5: support   98, predicted  107, precision  57.94%, recall  63.27%, f1  60.49%
    class   9: support  104, predicted   95, precision  69.47%, recall  63.46%, f1  66.33%
    class   4: support   97, predicted  111, precision  55.86%, recall  63.92%, f1  59.62%
    class   0: support  103, predicted   96, precision  69.79%, recall  65.05%, f1  67.34%
    class   7: support  104, predicted  130, precision  55.38%, recall  69.23%, f1  61.54%
    class  10: support   71, predicted   96, precision  52.08%, recall  70.42%, f1  59.88%
    class  11: support   78, predicted  112, precision  50.89%, recall  73.08%, f1  60.00%
    class   6: support   70, predicted   94, precision  57.45%, recall  77.14%, f1  65.85%

Captured dataset metrics (176 samples):
Inference accuracy: 25.00%
Macro accuracy (24 classes): 93.37%
Macro precision (24 classes): 75.30%
Macro recall (24 classes): 76.63%
Macro F1 (24 classes): 72.26%
Macro PR AUC (24 classes): 84.21%
Sample-wise F1: 78.72%
Captured class insights:
  Lowest precision (10):
    class   8: support   11, predicted   34, precision  29.41%, recall  90.91%, f1  44.44%
    class  10: support   10, predicted   23, precision  34.78%, recall  80.00%, f1  48.48%
    class  11: support   14, predicted   27, precision  40.74%, recall  78.57%, f1  53.66%
    class   9: support   14, predicted   23, precision  43.48%, recall  71.43%, f1  54.05%
    class   6: support   11, predicted   19, precision  47.37%, recall  81.82%, f1  60.00%
    class   5: support   20, predicted   26, precision  53.85%, recall  70.00%, f1  60.87%
    class   7: support   18, predicted   16, precision  62.50%, recall  55.56%, f1  58.82%
    class   3: support   12, predicted   14, precision  64.29%, recall  75.00%, f1  69.23%
    class  13: support   27, predicted   30, precision  70.00%, recall  77.78%, f1  73.68%
    class  23: support   32, predicted   41, precision  73.17%, recall  93.75%, f1  82.19%
  Lowest recall (10):
    class   0: support   25, predicted    8, precision 100.00%, recall  32.00%, f1  48.48%
    class   2: support   15, predicted    5, precision 100.00%, recall  33.33%, f1  50.00%
    class   7: support   18, predicted   16, precision  62.50%, recall  55.56%, f1  58.82%
    class   1: support   12, predicted    7, precision 100.00%, recall  58.33%, f1  73.68%
    class  12: support   44, predicted   29, precision 100.00%, recall  65.91%, f1  79.45%
    class  14: support   36, predicted   26, precision  96.15%, recall  69.44%, f1  80.65%
    class   5: support   20, predicted   26, precision  53.85%, recall  70.00%, f1  60.87%
    class   9: support   14, predicted   23, precision  43.48%, recall  71.43%, f1  54.05%
    class   3: support   12, predicted   14, precision  64.29%, recall  75.00%, f1  69.23%
    class  18: support   30, predicted   29, precision  79.31%, recall  76.67%, f1  77.97%
```

# Experiment Pass 18

Upgrade to burn 0.19 and CUDA 13.0,

## Experiment: Frequency Pooling and Folded Bass Target

### Command

```bash
cargo run --bin kord --no-default-features --features "cli ml_train ml_remote ml_loader_frequency_pooled ml_target_folded_bass" --release -- -q ml train --backend ws://192.168.229.200:3000 --training-sources kord/samples/captured/ --training-sources kord/samples/slakh --training-sources sim --model-epochs 4
```

### Output

```
Validation dataset metrics (1000 samples):
Inference accuracy: 33.10%
Macro accuracy (24 classes): 94.53%
Macro precision (24 classes): 80.06%
Macro recall (24 classes): 78.01%
Macro F1 (24 classes): 78.69%
Macro PR AUC (24 classes): 83.89%
Sample-wise F1: 85.97%
Validation class insights:
  Lowest precision (10):
    class   7: support  114, predicted  153, precision  58.17%, recall  78.07%, f1  66.67%
    class  11: support   79, predicted   79, precision  60.76%, recall  60.76%, f1  60.76%
    class   5: support   81, predicted  100, precision  61.00%, recall  75.31%, f1  67.40%
    class   1: support   55, predicted   55, precision  63.64%, recall  63.64%, f1  63.64%
    class   2: support   96, predicted   83, precision  65.06%, recall  56.25%, f1  60.34%
    class   0: support   91, predicted   71, precision  69.01%, recall  53.85%, f1  60.49%
    class  10: support   79, predicted   89, precision  69.66%, recall  78.48%, f1  73.81%
    class   9: support  102, predicted   88, precision  71.59%, recall  61.76%, f1  66.32%
    class   4: support  100, predicted  105, precision  72.38%, recall  76.00%, f1  74.15%
    class   8: support   69, predicted   66, precision  74.24%, recall  71.01%, f1  72.59%
  Lowest recall (10):
    class   0: support   91, predicted   71, precision  69.01%, recall  53.85%, f1  60.49%
    class   6: support   75, predicted   53, precision  77.36%, recall  54.67%, f1  64.06%
    class   2: support   96, predicted   83, precision  65.06%, recall  56.25%, f1  60.34%
    class  11: support   79, predicted   79, precision  60.76%, recall  60.76%, f1  60.76%
    class   9: support  102, predicted   88, precision  71.59%, recall  61.76%, f1  66.32%
    class   1: support   55, predicted   55, precision  63.64%, recall  63.64%, f1  63.64%
    class   3: support   59, predicted   44, precision  88.64%, recall  66.10%, f1  75.73%
    class   8: support   69, predicted   66, precision  74.24%, recall  71.01%, f1  72.59%
    class   5: support   81, predicted  100, precision  61.00%, recall  75.31%, f1  67.40%
    class   4: support  100, predicted  105, precision  72.38%, recall  76.00%, f1  74.15%

Captured dataset metrics (176 samples):
Inference accuracy: 35.80%
Macro accuracy (24 classes): 95.53%
Macro precision (24 classes): 83.06%
Macro recall (24 classes): 77.94%
Macro F1 (24 classes): 78.83%
Macro PR AUC (24 classes): 86.53%
Sample-wise F1: 85.48%
Captured class insights:
  Lowest precision (10):
    class  10: support   10, predicted   22, precision  36.36%, recall  80.00%, f1  50.00%
    class   5: support   20, predicted   23, precision  56.52%, recall  65.00%, f1  60.47%
    class   7: support   18, predicted   18, precision  61.11%, recall  61.11%, f1  61.11%
    class   6: support   11, predicted   13, precision  61.54%, recall  72.73%, f1  66.67%
    class  11: support   14, predicted   21, precision  61.90%, recall  92.86%, f1  74.29%
    class   9: support   14, predicted   16, precision  62.50%, recall  71.43%, f1  66.67%
    class   3: support   12, predicted    7, precision  71.43%, recall  41.67%, f1  52.63%
    class  23: support   32, predicted   38, precision  76.32%, recall  90.62%, f1  82.86%
    class   8: support   11, predicted   10, precision  80.00%, recall  72.73%, f1  76.19%
    class  22: support   37, predicted   40, precision  85.00%, recall  91.89%, f1  88.31%
  Lowest recall (10):
    class   2: support   15, predicted    6, precision 100.00%, recall  40.00%, f1  57.14%
    class   3: support   12, predicted    7, precision  71.43%, recall  41.67%, f1  52.63%
    class   0: support   25, predicted   16, precision  93.75%, recall  60.00%, f1  73.17%
    class   7: support   18, predicted   18, precision  61.11%, recall  61.11%, f1  61.11%
    class   4: support   14, predicted   10, precision  90.00%, recall  64.29%, f1  75.00%
    class   5: support   20, predicted   23, precision  56.52%, recall  65.00%, f1  60.47%
    class   9: support   14, predicted   16, precision  62.50%, recall  71.43%, f1  66.67%
    class   6: support   11, predicted   13, precision  61.54%, recall  72.73%, f1  66.67%
    class   8: support   11, predicted   10, precision  80.00%, recall  72.73%, f1  76.19%
    class   1: support   12, predicted   10, precision  90.00%, recall  75.00%, f1  81.82%
```

### Command

```bash
cargo run --bin kord --no-default-features --features "cli ml_train ml_remote ml_loader_note_binned_convolution ml_target_folded_bass" --release -- -q ml train --backend ws://192.168.229.200:3000 --training-sources kord/samples/captured/ --training-sources kord/samples/slakh --training-sources sim --model-epochs 4
```

### Output

```
Validation dataset metrics (1000 samples):
Inference accuracy: 23.50%
Macro accuracy (24 classes): 92.88%
Macro precision (24 classes): 74.85%
Macro recall (24 classes): 76.65%
Macro F1 (24 classes): 75.30%
Macro PR AUC (24 classes): 79.96%
Sample-wise F1: 79.32%
Validation class insights:
  Lowest precision (10):
    class   6: support   70, predicted  124, precision  43.55%, recall  77.14%, f1  55.67%
    class   8: support   56, predicted   72, precision  51.39%, recall  66.07%, f1  57.81%
    class  10: support   69, predicted   86, precision  59.30%, recall  73.91%, f1  65.81%
    class   2: support  100, predicted  117, precision  61.54%, recall  72.00%, f1  66.36%
    class   0: support   96, predicted  128, precision  62.50%, recall  83.33%, f1  71.43%
    class   3: support   65, predicted   59, precision  64.41%, recall  58.46%, f1  61.29%
    class   4: support   90, predicted   87, precision  65.52%, recall  63.33%, f1  64.41%
    class   5: support   82, predicted   85, precision  67.06%, recall  69.51%, f1  68.26%
    class   1: support   67, predicted   69, precision  68.12%, recall  70.15%, f1  69.12%
    class   7: support  104, predicted  107, precision  71.96%, recall  74.04%, f1  72.99%
  Lowest recall (10):
    class   3: support   65, predicted   59, precision  64.41%, recall  58.46%, f1  61.29%
    class   4: support   90, predicted   87, precision  65.52%, recall  63.33%, f1  64.41%
    class   8: support   56, predicted   72, precision  51.39%, recall  66.07%, f1  57.81%
    class   5: support   82, predicted   85, precision  67.06%, recall  69.51%, f1  68.26%
    class   1: support   67, predicted   69, precision  68.12%, recall  70.15%, f1  69.12%
    class  11: support   88, predicted   87, precision  72.41%, recall  71.59%, f1  72.00%
    class   2: support  100, predicted  117, precision  61.54%, recall  72.00%, f1  66.36%
    class   9: support  113, predicted  114, precision  72.81%, recall  73.45%, f1  73.13%
    class  18: support  225, predicted  185, precision  89.73%, recall  73.78%, f1  80.98%
    class  10: support   69, predicted   86, precision  59.30%, recall  73.91%, f1  65.81%

Captured dataset metrics (176 samples):
Inference accuracy: 36.93%
Macro accuracy (24 classes): 95.45%
Macro precision (24 classes): 83.77%
Macro recall (24 classes): 77.37%
Macro F1 (24 classes): 78.17%
Macro PR AUC (24 classes): 89.14%
Sample-wise F1: 85.71%
Captured class insights:
  Lowest precision (10):
    class  10: support   10, predicted   24, precision  37.50%, recall  90.00%, f1  52.94%
    class   6: support   11, predicted   20, precision  45.00%, recall  81.82%, f1  58.06%
    class   8: support   11, predicted   14, precision  57.14%, recall  72.73%, f1  64.00%
    class   7: support   18, predicted   11, precision  63.64%, recall  38.89%, f1  48.28%
    class  11: support   14, predicted   20, precision  65.00%, recall  92.86%, f1  76.47%
    class   9: support   14, predicted   17, precision  70.59%, recall  85.71%, f1  77.42%
    class   5: support   20, predicted   11, precision  72.73%, recall  40.00%, f1  51.61%
    class   3: support   12, predicted    8, precision  75.00%, recall  50.00%, f1  60.00%
    class   0: support   25, predicted   27, precision  81.48%, recall  88.00%, f1  84.62%
    class  22: support   37, predicted   42, precision  85.71%, recall  97.30%, f1  91.14%
  Lowest recall (10):
    class   7: support   18, predicted   11, precision  63.64%, recall  38.89%, f1  48.28%
    class   5: support   20, predicted   11, precision  72.73%, recall  40.00%, f1  51.61%
    class   3: support   12, predicted    8, precision  75.00%, recall  50.00%, f1  60.00%
    class   4: support   14, predicted    7, precision 100.00%, recall  50.00%, f1  66.67%
    class   2: support   15, predicted    9, precision  88.89%, recall  53.33%, f1  66.67%
    class  14: support   36, predicted   20, precision 100.00%, recall  55.56%, f1  71.43%
    class  18: support   30, predicted   20, precision 100.00%, recall  66.67%, f1  80.00%
    class   8: support   11, predicted   14, precision  57.14%, recall  72.73%, f1  64.00%
    class  12: support   44, predicted   33, precision 100.00%, recall  75.00%, f1  85.71%
    class   6: support   11, predicted   20, precision  45.00%, recall  81.82%, f1  58.06%
``` 

# Experiment Pass 19

No change: validating consistency after burn and CUDA upgrade.

## Experiment: Frequency Pooling and Folded Bass Target

### Command

```bash
cargo run --bin kord --no-default-features --features "cli ml_train ml_remote ml_loader_frequency_pooled ml_target_folded_bass" --release -- -q ml train --backend ws://192.168.229.200:3000 --training-sources kord/samples/captured/ --training-sources kord/samples/slakh --training-sources sim --model-epochs 4
```

### Output

```
Validation dataset metrics (1000 samples):
Inference accuracy: 28.90%
Macro accuracy (24 classes): 94.03%
Macro precision (24 classes): 76.47%
Macro recall (24 classes): 79.32%
Macro F1 (24 classes): 77.40%
Macro PR AUC (24 classes): 82.47%
Sample-wise F1: 85.29%
Validation class insights:
  Lowest precision (10):
    class   6: support   65, predicted   86, precision  45.35%, recall  60.00%, f1  51.66%
    class   2: support   86, predicted  132, precision  50.76%, recall  77.91%, f1  61.47%
    class   1: support   52, predicted   67, precision  52.24%, recall  67.31%, f1  58.82%
    class   9: support  107, predicted  119, precision  56.30%, recall  62.62%, f1  59.29%
    class   3: support   63, predicted   77, precision  57.14%, recall  69.84%, f1  62.86%
    class   8: support   61, predicted   80, precision  57.50%, recall  75.41%, f1  65.25%
    class   7: support  117, predicted  157, precision  59.87%, recall  80.34%, f1  68.61%
    class   0: support  108, predicted  127, precision  69.29%, recall  81.48%, f1  74.89%
    class  11: support   92, predicted   95, precision  71.58%, recall  73.91%, f1  72.73%
    class   5: support   77, predicted   69, precision  73.91%, recall  66.23%, f1  69.86%
  Lowest recall (10):
    class   4: support   99, predicted   77, precision  76.62%, recall  59.60%, f1  67.05%
    class   6: support   65, predicted   86, precision  45.35%, recall  60.00%, f1  51.66%
    class  10: support   73, predicted   57, precision  77.19%, recall  60.27%, f1  67.69%
    class   9: support  107, predicted  119, precision  56.30%, recall  62.62%, f1  59.29%
    class   5: support   77, predicted   69, precision  73.91%, recall  66.23%, f1  69.86%
    class   1: support   52, predicted   67, precision  52.24%, recall  67.31%, f1  58.82%
    class   3: support   63, predicted   77, precision  57.14%, recall  69.84%, f1  62.86%
    class  11: support   92, predicted   95, precision  71.58%, recall  73.91%, f1  72.73%
    class   8: support   61, predicted   80, precision  57.50%, recall  75.41%, f1  65.25%
    class   2: support   86, predicted  132, precision  50.76%, recall  77.91%, f1  61.47%

Captured dataset metrics (176 samples):
Inference accuracy: 39.77%
Macro accuracy (24 classes): 95.60%
Macro precision (24 classes): 82.69%
Macro recall (24 classes): 79.06%
Macro F1 (24 classes): 79.11%
Macro PR AUC (24 classes): 87.02%
Sample-wise F1: 85.90%
Captured class insights:
  Lowest precision (10):
    class  10: support   10, predicted   21, precision  38.10%, recall  80.00%, f1  51.61%
    class   6: support   11, predicted   14, precision  57.14%, recall  72.73%, f1  64.00%
    class   8: support   11, predicted   14, precision  57.14%, recall  72.73%, f1  64.00%
    class   9: support   14, predicted   17, precision  58.82%, recall  71.43%, f1  64.52%
    class   3: support   12, predicted   15, precision  66.67%, recall  83.33%, f1  74.07%
    class   7: support   18, predicted   15, precision  66.67%, recall  55.56%, f1  60.61%
    class  11: support   14, predicted   17, precision  76.47%, recall  92.86%, f1  83.87%
    class   0: support   25, predicted   28, precision  78.57%, recall  88.00%, f1  83.02%
    class  13: support   27, predicted   29, precision  79.31%, recall  85.19%, f1  82.14%
    class   2: support   15, predicted   10, precision  80.00%, recall  53.33%, f1  64.00%
  Lowest recall (10):
    class   4: support   14, predicted    6, precision 100.00%, recall  42.86%, f1  60.00%
    class   5: support   20, predicted   10, precision  90.00%, recall  45.00%, f1  60.00%
    class   2: support   15, predicted   10, precision  80.00%, recall  53.33%, f1  64.00%
    class   7: support   18, predicted   15, precision  66.67%, recall  55.56%, f1  60.61%
    class  14: support   36, predicted   23, precision 100.00%, recall  63.89%, f1  77.97%
    class   9: support   14, predicted   17, precision  58.82%, recall  71.43%, f1  64.52%
    class   6: support   11, predicted   14, precision  57.14%, recall  72.73%, f1  64.00%
    class   8: support   11, predicted   14, precision  57.14%, recall  72.73%, f1  64.00%
    class   1: support   12, predicted   10, precision  90.00%, recall  75.00%, f1  81.82%
    class  12: support   44, predicted   33, precision 100.00%, recall  75.00%, f1  85.71%
```

# Experiment Pass 19

No change: validating consistency after burn and CUDA upgrade.

## Experiment: Frequency Pooling and Folded Bass Target

### Command

```bash
$ cd kord
$ cargo run --bin kord --no-default-features --features "cli ml_train ml_train_precision_fp32 ml_store_precision_full ml_tch ml_loader_frequency_pooled ml_target_folded_bass" --release -- -q ml train --backend tch --training-sources samples/captured --training-sources samples/slakh --training-sources sim --noise-asset-root samples/noise --destination model --model-epochs 16
```

### Output

```
Validation dataset metrics (1000 samples):
Inference accuracy: 30.50%
Macro accuracy (24 classes): 94.47%
Macro precision (24 classes): 77.85%
Macro recall (24 classes): 81.29%
Macro F1 (24 classes): 79.22%
Macro PR AUC (24 classes): 84.75%
Sample-wise F1: 85.97%
Validation class insights:
  Lowest precision (10):
    class   9: support  102, predicted  149, precision  51.68%, recall  75.49%, f1  61.35%
    class   1: support   47, predicted   70, precision  52.86%, recall  78.72%, f1  63.25%
    class   2: support   95, predicted  106, precision  57.55%, recall  64.21%, f1  60.70%
    class   3: support   48, predicted   63, precision  58.73%, recall  77.08%, f1  66.67%
    class  11: support   76, predicted   89, precision  61.80%, recall  72.37%, f1  66.67%
    class   4: support   87, predicted  106, precision  63.21%, recall  77.01%, f1  69.43%
    class   0: support  110, predicted  113, precision  69.03%, recall  70.91%, f1  69.96%
    class   7: support  120, predicted  136, precision  69.12%, recall  78.33%, f1  73.44%
    class   5: support  104, predicted   99, precision  69.70%, recall  66.35%, f1  67.98%
    class   6: support   73, predicted   73, precision  69.86%, recall  69.86%, f1  69.86%
  Lowest recall (10):
    class   2: support   95, predicted  106, precision  57.55%, recall  64.21%, f1  60.70%
    class   5: support  104, predicted   99, precision  69.70%, recall  66.35%, f1  67.98%
    class  10: support   66, predicted   61, precision  75.41%, recall  69.70%, f1  72.44%
    class   6: support   73, predicted   73, precision  69.86%, recall  69.86%, f1  69.86%
    class   0: support  110, predicted  113, precision  69.03%, recall  70.91%, f1  69.96%
    class  11: support   76, predicted   89, precision  61.80%, recall  72.37%, f1  66.67%
    class   8: support   72, predicted   74, precision  71.62%, recall  73.61%, f1  72.60%
    class   9: support  102, predicted  149, precision  51.68%, recall  75.49%, f1  61.35%
    class   4: support   87, predicted  106, precision  63.21%, recall  77.01%, f1  69.43%
    class   3: support   48, predicted   63, precision  58.73%, recall  77.08%, f1  66.67%

Captured dataset metrics (176 samples):
Inference accuracy: 38.64%
Macro accuracy (24 classes): 95.31%
Macro precision (24 classes): 80.21%
Macro recall (24 classes): 81.37%
Macro F1 (24 classes): 79.20%
Macro PR AUC (24 classes): 87.11%
Sample-wise F1: 85.17%
Captured class insights:
  Lowest precision (10):
    class   9: support   14, predicted   25, precision  40.00%, recall  71.43%, f1  51.28%
    class  10: support   10, predicted   16, precision  43.75%, recall  70.00%, f1  53.85%
    class  11: support   14, predicted   24, precision  54.17%, recall  92.86%, f1  68.42%
    class   6: support   11, predicted   16, precision  56.25%, recall  81.82%, f1  66.67%
    class   5: support   20, predicted   23, precision  56.52%, recall  65.00%, f1  60.47%
    class   7: support   18, predicted   14, precision  57.14%, recall  44.44%, f1  50.00%
    class   8: support   11, predicted   13, precision  69.23%, recall  81.82%, f1  75.00%
    class   3: support   12, predicted   14, precision  71.43%, recall  83.33%, f1  76.92%
    class   4: support   14, predicted   15, precision  73.33%, recall  78.57%, f1  75.86%
    class  22: support   37, predicted   48, precision  75.00%, recall  97.30%, f1  84.71%
  Lowest recall (10):
    class   2: support   15, predicted    6, precision 100.00%, recall  40.00%, f1  57.14%
    class   7: support   18, predicted   14, precision  57.14%, recall  44.44%, f1  50.00%
    class   5: support   20, predicted   23, precision  56.52%, recall  65.00%, f1  60.47%
    class  10: support   10, predicted   16, precision  43.75%, recall  70.00%, f1  53.85%
    class   9: support   14, predicted   25, precision  40.00%, recall  71.43%, f1  51.28%
    class  12: support   44, predicted   34, precision 100.00%, recall  77.27%, f1  87.18%
    class  13: support   27, predicted   21, precision 100.00%, recall  77.78%, f1  87.50%
    class   4: support   14, predicted   15, precision  73.33%, recall  78.57%, f1  75.86%
    class  15: support   39, predicted   31, precision 100.00%, recall  79.49%, f1  88.57%
    class  18: support   30, predicted   24, precision 100.00%, recall  80.00%, f1  88.89%
```

# Experiment Pass 20

Classification now uses proper loss for the folded bass target.

## Experiment: Frequency Pooling and Folded Bass Target

### Command

```bash
$ cd kord
$ cargo run --bin kord --no-default-features --features "cli ml_train ml_train_precision_fp32 ml_store_precision_full ml_tch ml_loader_frequency_pooled ml_target_folded_bass" --release -- -q ml train --backend tch --training-sources samples/captured --training-sources samples/slakh --training-sources sim --noise-asset-root samples/noise --destination model --model-epochs 16
```

### Output

```
Validation dataset metrics (1000 samples):
Inference accuracy: 39.60%
Macro accuracy (24 classes): 94.31%
Macro precision (24 classes): 79.04%
Macro recall (24 classes): 77.62%
Macro F1 (24 classes): 78.22%
Macro PR AUC (24 classes): 85.28%
Sample-wise F1: 85.71%
Validation class insights:
  Lowest precision (10):
    class  10: support   65, predicted   58, precision  62.07%, recall  55.38%, f1  58.54%
    class   5: support   84, predicted   88, precision  63.64%, recall  66.67%, f1  65.12%
    class   7: support  133, predicted  144, precision  63.89%, recall  69.17%, f1  66.43%
    class   3: support   65, predicted   67, precision  64.18%, recall  66.15%, f1  65.15%
    class   4: support   88, predicted   97, precision  64.95%, recall  71.59%, f1  68.11%
    class   0: support   91, predicted   92, precision  65.22%, recall  65.93%, f1  65.57%
    class   1: support   42, predicted   36, precision  66.67%, recall  57.14%, f1  61.54%
    class  11: support   80, predicted   78, precision  69.23%, recall  67.50%, f1  68.35%
    class   9: support  118, predicted  116, precision  69.83%, recall  68.64%, f1  69.23%
    class   2: support  109, predicted  104, precision  70.19%, recall  66.97%, f1  68.54%
  Lowest recall (10):
    class  10: support   65, predicted   58, precision  62.07%, recall  55.38%, f1  58.54%
    class   1: support   42, predicted   36, precision  66.67%, recall  57.14%, f1  61.54%
    class   6: support   66, predicted   54, precision  75.93%, recall  62.12%, f1  68.33%
    class   0: support   91, predicted   92, precision  65.22%, recall  65.93%, f1  65.57%
    class   3: support   65, predicted   67, precision  64.18%, recall  66.15%, f1  65.15%
    class   5: support   84, predicted   88, precision  63.64%, recall  66.67%, f1  65.12%
    class   2: support  109, predicted  104, precision  70.19%, recall  66.97%, f1  68.54%
    class  11: support   80, predicted   78, precision  69.23%, recall  67.50%, f1  68.35%
    class   9: support  118, predicted  116, precision  69.83%, recall  68.64%, f1  69.23%
    class   7: support  133, predicted  144, precision  63.89%, recall  69.17%, f1  66.43%

Captured dataset metrics (176 samples):
Inference accuracy: 42.61%
Macro accuracy (24 classes): 95.34%
Macro precision (24 classes): 81.36%
Macro recall (24 classes): 80.23%
Macro F1 (24 classes): 79.51%
Macro PR AUC (24 classes): 87.77%
Sample-wise F1: 85.29%
Captured class insights:
  Lowest precision (10):
    class  10: support   10, predicted   16, precision  50.00%, recall  80.00%, f1  61.54%
    class   5: support   20, predicted   27, precision  55.56%, recall  75.00%, f1  63.83%
    class  11: support   14, predicted   21, precision  61.90%, recall  92.86%, f1  74.29%
    class   9: support   14, predicted   16, precision  62.50%, recall  71.43%, f1  66.67%
    class   8: support   11, predicted   14, precision  64.29%, recall  81.82%, f1  72.00%
    class   6: support   11, predicted   12, precision  66.67%, recall  72.73%, f1  69.57%
    class  23: support   32, predicted   43, precision  69.77%, recall  93.75%, f1  80.00%
    class   2: support   15, predicted    8, precision  75.00%, recall  40.00%, f1  52.17%
    class  13: support   27, predicted   30, precision  76.67%, recall  85.19%, f1  80.70%
    class   7: support   18, predicted   13, precision  76.92%, recall  55.56%, f1  64.52%
  Lowest recall (10):
    class   2: support   15, predicted    8, precision  75.00%, recall  40.00%, f1  52.17%
    class   7: support   18, predicted   13, precision  76.92%, recall  55.56%, f1  64.52%
    class   0: support   25, predicted   17, precision  94.12%, recall  64.00%, f1  76.19%
    class   1: support   12, predicted    8, precision 100.00%, recall  66.67%, f1  80.00%
    class   3: support   12, predicted   10, precision  80.00%, recall  66.67%, f1  72.73%
    class   9: support   14, predicted   16, precision  62.50%, recall  71.43%, f1  66.67%
    class   6: support   11, predicted   12, precision  66.67%, recall  72.73%, f1  69.57%
    class  18: support   30, predicted   22, precision 100.00%, recall  73.33%, f1  84.62%
    class   5: support   20, predicted   27, precision  55.56%, recall  75.00%, f1  63.83%
    class  12: support   44, predicted   34, precision  97.06%, recall  75.00%, f1  84.62%
```

# Experiment Pass 21

Bass targets are now mutually exclusive.

## Experiment: Frequency Pooling and Folded Bass Target

### Command

```bash
$ cd kord
$ cargo run --bin kord --no-default-features --features "cli ml_train ml_tch ml_train_precision_fp32 ml_store_precision_full ml_loader_frequency_pooled ml_target_folded_bass" --release -- -q ml train --backend tch --training-sources samples/captured --training-sources samples/slakh --training-sources sim --noise-asset-root samples/noise --destination model --model-epochs 16
```

### Output

```
Validation dataset metrics (1000 samples):
Inference accuracy: 41.30%
Macro accuracy (24 classes): 94.61%
Macro precision (24 classes): 80.24%
Macro recall (24 classes): 78.60%
Macro F1 (24 classes): 79.18%
Macro PR AUC (24 classes): 85.25%
Sample-wise F1: 86.47%
Validation class insights:
  Lowest precision (10):
    class   7: support  117, predicted  143, precision  58.74%, recall  71.79%, f1  64.62%
    class   2: support  100, predicted   98, precision  62.24%, recall  61.00%, f1  61.62%
    class   8: support   51, predicted   61, precision  62.30%, recall  74.51%, f1  67.86%
    class  11: support   68, predicted   79, precision  63.29%, recall  73.53%, f1  68.03%
    class   1: support   60, predicted   71, precision  66.20%, recall  78.33%, f1  71.76%
    class   0: support  106, predicted   96, precision  67.71%, recall  61.32%, f1  64.36%
    class   5: support   83, predicted   82, precision  68.29%, recall  67.47%, f1  67.88%
    class   4: support   95, predicted   98, precision  69.39%, recall  71.58%, f1  70.47%
    class   9: support  115, predicted  102, precision  71.57%, recall  63.48%, f1  67.28%
    class   3: support   57, predicted   53, precision  73.58%, recall  68.42%, f1  70.91%
  Lowest recall (10):
    class   2: support  100, predicted   98, precision  62.24%, recall  61.00%, f1  61.62%
    class   0: support  106, predicted   96, precision  67.71%, recall  61.32%, f1  64.36%
    class   6: support   62, predicted   48, precision  81.25%, recall  62.90%, f1  70.91%
    class   9: support  115, predicted  102, precision  71.57%, recall  63.48%, f1  67.28%
    class  10: support   86, predicted   69, precision  79.71%, recall  63.95%, f1  70.97%
    class   5: support   83, predicted   82, precision  68.29%, recall  67.47%, f1  67.88%
    class   3: support   57, predicted   53, precision  73.58%, recall  68.42%, f1  70.91%
    class   4: support   95, predicted   98, precision  69.39%, recall  71.58%, f1  70.47%
    class   7: support  117, predicted  143, precision  58.74%, recall  71.79%, f1  64.62%
    class  11: support   68, predicted   79, precision  63.29%, recall  73.53%, f1  68.03%

Captured dataset metrics (176 samples):
Inference accuracy: 47.73%
Macro accuracy (24 classes): 95.71%
Macro precision (24 classes): 83.84%
Macro recall (24 classes): 79.68%
Macro F1 (24 classes): 80.38%
Macro PR AUC (24 classes): 88.29%
Sample-wise F1: 86.43%
Captured class insights:
  Lowest precision (10):
    class  10: support   10, predicted   19, precision  47.37%, recall  90.00%, f1  62.07%
    class   8: support   11, predicted   17, precision  52.94%, recall  81.82%, f1  64.29%
    class  11: support   14, predicted   24, precision  54.17%, recall  92.86%, f1  68.42%
    class   9: support   14, predicted   17, precision  58.82%, recall  71.43%, f1  64.52%
    class   6: support   11, predicted   11, precision  72.73%, recall  72.73%, f1  72.73%
    class   7: support   18, predicted   11, precision  72.73%, recall  44.44%, f1  55.17%
    class  23: support   32, predicted   36, precision  75.00%, recall  84.38%, f1  79.41%
    class   3: support   12, predicted    9, precision  77.78%, recall  58.33%, f1  66.67%
    class   5: support   20, predicted   14, precision  78.57%, recall  55.00%, f1  64.71%
    class   2: support   15, predicted   10, precision  80.00%, recall  53.33%, f1  64.00%
  Lowest recall (10):
    class   7: support   18, predicted   11, precision  72.73%, recall  44.44%, f1  55.17%
    class   2: support   15, predicted   10, precision  80.00%, recall  53.33%, f1  64.00%
    class   5: support   20, predicted   14, precision  78.57%, recall  55.00%, f1  64.71%
    class   3: support   12, predicted    9, precision  77.78%, recall  58.33%, f1  66.67%
    class   9: support   14, predicted   17, precision  58.82%, recall  71.43%, f1  64.52%
    class   6: support   11, predicted   11, precision  72.73%, recall  72.73%, f1  72.73%
    class   0: support   25, predicted   21, precision  90.48%, recall  76.00%, f1  82.61%
    class  12: support   44, predicted   34, precision 100.00%, recall  77.27%, f1  87.18%
    class   4: support   14, predicted   13, precision  84.62%, recall  78.57%, f1  81.48%
    class  15: support   39, predicted   31, precision 100.00%, recall  79.49%, f1  88.57%
```

# Experiment Pass 22

Trying the non-pooled loader again with the new improvements.

## Experiment: Mel and Folded Bass Target

### Command

```bash
$ cd kord
$ cargo run --bin kord --no-default-features --features "cli ml_train ml_tch ml_train_precision_fp32 ml_store_precision_full ml_loader_mel ml_target_folded_bass" --release -- -q ml train --backend tch --training-sources samples/captured --training-sources samples/slakh --training-sources sim --noise-asset-root samples/noise --destination model --model-epochs 16
```

### Output

```
Validation dataset metrics (1000 samples):
Inference accuracy: 51.10%
Macro accuracy (24 classes): 95.51%
Macro precision (24 classes): 82.80%
Macro recall (24 classes): 82.48%
Macro F1 (24 classes): 82.55%
Macro PR AUC (24 classes): 88.43%
Sample-wise F1: 88.91%
Validation class insights:
  Lowest precision (10):
    class   7: support   95, predicted  102, precision  63.73%, recall  68.42%, f1  65.99%
    class   9: support  103, predicted  125, precision  64.00%, recall  77.67%, f1  70.18%
    class   8: support   60, predicted   59, precision  69.49%, recall  68.33%, f1  68.91%
    class   0: support   99, predicted  104, precision  70.19%, recall  73.74%, f1  71.92%
    class   5: support   92, predicted   83, precision  72.29%, recall  65.22%, f1  68.57%
    class   4: support   94, predicted   87, precision  72.41%, recall  67.02%, f1  69.61%
    class  11: support   98, predicted   92, precision  75.00%, recall  70.41%, f1  72.63%
    class   3: support   65, predicted   70, precision  75.71%, recall  81.54%, f1  78.52%
    class   2: support   93, predicted   89, precision  76.40%, recall  73.12%, f1  74.73%
    class   6: support   63, predicted   58, precision  77.59%, recall  71.43%, f1  74.38%
  Lowest recall (10):
    class   5: support   92, predicted   83, precision  72.29%, recall  65.22%, f1  68.57%
    class   4: support   94, predicted   87, precision  72.41%, recall  67.02%, f1  69.61%
    class   8: support   60, predicted   59, precision  69.49%, recall  68.33%, f1  68.91%
    class   7: support   95, predicted  102, precision  63.73%, recall  68.42%, f1  65.99%
    class  11: support   98, predicted   92, precision  75.00%, recall  70.41%, f1  72.63%
    class   6: support   63, predicted   58, precision  77.59%, recall  71.43%, f1  74.38%
    class   2: support   93, predicted   89, precision  76.40%, recall  73.12%, f1  74.73%
    class   0: support   99, predicted  104, precision  70.19%, recall  73.74%, f1  71.92%
    class   1: support   63, predicted   60, precision  78.33%, recall  74.60%, f1  76.42%
    class   9: support  103, predicted  125, precision  64.00%, recall  77.67%, f1  70.18%

Captured dataset metrics (176 samples):
Inference accuracy: 61.93%
Macro accuracy (24 classes): 96.92%
Macro precision (24 classes): 87.20%
Macro recall (24 classes): 84.97%
Macro F1 (24 classes): 84.80%
Macro PR AUC (24 classes): 90.48%
Sample-wise F1: 90.42%
Captured class insights:
  Lowest precision (10):
    class  10: support   10, predicted   17, precision  52.94%, recall  90.00%, f1  66.67%
    class   9: support   14, predicted   22, precision  59.09%, recall  92.86%, f1  72.22%
    class   8: support   11, predicted   12, precision  66.67%, recall  72.73%, f1  69.57%
    class   5: support   20, predicted   19, precision  68.42%, recall  65.00%, f1  66.67%
    class   0: support   25, predicted   31, precision  74.19%, recall  92.00%, f1  82.14%
    class  11: support   14, predicted   17, precision  76.47%, recall  92.86%, f1  83.87%
    class   7: support   18, predicted   11, precision  81.82%, recall  50.00%, f1  62.07%
    class   3: support   12, predicted   12, precision  83.33%, recall  83.33%, f1  83.33%
    class  22: support   37, predicted   43, precision  86.05%, recall 100.00%, f1  92.50%
    class   2: support   15, predicted    8, precision  87.50%, recall  46.67%, f1  60.87%
  Lowest recall (10):
    class   2: support   15, predicted    8, precision  87.50%, recall  46.67%, f1  60.87%
    class   7: support   18, predicted   11, precision  81.82%, recall  50.00%, f1  62.07%
    class   1: support   12, predicted    7, precision 100.00%, recall  58.33%, f1  73.68%
    class   5: support   20, predicted   19, precision  68.42%, recall  65.00%, f1  66.67%
    class   4: support   14, predicted   10, precision 100.00%, recall  71.43%, f1  83.33%
    class   8: support   11, predicted   12, precision  66.67%, recall  72.73%, f1  69.57%
    class   6: support   11, predicted   10, precision  90.00%, recall  81.82%, f1  85.71%
    class   3: support   12, predicted   12, precision  83.33%, recall  83.33%, f1  83.33%
    class  12: support   44, predicted   38, precision 100.00%, recall  86.36%, f1  92.68%
    class  10: support   10, predicted   17, precision  52.94%, recall  90.00%, f1  66.67%
```

# Experiment Pass 23

Trying the mel loader with full folding with new improvements.

## Experiment: Mel and Folded Bass Target

### Command

```bash
$ cd kord
$ cargo run --bin kord --no-default-features --features "cli ml_train ml_tch ml_train_precision_fp32 ml_store_precision_full ml_loader_mel ml_target_folded" --release -- -q ml train --backend tch --training-sources samples/captured --training-sources samples/slakh --training-sources sim --noise-asset-root samples/noise --destination model --model-epochs 16
```

### Output

```
Validation dataset metrics (1000 samples):
Inference accuracy: 65.10%
Macro accuracy (12 classes): 95.66%
Macro precision (12 classes): 92.53%
Macro recall (12 classes): 92.30%
Macro F1 (12 classes): 92.37%
Macro PR AUC (12 classes): 96.85%
Sample-wise F1: 93.19%
Validation class insights:
  Lowest precision (10):
    class  10: support  373, predicted  403, precision  88.09%, recall  95.17%, f1  91.49%
    class   1: support  255, predicted  263, precision  90.11%, recall  92.94%, f1  91.51%
    class   5: support  355, predicted  379, precision  90.50%, recall  96.62%, f1  93.46%
    class   6: support  228, predicted  239, precision  90.79%, recall  95.18%, f1  92.93%
    class   8: support  319, predicted  301, precision  92.36%, recall  87.15%, f1  89.68%
    class   3: support  359, predicted  359, precision  93.04%, recall  93.04%, f1  93.04%
    class  11: support  201, predicted  197, precision  93.40%, recall  91.54%, f1  92.46%
    class   7: support  299, predicted  293, precision  93.52%, recall  91.64%, f1  92.57%
    class   0: support  342, predicted  342, precision  93.57%, recall  93.57%, f1  93.57%
    class   9: support  213, predicted  204, precision  94.61%, recall  90.61%, f1  92.57%
  Lowest recall (10):
    class   8: support  319, predicted  301, precision  92.36%, recall  87.15%, f1  89.68%
    class   2: support  254, predicted  239, precision  94.98%, recall  89.37%, f1  92.09%
    class   9: support  213, predicted  204, precision  94.61%, recall  90.61%, f1  92.57%
    class   4: support  207, predicted  197, precision  95.43%, recall  90.82%, f1  93.07%
    class  11: support  201, predicted  197, precision  93.40%, recall  91.54%, f1  92.46%
    class   7: support  299, predicted  293, precision  93.52%, recall  91.64%, f1  92.57%
    class   1: support  255, predicted  263, precision  90.11%, recall  92.94%, f1  91.51%
    class   3: support  359, predicted  359, precision  93.04%, recall  93.04%, f1  93.04%
    class   0: support  342, predicted  342, precision  93.57%, recall  93.57%, f1  93.57%
    class  10: support  373, predicted  403, precision  88.09%, recall  95.17%, f1  91.49%

Captured dataset metrics (176 samples):
Inference accuracy: 82.39%
Macro accuracy (12 classes): 98.25%
Macro precision (12 classes): 95.46%
Macro recall (12 classes): 96.67%
Macro F1 (12 classes): 95.83%
Macro PR AUC (12 classes): 99.61%
Sample-wise F1: 95.79%
Captured class insights:
  Lowest precision (10):
    class  10: support   37, predicted   47, precision  78.72%, recall 100.00%, f1  88.10%
    class  11: support   32, predicted   37, precision  83.78%, recall  96.88%, f1  89.86%
    class   8: support   36, predicted   39, precision  92.31%, recall 100.00%, f1  96.00%
    class   1: support   27, predicted   29, precision  93.10%, recall 100.00%, f1  96.43%
    class   5: support   42, predicted   41, precision  97.56%, recall  95.24%, f1  96.39%
    class   0: support   44, predicted   40, precision 100.00%, recall  90.91%, f1  95.24%
    class   2: support   36, predicted   31, precision 100.00%, recall  86.11%, f1  92.54%
    class   3: support   39, predicted   39, precision 100.00%, recall 100.00%, f1 100.00%
    class   4: support   32, predicted   32, precision 100.00%, recall 100.00%, f1 100.00%
    class   6: support   30, predicted   29, precision 100.00%, recall  96.67%, f1  98.31%
  Lowest recall (10):
    class   2: support   36, predicted   31, precision 100.00%, recall  86.11%, f1  92.54%
    class   0: support   44, predicted   40, precision 100.00%, recall  90.91%, f1  95.24%
    class   9: support   35, predicted   33, precision 100.00%, recall  94.29%, f1  97.06%
    class   5: support   42, predicted   41, precision  97.56%, recall  95.24%, f1  96.39%
    class   6: support   30, predicted   29, precision 100.00%, recall  96.67%, f1  98.31%
    class  11: support   32, predicted   37, precision  83.78%, recall  96.88%, f1  89.86%
    class   1: support   27, predicted   29, precision  93.10%, recall 100.00%, f1  96.43%
    class   3: support   39, predicted   39, precision 100.00%, recall 100.00%, f1 100.00%
    class   4: support   32, predicted   32, precision 100.00%, recall 100.00%, f1 100.00%
    class   7: support   46, predicted   46, precision 100.00%, recall 100.00%, f1 100.00%
```

# Experiment Pass 24

Try "splitting" bass into its own class using mel.

## Experiment: Mel and Folded Bass Target

### Command

```bash
$ cd kord
$ cargo run --bin kord --no-default-features --features "cli ml_train ml_tch ml_train_precision_fp32 ml_store_precision_full ml_loader_mel ml_target_folded_bass" --release -- -q ml train --backend tch --training-sources samples/captured --training-sources samples/slakh --training-sources sim --noise-asset-root samples/noise --destination model --model-epochs 16
```

### Output

```
Validation dataset metrics (1000 samples):
Inference accuracy: 50.20%
Macro accuracy (24 classes): 95.49%
Macro precision (24 classes): 82.91%
Macro recall (24 classes): 82.66%
Macro F1 (24 classes): 82.60%
Macro PR AUC (24 classes): 89.46%
Sample-wise F1: 88.91%
Validation class insights:
  Lowest precision (10):
    class   4: support   97, predicted  108, precision  65.74%, recall  73.20%, f1  69.27%
    class   2: support   87, predicted   91, precision  65.93%, recall  68.97%, f1  67.42%
    class   0: support   96, predicted  114, precision  66.67%, recall  79.17%, f1  72.38%
    class   7: support  109, predicted  109, precision  68.81%, recall  68.81%, f1  68.81%
    class   1: support   45, predicted   49, precision  69.39%, recall  75.56%, f1  72.34%
    class  10: support   61, predicted   67, precision  71.64%, recall  78.69%, f1  75.00%
    class   6: support   70, predicted   78, precision  73.08%, recall  81.43%, f1  77.03%
    class   8: support   67, predicted   67, precision  76.12%, recall  76.12%, f1  76.12%
    class   5: support   76, predicted   71, precision  77.46%, recall  72.37%, f1  74.83%
    class   9: support  145, predicted  131, precision  79.39%, recall  71.72%, f1  75.36%
  Lowest recall (10):
    class  11: support   72, predicted   51, precision  82.35%, recall  58.33%, f1  68.29%
    class   7: support  109, predicted  109, precision  68.81%, recall  68.81%, f1  68.81%
    class   2: support   87, predicted   91, precision  65.93%, recall  68.97%, f1  67.42%
    class   9: support  145, predicted  131, precision  79.39%, recall  71.72%, f1  75.36%
    class   5: support   76, predicted   71, precision  77.46%, recall  72.37%, f1  74.83%
    class   4: support   97, predicted  108, precision  65.74%, recall  73.20%, f1  69.27%
    class   3: support   75, predicted   64, precision  87.50%, recall  74.67%, f1  80.58%
    class   1: support   45, predicted   49, precision  69.39%, recall  75.56%, f1  72.34%
    class   8: support   67, predicted   67, precision  76.12%, recall  76.12%, f1  76.12%
    class  10: support   61, predicted   67, precision  71.64%, recall  78.69%, f1  75.00%

Captured dataset metrics (176 samples):
Inference accuracy: 60.23%
Macro accuracy (24 classes): 96.85%
Macro precision (24 classes): 85.20%
Macro recall (24 classes): 84.71%
Macro F1 (24 classes): 84.12%
Macro PR AUC (24 classes): 90.78%
Sample-wise F1: 90.51%
Captured class insights:
  Lowest precision (10):
    class  10: support   10, predicted   19, precision  42.11%, recall  80.00%, f1  55.17%
    class   9: support   14, predicted   21, precision  61.90%, recall  92.86%, f1  74.29%
    class   2: support   15, predicted   11, precision  63.64%, recall  46.67%, f1  53.85%
    class   5: support   20, predicted   18, precision  66.67%, recall  60.00%, f1  63.16%
    class   0: support   25, predicted   30, precision  73.33%, recall  88.00%, f1  80.00%
    class   8: support   11, predicted   12, precision  75.00%, recall  81.82%, f1  78.26%
    class   7: support   18, predicted   13, precision  76.92%, recall  55.56%, f1  64.52%
    class   3: support   12, predicted    9, precision  77.78%, recall  58.33%, f1  66.67%
    class   6: support   11, predicted   10, precision  80.00%, recall  72.73%, f1  76.19%
    class  23: support   32, predicted   40, precision  80.00%, recall 100.00%, f1  88.89%
  Lowest recall (10):
    class   2: support   15, predicted   11, precision  63.64%, recall  46.67%, f1  53.85%
    class   7: support   18, predicted   13, precision  76.92%, recall  55.56%, f1  64.52%
    class   3: support   12, predicted    9, precision  77.78%, recall  58.33%, f1  66.67%
    class   5: support   20, predicted   18, precision  66.67%, recall  60.00%, f1  63.16%
    class   6: support   11, predicted   10, precision  80.00%, recall  72.73%, f1  76.19%
    class   1: support   12, predicted    9, precision 100.00%, recall  75.00%, f1  85.71%
    class   4: support   14, predicted   12, precision  91.67%, recall  78.57%, f1  84.62%
    class  11: support   14, predicted   12, precision  91.67%, recall  78.57%, f1  84.62%
    class  10: support   10, predicted   19, precision  42.11%, recall  80.00%, f1  55.17%
    class   8: support   11, predicted   12, precision  75.00%, recall  81.82%, f1  78.26%
```

# Experiment Pass 25

No change: driver upgrade.

## Experiment: Mel and Folded Bass Target

### Command

```bash
$ cd kord
$ cargo run --bin kord --no-default-features --features "cli ml_train ml_tch ml_train_precision_fp32 ml_store_precision_full ml_loader_mel ml_target_folded_bass" --release -- -q ml train --backend tch --training-sources samples/captured --training-sources samples/slakh --training-sources sim --noise-asset-root samples/noise --destination model --model-epochs 16
```

### Output

```
Validation dataset metrics (1000 samples):
Inference accuracy: 51.40%
Macro accuracy (24 classes): 95.66%
Macro precision (24 classes): 83.81%
Macro recall (24 classes): 83.17%
Macro F1 (24 classes): 83.33%
Macro PR AUC (24 classes): 90.03%
Sample-wise F1: 89.39%
Validation class insights:
  Lowest precision (10):
    class   0: support   87, predicted  103, precision  67.96%, recall  80.46%, f1  73.68%
    class   4: support   93, predicted  107, precision  68.22%, recall  78.49%, f1  73.00%
    class   8: support   53, predicted   57, precision  70.18%, recall  75.47%, f1  72.73%
    class   9: support  110, predicted  115, precision  70.43%, recall  73.64%, f1  72.00%
    class   2: support   98, predicted   87, precision  73.56%, recall  65.31%, f1  69.19%
    class   7: support  117, predicted  114, precision  74.56%, recall  72.65%, f1  73.59%
    class   3: support   49, predicted   49, precision  75.51%, recall  75.51%, f1  75.51%
    class  10: support   80, predicted   86, precision  76.74%, recall  82.50%, f1  79.52%
    class   1: support   75, predicted   78, precision  79.49%, recall  82.67%, f1  81.05%
    class   5: support   88, predicted   79, precision  79.75%, recall  71.59%, f1  75.45%
  Lowest recall (10):
    class   6: support   73, predicted   55, precision  81.82%, recall  61.64%, f1  70.31%
    class   2: support   98, predicted   87, precision  73.56%, recall  65.31%, f1  69.19%
    class   5: support   88, predicted   79, precision  79.75%, recall  71.59%, f1  75.45%
    class   7: support  117, predicted  114, precision  74.56%, recall  72.65%, f1  73.59%
    class   9: support  110, predicted  115, precision  70.43%, recall  73.64%, f1  72.00%
    class  11: support   77, predicted   70, precision  82.86%, recall  75.32%, f1  78.91%
    class   8: support   53, predicted   57, precision  70.18%, recall  75.47%, f1  72.73%
    class   3: support   49, predicted   49, precision  75.51%, recall  75.51%, f1  75.51%
    class   4: support   93, predicted  107, precision  68.22%, recall  78.49%, f1  73.00%
    class   0: support   87, predicted  103, precision  67.96%, recall  80.46%, f1  73.68%

Captured dataset metrics (176 samples):
Inference accuracy: 65.34%
Macro accuracy (24 classes): 97.16%
Macro precision (24 classes): 89.04%
Macro recall (24 classes): 86.51%
Macro F1 (24 classes): 86.35%
Macro PR AUC (24 classes): 89.40%
Sample-wise F1: 91.38%
Captured class insights:
  Lowest precision (10):
    class  10: support   10, predicted   19, precision  47.37%, recall  90.00%, f1  62.07%
    class   8: support   11, predicted   20, precision  55.00%, recall 100.00%, f1  70.97%
    class   9: support   14, predicted   18, precision  66.67%, recall  85.71%, f1  75.00%
    class   5: support   20, predicted   20, precision  70.00%, recall  70.00%, f1  70.00%
    class   3: support   12, predicted   12, precision  83.33%, recall  83.33%, f1  83.33%
    class   0: support   25, predicted   25, precision  84.00%, recall  84.00%, f1  84.00%
    class  23: support   32, predicted   38, precision  84.21%, recall 100.00%, f1  91.43%
    class  22: support   37, predicted   42, precision  88.10%, recall 100.00%, f1  93.67%
    class   6: support   11, predicted   10, precision  90.00%, recall  81.82%, f1  85.71%
    class   7: support   18, predicted   11, precision  90.91%, recall  55.56%, f1  68.97%
  Lowest recall (10):
    class   2: support   15, predicted    7, precision 100.00%, recall  46.67%, f1  63.64%
    class   7: support   18, predicted   11, precision  90.91%, recall  55.56%, f1  68.97%
    class   5: support   20, predicted   20, precision  70.00%, recall  70.00%, f1  70.00%
    class   1: support   12, predicted    9, precision 100.00%, recall  75.00%, f1  85.71%
    class   4: support   14, predicted   11, precision 100.00%, recall  78.57%, f1  88.00%
    class   6: support   11, predicted   10, precision  90.00%, recall  81.82%, f1  85.71%
    class   3: support   12, predicted   12, precision  83.33%, recall  83.33%, f1  83.33%
    class   0: support   25, predicted   25, precision  84.00%, recall  84.00%, f1  84.00%
    class   9: support   14, predicted   18, precision  66.67%, recall  85.71%, f1  75.00%
    class  18: support   30, predicted   26, precision 100.00%, recall  86.67%, f1  92.86%
```

# Experiment Pass 26

Split the bass class away from the notes class to train separately.

## Experiment: Mel and Folded Bass Target

### Command

```bash
$ cd kord
$ cargo run --bin kord --no-default-features --features "cli ml_train ml_tch ml_train_precision_fp32 ml_store_precision_full ml_loader_mel ml_target_folded_bass" --release -- -q ml train --backend tch --training-sources samples/captured --training-sources samples/slakh --training-sources sim --noise-asset-root samples/noise --destination model --model-epochs 16
```

### Output

```
Validation dataset metrics (1000 samples):
Inference accuracy: 49.70%
Macro accuracy (24 classes): 95.43%
Macro precision (24 classes): 82.83%
Macro recall (24 classes): 82.87%
Macro F1 (24 classes): 82.76%
Macro PR AUC (24 classes): 89.21%
Sample-wise F1: 88.75%
Validation class insights:
  Lowest precision (10):
    class   0: support   84, predicted  106, precision  63.21%, recall  79.76%, f1  70.53%
    class   4: support   90, predicted   88, precision  68.18%, recall  66.67%, f1  67.42%
    class   2: support   89, predicted   86, precision  68.60%, recall  66.29%, f1  67.43%
    class   9: support  124, predicted  126, precision  69.05%, recall  70.16%, f1  69.60%
    class   6: support   55, predicted   57, precision  71.93%, recall  74.55%, f1  73.21%
    class   1: support   54, predicted   56, precision  73.21%, recall  75.93%, f1  74.55%
    class   3: support   51, predicted   54, precision  74.07%, recall  78.43%, f1  76.19%
    class   8: support   66, predicted   65, precision  76.92%, recall  75.76%, f1  76.34%
    class   5: support   99, predicted   87, precision  78.16%, recall  68.69%, f1  73.12%
    class  10: support   71, predicted   77, precision  79.22%, recall  85.92%, f1  82.43%
  Lowest recall (10):
    class   2: support   89, predicted   86, precision  68.60%, recall  66.29%, f1  67.43%
    class   4: support   90, predicted   88, precision  68.18%, recall  66.67%, f1  67.42%
    class   5: support   99, predicted   87, precision  78.16%, recall  68.69%, f1  73.12%
    class   9: support  124, predicted  126, precision  69.05%, recall  70.16%, f1  69.60%
    class  11: support   85, predicted   75, precision  82.67%, recall  72.94%, f1  77.50%
    class   7: support  132, predicted  123, precision  79.67%, recall  74.24%, f1  76.86%
    class   6: support   55, predicted   57, precision  71.93%, recall  74.55%, f1  73.21%
    class   8: support   66, predicted   65, precision  76.92%, recall  75.76%, f1  76.34%
    class   1: support   54, predicted   56, precision  73.21%, recall  75.93%, f1  74.55%
    class   3: support   51, predicted   54, precision  74.07%, recall  78.43%, f1  76.19%

Captured dataset metrics (176 samples):
Inference accuracy: 62.50%
Macro accuracy (24 classes): 96.95%
Macro precision (24 classes): 86.16%
Macro recall (24 classes): 85.35%
Macro F1 (24 classes): 85.00%
Macro PR AUC (24 classes): 91.09%
Sample-wise F1: 90.57%
Captured class insights:
  Lowest precision (10):
    class  10: support   10, predicted   17, precision  47.06%, recall  80.00%, f1  59.26%
    class   2: support   15, predicted   14, precision  64.29%, recall  60.00%, f1  62.07%
    class   8: support   11, predicted   15, precision  66.67%, recall  90.91%, f1  76.92%
    class   9: support   14, predicted   19, precision  68.42%, recall  92.86%, f1  78.79%
    class   3: support   12, predicted   10, precision  70.00%, recall  58.33%, f1  63.64%
    class   5: support   20, predicted   19, precision  73.68%, recall  70.00%, f1  71.79%
    class   7: support   18, predicted   13, precision  76.92%, recall  55.56%, f1  64.52%
    class  22: support   37, predicted   46, precision  78.26%, recall  97.30%, f1  86.75%
    class   6: support   11, predicted   10, precision  80.00%, recall  72.73%, f1  76.19%
    class   0: support   25, predicted   27, precision  81.48%, recall  88.00%, f1  84.62%
  Lowest recall (10):
    class   7: support   18, predicted   13, precision  76.92%, recall  55.56%, f1  64.52%
    class   3: support   12, predicted   10, precision  70.00%, recall  58.33%, f1  63.64%
    class   2: support   15, predicted   14, precision  64.29%, recall  60.00%, f1  62.07%
    class   5: support   20, predicted   19, precision  73.68%, recall  70.00%, f1  71.79%
    class   4: support   14, predicted   10, precision 100.00%, recall  71.43%, f1  83.33%
    class   6: support   11, predicted   10, precision  80.00%, recall  72.73%, f1  76.19%
    class   1: support   12, predicted    9, precision 100.00%, recall  75.00%, f1  85.71%
    class  10: support   10, predicted   17, precision  47.06%, recall  80.00%, f1  59.26%
    class  14: support   36, predicted   30, precision 100.00%, recall  83.33%, f1  90.91%
    class  11: support   14, predicted   13, precision  92.31%, recall  85.71%, f1  88.89%
```

# Experiment Pass 27

Added a lightweight MLP trunk (Linear→GELU→Dropout→Linear) between attention and the output head.

## Experiment: Mel and Folded Bass Target

### Command

```bash
$ cd kord
$ cargo run --bin kord --no-default-features --features "cli ml_train ml_tch ml_train_precision_fp32 ml_store_precision_full ml_loader_mel ml_target_folded_bass" --release -- -q ml train --backend tch --training-sources samples/captured --training-sources samples/slakh --training-sources sim --noise-asset-root samples/noise --destination model --model-epochs 16
```

### Output

```
Validation dataset metrics (1000 samples):
Inference accuracy: 61.00%
Macro accuracy (24 classes): 96.72%
Macro precision (24 classes): 89.66%
Macro recall (24 classes): 89.57%
Macro F1 (24 classes): 89.46%
Macro PR AUC (24 classes): 95.77%
Sample-wise F1: 92.49%
Validation class insights:
  Lowest precision (10):
    class   2: support   88, predicted  108, precision  75.00%, recall  92.05%, f1  82.65%
    class   1: support   56, predicted   67, precision  79.10%, recall  94.64%, f1  86.18%
    class   5: support   82, predicted   91, precision  81.32%, recall  90.24%, f1  85.55%
    class   0: support  108, predicted  107, precision  83.18%, recall  82.41%, f1  82.79%
    class   7: support  104, predicted  104, precision  83.65%, recall  83.65%, f1  83.65%
    class   6: support   68, predicted   71, precision  84.51%, recall  88.24%, f1  86.33%
    class   4: support   99, predicted   96, precision  86.46%, recall  83.84%, f1  85.13%
    class   9: support  115, predicted   97, precision  87.63%, recall  73.91%, f1  80.19%
    class  10: support   74, predicted   74, precision  87.84%, recall  87.84%, f1  87.84%
    class  15: support  357, predicted  387, precision  89.92%, recall  97.48%, f1  93.55%
  Lowest recall (10):
    class   9: support  115, predicted   97, precision  87.63%, recall  73.91%, f1  80.19%
    class   8: support   60, predicted   52, precision  94.23%, recall  81.67%, f1  87.50%
    class   0: support  108, predicted  107, precision  83.18%, recall  82.41%, f1  82.79%
    class  11: support   81, predicted   68, precision  98.53%, recall  82.72%, f1  89.93%
    class   7: support  104, predicted  104, precision  83.65%, recall  83.65%, f1  83.65%
    class   4: support   99, predicted   96, precision  86.46%, recall  83.84%, f1  85.13%
    class  10: support   74, predicted   74, precision  87.84%, recall  87.84%, f1  87.84%
    class   6: support   68, predicted   71, precision  84.51%, recall  88.24%, f1  86.33%
    class   5: support   82, predicted   91, precision  81.32%, recall  90.24%, f1  85.55%
    class  22: support  355, predicted  351, precision  91.74%, recall  90.70%, f1  91.22%

Captured dataset metrics (176 samples):
Inference accuracy: 88.64%
Macro accuracy (24 classes): 99.46%
Macro precision (24 classes): 98.73%
Macro recall (24 classes): 97.55%
Macro F1 (24 classes): 98.09%
Macro PR AUC (24 classes): 99.92%
Sample-wise F1: 98.37%
Captured class insights:
  Lowest precision (10):
    class   3: support   12, predicted   12, precision  91.67%, recall  91.67%, f1  91.67%
    class   6: support   11, predicted   12, precision  91.67%, recall 100.00%, f1  95.65%
    class   2: support   15, predicted   16, precision  93.75%, recall 100.00%, f1  96.77%
    class  17: support   42, predicted   42, precision  95.24%, recall  95.24%, f1  95.24%
    class  20: support   36, predicted   37, precision  97.30%, recall 100.00%, f1  98.63%
    class   0: support   25, predicted   25, precision 100.00%, recall 100.00%, f1 100.00%
    class   1: support   12, predicted   12, precision 100.00%, recall 100.00%, f1 100.00%
    class   4: support   14, predicted   14, precision 100.00%, recall 100.00%, f1 100.00%
    class   5: support   20, predicted   19, precision 100.00%, recall  95.00%, f1  97.44%
    class   7: support   18, predicted   17, precision 100.00%, recall  94.44%, f1  97.14%
  Lowest recall (10):
    class  16: support   32, predicted   28, precision 100.00%, recall  87.50%, f1  93.33%
    class   3: support   12, predicted   12, precision  91.67%, recall  91.67%, f1  91.67%
    class  18: support   30, predicted   28, precision 100.00%, recall  93.33%, f1  96.55%
    class  21: support   35, predicted   33, precision 100.00%, recall  94.29%, f1  97.06%
    class   7: support   18, predicted   17, precision 100.00%, recall  94.44%, f1  97.14%
    class  22: support   37, predicted   35, precision 100.00%, recall  94.59%, f1  97.22%
    class   5: support   20, predicted   19, precision 100.00%, recall  95.00%, f1  97.44%
    class  17: support   42, predicted   42, precision  95.24%, recall  95.24%, f1  95.24%
    class  15: support   39, predicted   38, precision 100.00%, recall  97.44%, f1  98.70%
    class  12: support   44, predicted   43, precision 100.00%, recall  97.73%, f1  98.85%
```

# Experiment Pass 28

Added 1D convolution.

## Experiment: Mel and Folded Bass Target

### Command

```bash
$ cd kord
$ cargo run --bin kord --no-default-features --features "cli ml_train ml_tch ml_train_precision_fp32 ml_store_precision_full ml_loader_mel ml_target_folded_bass ml_model_mel_conv1d" --release -- -q ml train --backend tch --training-sources samples/captured --training-sources samples/slakh --training-sources sim --noise-asset-root samples/noise --destination model --model-epochs 16
```

### Output

```

```