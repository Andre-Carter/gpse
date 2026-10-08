# Carter Ratio — Research Updates

## Current Research Status

The Carter Ratio investigation has progressed from observing numerical sequences to analyzing the algebraic and asymptotic structure of the formula.

The current research should distinguish between different versions of the formula rather than treating them as the same expression.

---

## 1. Original Carter Ratio Form

The earlier form investigated was:

$$
CT(n)=
\frac{
\left[
\left(n-\frac12\right)^2-
\left(1+\frac{\pi}{4}\right)
\right]^2
}{2\pi}
$$

This produces rapidly increasing values because the dominant term is proportional to \(n^4\).

For large \(n\):

$$
CT(n)\sim\frac{n^4}{2\pi}
$$

Therefore:

$$
\frac{CT(n)}{n^4}\rightarrow\frac{1}{2\pi}
$$

---

## 2. Normalized Carter Ratio

A normalized investigation was performed by dividing the Carter Ratio by \(n^4\):

$$
C_4(n)=\frac{CT(n)}{n^4}
$$

Observed values included:

```text
n = 10   -> 0.1266753213814777
n = 20   -> 0.14304419973275068
n = 40   -> 0.1511443623762527
n = 60   -> 0.15382570856675662
n = 80   -> 0.1551623656525511
n = 100  -> 0.1559630107061143
```

At:

$$
n=1,000,000
$$

the normalized value was:

```text
0.1591546247819183
```

while:

$$
\frac{1}{2\pi}
=
0.15915494309189535\ldots
$$

This demonstrates extremely close numerical convergence toward \(1/(2\pi)\).

---

## 3. Asymptotic Structure

The dominant behavior follows directly from the highest-power term.

Since:

$$
\left(n-\frac12\right)^2
=
n^2-n+\frac14
$$

the squared expression produces a fourth-degree polynomial in \(n\).

Consequently:

$$
CT(n)
=
\frac{n^4}{2\pi}
-\frac{n^3}{\pi}
+O(n^2)
$$

and therefore:

$$
\boxed{
\lim_{n\rightarrow\infty}
\frac{CT(n)}{n^4}
=
\frac{1}{2\pi}
}
$$

This is currently regarded as a mathematical property of the constructed formula, not evidence by itself of a new mathematical constant or theorem.

---

## 4. First-Order Residual Investigation

To investigate the convergence rather than merely observe it, the following residual was proposed:

$$
R_1(n)=
n\left[
\frac{1}{2\pi}
-
\frac{CT(n)}{n^4}
\right]
$$

The purpose is to remove the leading limiting value and determine the coefficient of the next asymptotic term.

From the polynomial expansion, the expected limiting coefficient is related to:

$$
\frac{1}{\pi}
$$

This provides a testable prediction.

---

## 5. Second-Order Residual Investigation

A second residual was proposed:

$$
R_2(n)=
n^2
\left[
\frac{1}{2\pi}
-
\frac{CT(n)}{n^4}
-
\frac{1}{\pi n}
\right]
$$

The goal is to remove both the leading constant and first-order correction, exposing the next coefficient.

This investigation must use one exact definition of the Carter Ratio throughout. Earlier calculations became inconsistent because the formula itself was changed.

---

# 6. Important Formula Correction

A significant distinction was discovered between these two expressions:

### Version A

$$
1+\frac{\pi}{4}
$$

### Version B

$$
\frac{1+\pi}{4}
$$

They are **not equivalent**.

Their difference is:

$$
\left(1+\frac{\pi}{4}\right)
-
\left(\frac{1+\pi}{4}\right)
=
\frac34
$$

Therefore, numerical results generated from the two formulas must not be mixed.

---

# 7. Current Carter Theory Implementation

The current Rust implementation is:

```rust
pub fn carter_theory(n: f64) -> f64 {
    let ct_input_1 = n - 1.0 / 2.0;
    let ct_input_2 = ct_input_1.powi(2);

    let ct_input_3 = (1.0 + PI) / 4.0;
    let ct_input_4 = ct_input_2 - ct_input_3;

    let ct_input_5 = ct_input_4.powi(2);

    let ct_input_6 = ct_input_5 / (2.0 * PI);

    let ct_input_7 = ct_input_6 / n.powi(4);

    ct_input_7
}
```

This implementation is **not the same formula as the original Carter Ratio investigation**.

The current function calculates:

$$
\boxed{
C(n)=
\frac{
\left[
\left(n-\frac12\right)^2-
\frac{1+\pi}{4}
\right]^2
}{
2\pi n^4
}
}
$$

The division by \(n^4\) is now part of the function itself.

---

# 8. Current Large-\(n\) Behavior

Despite the change in the constant term, the highest-order behavior remains:

$$
\left(n-\frac12\right)^2-\frac{1+\pi}{4}
\sim n^2
$$

Therefore:

$$
C(n)\sim\frac{n^4}{2\pi n^4}
$$

giving:

$$
\boxed{
\lim_{n\rightarrow\infty}C(n)=\frac{1}{2\pi}
}
$$

The observed result at \(n=1,000,000\):

```text
0.1591546247819183
```

is consistent with this asymptotic behavior.

---

# 9. Research Principle

The investigation has established an important methodology for the Carter Ratio project:

1. Define the exact formula.
2. Keep each formula version separate.
3. Generate numerical sequences.
4. Identify apparent limits.
5. Algebraically expand the expression.
6. Derive predicted asymptotic coefficients.
7. Test those coefficients numerically.
8. Examine residuals.
9. Compare against known mathematical relationships.
10. Only then investigate whether anything genuinely novel remains.

Numerical convergence alone does not establish mathematical novelty.

---

# 10. Current Research Question

The central question has evolved from:

> "What sequence does the Carter Ratio produce?"

to:

> **"What mathematical structure is encoded in the Carter Ratio, and does that structure contain relationships not immediately explained by its construction?"**

The next investigation should therefore focus on the **current formula independently**, deriving its complete asymptotic expansion and testing successive residuals.

---

## Research Status

**Confirmed:**

* The normalized Carter Ratio approaches \(1/(2\pi)\).
* The fourth-order behavior is responsible for this convergence.
* The convergence can be analyzed algebraically.
* Different formula constructions produce different sequences and must remain separate.
* The \(n=1,000,000\) calculation provides strong numerical confirmation of the predicted limiting behavior.

**Not yet established:**

* A new mathematical constant.
* A new theorem.
* A previously unknown identity.
* A new fundamental mathematical relationship.
* That the Carter Ratio itself constitutes a novel mathematical discovery.

The investigation remains open.
