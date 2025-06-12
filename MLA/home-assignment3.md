---
layout: note-sidebar
title: Home Assignment 3  
category: MLA  
---

# Preprocessing (33 points)

## Importance of Preprocessing (6 points)

We have the following data points:

### a)

| Person | Age in years | Income in thousands of USD | Paid off |
|--------|--------------|----------------------------|----------|
| A      | 47           | 35                         | yes      |
| B      | 22           | 40                         | no       |
| C      | 21           | 36                         | -        |

$$
d_A = \sum_{i=1}^2 (x_i - y_i)^2 = (21-47)^2 + (36-35)^2 = 677
$$

$$
d_B = (21-22)^2 + (36-40)^2 = 17
$$

Therefore, we get $d_A > d_B$, and we can conclude the BoL should not give credit to C, according to the nearest neighbor algorithm.

### b)

| Person | Age in years | Income in thousands of USD | Paid off |
|--------|--------------|----------------------------|----------|
| A      | 47           | 35000                      | yes      |
| B      | 22           | 40000                      | no       |
| C      | 21           | 36000                      | -        |

$$
d_A = \sum_{i=1}^2 (x_i - y_i)^2 = (21-47)^2 + (36000-35000)^2 = 1000676
$$

$$
d_B = (21-22)^2 + (36000-40000)^2 = 16000001
$$

Therefore, we get $d_A < d_B$, and we can conclude the BoL should give credit to C, according to the nearest neighbor algorithm.

## Input Centering (9 points)

### a)

Considering the following equations:

$$ z_n = x_n - \bar{x}, \forall n = 1, \dots, N $$

$$ \bar{x} = \frac{1}{N} X^T 1 $$

$$ \gamma = 1 - \frac{1}{N} 1 1^T $$

We can show that:

$$ Z = \gamma X $$

Indeed:

$$ z_n = x_n - \bar{x}, \forall n = 1, \dots, N $$

Turning this into a matrix form, we get:

$$ Z = X - 1 \bar{x}^T $$

$$ = X - 1 (\frac{1}{N} X^T 1)^T $$

Remembering that $(A B)^T = B^T A^T$, we get:

$$ Z = X -  \frac{1}{N} 1 1^T X $$

$$ = I X -  \frac{1}{N} 1 1^T X $$

$$ = (I -  \frac{1}{N} 1 1^T) X $$

$$ = \gamma X $$

Therefore, we have shown that $Z = \gamma X$.

### b)

Considering that $Z$ is an $N \times D$ matrix and $\text{rank}(Z) = \text{rank}(Z^T)$. Using the rank-nullity theorem, we have:

$$ \text{rank}(A) + \text{rank}(\text{ker}(A)) = d $$

Citing a property of the rank:

$$ \text{rank}(A B) \leq \min(\text{rank}(A), \text{rank}(B)) $$

Therefore, we have that:

$$ \text{rank}(\gamma) + \text{rank}(\text{ker}(\gamma)) = N $$

And since $\text{rank}(\text{ker}(\gamma)) = 1$, we have $\text{rank}(\gamma) = N - 1$.

Thus, $\text{rank}(Z) = \text{rank}(\gamma X) \leq \min(\text{rank}(X), N - 1) < N$.

## Input Whitening (18 points)

### a)

Given the following:

$$ \text{Var}(\hat{x}_1) = \text{Var}(\hat{x}_2) = 1 $$

$$ \mathbb{E}[\hat{x}_1] = \mathbb{E}[\hat{x}_2] = 0 $$

$$ x_1 = \hat{x}_1 $$

$$ x_2 = \sqrt{1 - \epsilon^2} \hat{x}_1 + \epsilon \hat{x}_2, \quad \text{for } \epsilon \in [-1, 1] $$

$$ \text{Cov}(\hat{x}_1, \hat{x}_2) = 0 $$

The last equation is given by the fact that the two variables are independent. Therefore, we already have the variance of $x_1$: $\text{Var}(x_1) = \text{Var}(\hat{x}_1) = 1$.

The variance of $x_2$ is given by:

