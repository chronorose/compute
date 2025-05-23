import matplotlib.pyplot as plt
import numpy as np


def f(z):
    return z**3 - 1.0


def df(z):
    return 3.0 * z**2


def newtonMethod(sol):
    start = sol
    attempt = sol
    iters = 5000
    eps = 0.00001
    for _ in range(iters):
        print(sol)
        sol = attempt - f(attempt) / df(attempt)
        if abs(attempt - sol) < eps:
            break
        attempt = sol
    real = round(sol.real, 4)
    imag = round(sol.imag, 4)
    return complex(real, imag)


def plot():
    def color(root):
        eps = 0.001
        if abs(root - 1) < eps:
            return "g"
        if abs(root - complex(-0.5000, -0.86603)) < eps:
            return "b"
        return "r"

    size = 1
    step = 0.01
    print(newtonMethod(complex(-0.617, -0.217)))
    # real = np.arange(-size, size, step)
    # imag = np.arange(-size, size, step)
    # for x in real:
    #     for y in imag:
    #         root = newtonMethod(complex(x, y))
    #         plt.plot(x, y, color(root) + "o")
    #
    # plt.show()


if __name__ == "__main__":
    plot()
