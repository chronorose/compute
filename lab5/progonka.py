import numpy as np
import matplotlib.pyplot as plt


def thomas(A, B, C, F):
    n = len(B)
    alpha = [-C[0] / B[0]]
    beta = [F[0] / B[0]]
    for i in range(1, n):
        divisor = A[i] * alpha[-1] + B[i]
        alpha.append(-C[i]/divisor)
        beta.append((F[i] - A[i] * beta[i - 1])/divisor)

    x = [beta[-1]]
    for i in range(n - 2, -1, -1):
        x.append(alpha[i] * x[-1] + beta[i])
    x.reverse()
    return np.array(x)


def task(n, boundaries):
    a, b = -np.pi / 2, np.pi / 2

    xs = np.linspace(a, b, n)

    h = (b - a) / (n - 1)
    h2 = h ** 2

    l1, r1, phi1, l2, r2, phi2 = boundaries

    A = np.repeat(1 / h2, n)
    B = np.repeat(-2 / h2, n)
    C = np.repeat(1 / h2, n)

    A[0] = 0
    B[0] = l1 - r1 / h
    C[0] = r1 / h

    A[-1] = -r2 / h
    B[-1] = l2 + (r2 / h)
    C[-1] = 0

    F = np.cos(xs)
    F[0], F[-1] = phi1, phi2

    return xs, thomas(A, B, C, F)


if __name__ == '__main__':
    # boundaries = (1, 0, 0, 1, 0, 0)
    # def expected(x): return -np.cos(x)
    boundaries = (1, 0, 2, 1, 0, 4)
    def expected(x): return -np.cos(x) + 3 + (2 / np.pi) * x
    n = 50
    xs, ys = task(n, boundaries)
    plt.plot(xs, ys, 'bo')
    plt.plot(xs, expected(xs))
    plt.show()
