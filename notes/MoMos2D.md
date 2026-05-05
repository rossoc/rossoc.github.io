---
layout: note
title: MoMos 2D
---

## Project Overview: Algorithmic Complexity in Neural Networks

Recent research has explored the relationship between **Neural Network (NN) complexity** and **Kolmogorov complexity** ([Sakabe et al., 2025](#ref-sakabe); [Bakhtiarifard et al., 2026](#ref-momo)). 

### 1. Complexity and Generalization
Specifically, [Sakabe et al. (2025)](#ref-sakabe) demonstrate that approximating the Kolmogorov complexity of Binarized NNs provides critical insights into training dynamics and correlates strongly with generalization capabilities. 

*   **Global vs. Local:** Unlike traditional statistical entropy, which measures global complexity, Kolmogorov complexity offers a more granular understanding of:
    *   Internal model structure.
    *   Block-wise weight complexity.

### 2. The Mosaic-of-Motifs (MoMo) Framework
Building on these foundations, the **Mosaic-of-Motifs (MoMo)** framework ([Bakhtiarifard et al., 2026](#ref-momo)) provides a reliable method to bound Kolmogorov complexity from above by decoupling a model's weights from its architecture. 

> **Key Advantage:** MoMo is highly flexible and can be applied to any architecture. It often yields better compression rates than standard quantization by enforcing algorithmic simplicity directly during the training phase.

### 3. Proposed Enhancements
The aim of this project is to enhance the MoMo framework by introducing more expressive mappings. Our research focuses on two primary directions:

*   **2D Structural Mappings:** We propose using 2D mappings ($\phi$) to better capture structural regularities and repetitions within weight matrices. This approach aims to:
    *   Allow for larger block sizes.
    *   Reduce overall network capacity without sacrificing representational power.
*   **Hierarchical Compression:** We investigate hierarchical compression as a post-hoc procedure to further simplify discovered motifs. This aims to produce neural networks that are both algorithmically simpler and more energy-efficient.

---

### References

<a id="ref-sakabe"></a>
*   **Sakabe, E. Y., et al. (2025).** *Binarized Neural Networks Converge Toward Algorithmic Simplicity: Empirical Support for the Learning-as-Compression Hypothesis.* [arXiv:2505.20646](http://arxiv.org/abs/2505.20646).

<a id="ref-momo"></a>
*   **Bakhtiarifard, P., et al. (2026).** *Algorithmic Simplification of Neural Networks with Mosaic-of-Motifs.* [arXiv:2602.14896](http://arxiv.org/abs/2602.14896).
