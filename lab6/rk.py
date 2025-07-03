import numpy as np
import matplotlib.pyplot as plt
from collections.abc import Callable


def task1(h):
    def f(t: float, y: float) -> float:
        return -y

    def solution(t: float) -> float:
        return np.exp(-t)
    y0 = 1.

    ts = np.arange(0, 2, h)
    ysrk = rk4scalar(f, y0, ts)
    ysexeuler = exeulerscalar(f, y0, ts)
    ysimpleuler = impleulerscalar(f, y0, ts)
    ys = solution(ts)
    errorsrk = np.max(np.abs(ysrk - ys))
    errorsexeuler = np.max(np.abs(ysexeuler - ys))
    errorsimpleuler = np.max(np.abs(ysimpleuler - ys))

    ts2 = np.arange(0, 2, h / 2)
    ys2rk = rk4scalar(f, y0, ts2)
    ys2exeuler = exeulerscalar(f, y0, ts2)
    ys2impleuler = impleulerscalar(f, y0, ts2)
    ys2 = solution(ts2)
    errors2rk = np.max(np.abs(ys2rk - ys2))
    errors2exeuler = np.max(np.abs(ys2exeuler - ys2))
    errors2impleuler = np.max(np.abs(ys2impleuler - ys2))

    errorrk = np.log2(errorsrk / errors2rk)
    errorexeuler = np.log2(errorsexeuler / errors2exeuler)
    errorimpleuler = np.log2(errorsimpleuler / errors2impleuler)
    return (errorrk, errorexeuler, errorimpleuler)


def exeulerscalar(f, y0, t):
    n = len(t)
    ys = np.zeros(n)
    ys[0] = y0
    for i in range(n - 1):
        h = t[i + 1] - t[i]
        ys[i + 1] = ys[i] + h * f(t[i], ys[i])
    return ys


def impleulerscalar(f, y0, t):
    n = len(t)
    ys = np.zeros(n)
    ys[0] = y0
    for i in range(n - 1):
        h = t[i + 1] - t[i]
        ys[i + 1] = ys[i] + h * f(t[i] + h, ys[i] + h)
    return ys


def rk4scalar(f: Callable, y0: float, t):
    ys = np.zeros(len(t))
    ys[0] = y0
    h = t[1] - t[0]

    for i in range(len(t) - 1):
        k1 = f(t[i], ys[i])
        k2 = f(t[i] + h / 2, ys[i] + h * k1 / 2)
        k3 = f(t[i] + h / 2, ys[i] + h * k2 / 2)
        k4 = f(t[i] + h, ys[i] + h * k3)
        ys[i + 1] = ys[i] + (h / 6) * (k1 + 2 * k2 + 2 * k3 + k4)
    return ys


def rk4v(f: Callable, start, t, sigma, b, r):
    ys = np.zeros((len(t), len(start)))
    ys[0] = start
    h = t[1] - t[0]

    for i in range(len(t) - 1):
        k1 = f(t[i], ys[i], sigma, b, r)
        k2 = f(t[i] + h / 2, ys[i] + h * k1 / 2, sigma, b, r)
        k3 = f(t[i] + h / 2, ys[i] + h * k2 / 2, sigma, b, r)
        k4 = f(t[i] + h, ys[i] + h * k3, sigma, b, r)
        ys[i + 1] = ys[i] + (h / 6) * (k1 + 2 * k2 + 2 * k3 + k4)

    return ys


def task3():
    def lorenz(t, v, sigma, b, r):
        dx = sigma * (v[1] - v[0])
        dy = v[0] * (r - v[2]) - v[1]
        dz = v[0] * v[1] - b * v[2]
        return np.array([dx, dy, dz])

    x = 1.
    y = 1.
    z = 1.
    sigma = 10.
    b = 8. / 3.
    r = 25.
    ts = np.arange(0, 100, 0.01)
    start = np.array([x, y, z])
    lorenz_solution = rk4v(lorenz, start, ts, sigma, b, r)

    fig = plt.figure()
    sp = fig.add_subplot(111, projection='3d')

    sp.plot(lorenz_solution[:, 0],
            lorenz_solution[:, 1], lorenz_solution[:, 2])

    plt.show()

    return lorenz_solution


def f(u, v):
    du = 998 * u + 1998 * v
    dv = -999 * u - 1999 * v
    return np.array([du, dv])


def exeulerv(f, ts, u0, v0):
    n = len(ts)
    u = np.zeros(n)
    v = np.zeros(n)
    u[0] = u0
    v[0] = v0
    for i in range(n - 1):
        h = ts[i + 1] - ts[i]
        it = f(u[i], v[i])
        u[i + 1] = u[i] + h * it[0]
        v[i + 1] = v[i] + h * it[1]
    return u, v


def impleulerv(f, ts, u0, v0):
    n = len(ts)
    u = np.zeros(n)
    v = np.zeros(n)
    u[0] = u0
    v[0] = v0
    for i in range(n - 1):
        h = ts[i + 1] - ts[i]
        it = f(u[i] + h, v[i] + h)
        u[i + 1] = u[i] + h * it[0]
        v[i + 1] = v[i] + h * it[1]
    return u, v


def task2():
    def solution(t):
        u = 2 * np.exp(-t) - np.exp(-1000*t)
        v = np.exp(-1000*t) - np.exp(-t)
        return (u, v)

    ts = np.arange(-1, 1, 0.01)
    u, v = solution(ts)
    ui, vi = impleulerv(f, ts, 0, 0)
    ue, ve = exeulerv(f, ts, 0, 0)
    fig = plt.figure()
    sp = fig.add_subplot(111, projection='3d')
    sp.plot(ts, ue, ve, 'r--')
    sp.plot(ts, ui, vi, 'b--')
    sp.plot(ts, u, v, 'g--')
    plt.grid()
    plt.show()
    return ui, vi


if __name__ == "__main__":
    rk_errors = task1(0.01)
    lorenz_attractor = task3()
    u, v = task2()
    print(rk_errors)
    # print(lorenz_attractor)