$$ \text{Var}(x_2) = (\sqrt{1 - \epsilon^2})^2 \text{Var}(\hat{x}_1) + \epsilon^2 \text{Var}(\hat{x}_2) $$

$$ = 1 - \epsilon^2 + \epsilon^2 $$

$$ = 1 $$

Finally, the covariance between $x_1$ and $x_2$ is given by:

$$ \text{Cov}(x_1, x_2) = \sqrt{1 - \epsilon^2} \text{Cov}(\hat{x}_1, \hat{x}_1) + \epsilon \text{Cov}(\hat{x}_1, \hat{x}_2) $$

$$ = \sqrt{1 - \epsilon^2} \text{Var}(\hat{x}_1) $$

$$ = \sqrt{1 - \epsilon^2} $$

### b)

Given the following:

$$ x = (x_1, x_2)^T $$

$$ \hat{x} = (\hat{x}_1, \hat{x}_2)^T $$

$$ f(\hat{x}) = \hat{w}_1 \hat{x}_1 + \hat{w}_2 \hat{x}_2 $$

This leads to the following equivalent statements:

$$ w_1 x_1 + w_2 x_2 = \hat{w}_1 \hat{x}_1 + \hat{w}_2 \hat{x}_2 $$

$$ w_1 \hat{x}_1 + w_2 (\sqrt{1 - \epsilon^2} \hat{x}_1 + \epsilon \hat{x}_2) = \hat{w}_1 \hat{x}_1 + \hat{w}_2 \hat{x}_2 $$

$$ w_1 \hat{x}_1 + w_2 \sqrt{1 - \epsilon^2} \hat{x}_1 + w_2 \epsilon \hat{x}_2 = \hat{w}_1 \hat{x}_1 + \hat{w}_2 \hat{x}_2 $$

This results in the following system of equations:

$$
\begin{cases}
    (w_1 + w_2 \sqrt{1 - \epsilon^2}) \hat{x}_1 = \hat{w}_1 \hat{x}_1 \\
    w_2 \epsilon \hat{x}_2 = \hat{w}_2 \hat{x}_2
\end{cases}
$$

Thus, we arrive at the final conclusion that $f$ is linear in the correlated inputs:

$$
\begin{cases}
    w_1 = \hat{w}_1 - \hat{w}_2 / \epsilon \sqrt{1 - \epsilon^2} \\
    w_2 = \hat{w}_2 / \epsilon
\end{cases}
$$

### c)

Given the target function:

$$ f(\hat{x}) = \hat{x}_1 + \hat{x}_2 $$

With the constraint $C$:

$$ w_1^2 + w_2^2 \leq C $$

If we perform regression with the correlated inputs $x$, let's find the minimum value of $C$ such that the constraint is satisfied.

First, let's compute the values of $w_1$ and $w_2$:

$$ \hat{w}_1 = 1 $$

$$ \hat{w}_2 = 1 $$

Therefore:

$$ w_1 = 1 - 1 / \epsilon \sqrt{1 - \epsilon^2} $$

$$ w_2 = 1 / \epsilon $$

Now, let's compute the value of $C$:

$$ C = w_1^2 + w_2^2 $$

$$ = (1 - 1 / \epsilon \sqrt{1 - \epsilon^2})^2 + (1 / \epsilon)^2 $$

$$ = 1 + \frac{1 - \epsilon^2}{\epsilon^2} - \frac{2}{\epsilon} \sqrt{1 - \epsilon^2} + \frac{1}{\epsilon^2} $$

$$ = \frac{2}{\epsilon^2} - \frac{2 \sqrt{1 - \epsilon^2}}{\epsilon} $$

### d)

Finally, let's compute the following limit:

$$ \lim_{\epsilon \to 0} C = \lim_{\epsilon \to 0} \left( \frac{2}{\epsilon^2} - \frac{2 \sqrt{1 - \epsilon^2}}{\epsilon} \right) = \infty $$

# Competition Design to Find Defective Products (24 points)

## 

Follows the theorem of generalization bound for selection from finite $\mathcal{H}$:

$$ \mathbb{P}(L(\hat{h}^\star_S) \leq \hat{L}(\hat{h}^\star_S, S) + \sqrt{\frac{\ln(M / \delta)}{2n}}) \geq 1 - \delta $$

