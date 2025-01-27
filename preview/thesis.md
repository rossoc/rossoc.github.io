--- 
layout: preview
title: Comparison of Kernel Methods and Large Language Models in Sentiment Analysis 
category: preview
---

This summary describes the work carried out during a roughly 300-hour internship 
by Carlo Rosso, a student at the University of Padua. 
The project had multiple objectives.

After studying the problem of sentiment analysis and reviewing the related 
scientific literature, I implemented, experimented with, and compared at least 
three state-of-the-art models using the Sentiment Penn Treebank corpus. 
I also documented the implemented models and uploaded them to a git repository 
to ensure the reproducibility of the results.

More specifically, during the internship, I investigated the role of syntactic 
analysis and its compositional properties in understanding semantic phenomena, 
with a particular focus on sentiment analysis.

---

# Problem

Sentiment analysis

![classifier](assets/img/thesis/classifier.png)

---

# Dataset

Stanford Sentiment Treebank

![labeled-tree](assets/img/thesis/labeled-tree.png)

---

# Dataset

![grammar-tree](assets/img/thesis/labeled-tree.png)

---

# Dataset

![label-distribution](assets/img/thesis/label-distribution.png)

---

# RNN

| Modello | Accuratezza (%)
| --- | --- |
| RNN | 43.2 |
| RNN con 25 unità nascoste | 39.7 |
| RNN con 50 unità nascoste | 42.4 |
| RNN con 75 unità nascoste | 38.8 |
| RNN con 100 unità nascoste | 39.8 |

---

# Kernel Method

| Dataset | Accuratezza (%) |
| --- | --- |
| Subtree | 51 |
| Merged | 53 |
| Sentiment | 54 |
| Syntax | 39 |

---

# Kernel Method

| Modello | Accuratezza(%) |
| --- | --- |
| Subset Tree Sentiment non normalized | 55.2 |
| Subset Tree-bow Sentiment non normalized | 55.2 |
| Partial Tree Sentiment non normalized | 55.2 |

---

# Large Language Model

| Modello | Accuratezza(%) |
| --- | --- |
| Bert | 52.3 |
| Bert (implementato da me) | 53.2 |
| RoBERTa | 56.4 |
| RoBERTa (implementato da me) | 57.3 |
| DistilBert | 52.0 |

---

# Comparison

| Modello | Accuratezza(%) |
| --- | --- |
| RNN | 42.4 |
| Kernel Method | 55.2 |
| RoBERTa | 57.3 |

---

# Objectives

**Achieved Objectives**:
- Study of the problem and the literature
- Comparison among at least three state-of-the-art models
- Documentation of the models, their implementation, and their experimentation

**Desirable Objective Not Achieved**:
- Implementation of a model that combines the best developed models

---

# Future Work

Models combinations:

![models-combinations](assets/img/thesis/models-combinations.png)

---

# Acquired Skills

- Comparison and evaluation of models
- In-depth study of literature (NLP)
- Time management
- Independent work
- Problem solving