Let's repeat our hypothesis:

$$ M = 20 $$

$$ \delta = 2 $$

We are looking for the minimum value of $n$ such that the following inequality is satisfied:

$$ \sqrt{\frac{\ln(M / \delta)}{2n}} \leq 0.04 $$

Therefore:

$$ \frac{\ln(20 / 2)}{2 \cdot 0.04^2} = 312.5 < 313 = n $$

## 

Given the following:

$$ n = 1800 $$

$$ \delta = 2 $$

We are looking for the maximum value of $M$ such that the following inequality is satisfied:

$$ \sqrt{\frac{\ln(M / \delta)}{2n}} \leq 0.04 $$

Therefore:

$$ 2 \exp(0.04^2 \cdot 2 \cdot 1800) \approx 634.7 > 634 = M $$

# Combining Multiple Confidence Intervals (22 points)

Given the following:

$$ i \in I = \{1, 2, 3\} $$

$$ S_i = S $$

$$ \text{CI}_i = [l_i, u_i] \, \text{w.p.} \, 1 - \delta_i $$

$$ 0.99 = \prod_I (1 - \delta_i) $$

$$ \delta = \delta_i = \delta_j \, \text{for all} \, i, j \in I $$

Let's compute the value of $\delta$:

$$ 1 - \sqrt[3]{0.99} \approx 0.0033 < 0.004 = \delta $$

We could compute a more precise value for $\delta$, but I only need to show how to answer this question. Alex can choose any combination of the confidence interval endpoints such that:

$$ l_{\text{chosen}} \leq u_{\text{chosen}} $$

Because any such combination is a valid (at least 99%)-CI. Therefore, he should choose the combination that minimizes the length of the CI:

$$ \text{CI} = [\max(l_i), \min(u_i)] $$

# Early Stopping (21 points)

## Neural Network with Early Stopping (21 points)

Statistical bias, in the mathematical field of statistics, is a systematic tendency in which the methods used to gather data and generate statistics present an inaccurate, skewed, or biased depiction of reality. — [Wikipedia@bias]

### Predefined Stopping

The $S_{\text{val}}$ has no influence on the choice of the target function $h_{t^\star}$, so the bias is not present.

### Non-adaptive Stopping

The target function $ h_{t^\star} $ that minimizes the validation error 
$ \hat{L} (h_{t^\star}) $ is chosen. Therefore, the dataset is used to choose the 
best target function, which leads the final model to be biased by the validation 
set $S_{\text{val}}$.

### Adaptive Stopping

$h_{t^\star}$ is chosen from the sequence of hypotheses $h_1, h_2, h_3, \dots, 
h_t$. While the target function is not chosen based on the validation set, the 
sequence stops when the validation does not improve for a certain number of 
steps. Therefore, the sequence of models is biased by the validation set 
$S_{\text{val}}$. As a counterexample to the claim that the bias is not present, 
let's consider the case in which a different validation set $S_{\text{val}}'$ 
is used. Then, it would produce the sequence of hypothesis models $h_1, h_2, 
h_3, \dots, h_j$, and $j \neq t$. Let $j > t$ and $h_j$ be the best model, thus 
the final model choice differs from the one obtained with the original 
validation set $S_{\text{val}}$. Therefore, we can conclude that the final 
model is biased by the validation set $S_{\text{val}}$.

##

I have already cited the theorem of generalization bound (see @hp-bound), so follows the solution for the two cases.

### Predefined Stopping

We have $M = 1$, because we are only considering the final model:

$$ \mathbb{P}(L(\hat{h}^\star_S) \leq \hat{L}(\hat{h}^\star_S, S) + \sqrt{\frac{\ln(1 / \delta)}{2n}}) \geq 1 - \delta $$

### Non-adaptive Stopping

We have $M = T$, where $T$ is the number of epochs and thus the number of models to consider:


$$ \mathbb{P}(L(\hat{h}^\star_S) \leq \hat{L}(\hat{h}^\star_S, S) + \sqrt{\frac{\ln(T / \delta)}{2n}}) \geq 1 - \delta $$


