import doodle.core._
import doodle.syntax._
import scala.math._
import scala.util.boundary, boundary.break

val eps = 0.0001

def f(x: Double): Double =
  tan(x) - x

def finverse(x: Double): Double =
  atan(x)

def lfg(x: Double): Double =
  1 / (cos(x) * cos(x)) - 1

def df(f: Double => Double): Double => Double =
  x => (f(x + eps) - f(x)) / eps

def bisectionMethod(f: Double => Double, period: Int): Double =
  var (a, b) = getPeriod(period)
  var c = 0.0
  var fa = f(a)
  while (abs(a - b) > 2 * eps) do
    c = (a + b) / 2.0
    val fc = f(c)
    if fc == 0 then return c
    if fa * fc < 0 then b = c
    else
      a = c
      fa = fc
  c

def simpleIterations(
    f: Double => Double,
    finverse: Double => Double,
    period: Int
): Double =
  var (left, right) = getPeriod(period)
  var x = (left + right) / 2
  val shift = period * Pi
  val eps = 0.0001
  val a = 0.01
  val limit = 5000

  var f1 = f(x)
  var x1 = x - f1 * a
  while abs(x1 - x) > eps do
    x = x1
    f1 = f(x)
    x1 = x - a * f1
  x

def newton(period: Int, f: Double => Double, df: Double => Double): Double =
  def secantsStep(l: Double, r: Double): (Double, Double, Double) =
    val ret = r - f(r) * (r - l) / (f(r) - f(l))
    (r, ret, abs(r - ret))

  var (v1, v2) = (0.0, 0.0)
  if period >= 0 then
    v1 = period * Pi + (Pi / 2 - eps)
    v2 = v1 - eps
  else
    v1 = period * Pi - (Pi / 2 - eps)
    v2 = v1 + eps

  var delta = 1.0
  for i <- 0 until 10 do
    secantsStep(v1, v2) match
      case (tmp1, tmp2, tmpDelta) =>
        v1 = tmp1
        v2 = tmp2
        delta = tmpDelta

  val limit = 5000
  boundary:
    for i <- 0 to limit do
      v2 = v1
      v1 = v2 - f(v2) / df(v2)
      if abs(v2 - v1) < eps then break()
  v1

def secants(f: Double => Double, period: Int): Double =
  def secantsStep(l: Double, r: Double): (Double, Double, Double) =
    val ret = r - f(r) * (r - l) / (f(r) - f(l))
    (r, ret, abs(r - ret))

  var (x1, x2) = (0.0, 0.0)
  if period >= 0 then
    x1 = period * Pi + (Pi / 2 - eps)
    x2 = x1 - eps
  else
    x1 = period * Pi - (Pi / 2 - eps)
    x2 = x1 + eps
  var delta = 1.0

  for i <- 0 until 10 do
    secantsStep(x1, x2) match
      case (tmp1, tmp2, tmpDelta) =>
        x1 = tmp1
        x2 = tmp2
        delta = tmpDelta

  while delta > eps do
    secantsStep(x1, x2) match
      case (tmp1, tmp2, tmpDelta) =>
        x1 = tmp1
        x2 = tmp2
        delta = tmpDelta

  val deltaPrev = delta
  secantsStep(x1, x2) match
    case (tmp1, tmp2, tmpDelta) =>
      x1 = tmp1
      x2 = tmp2
      delta = tmpDelta

  if (delta > deltaPrev) return x2
  while delta < deltaPrev && delta > 0 do
    secantsStep(x1, x2) match
      case (tmp1, tmp2, tmpDelta) =>
        x1 = tmp1
        x2 = tmp2
        delta = tmpDelta
  x2

def printMethod(str: String, res: Double): Unit =
  println(str)
  println(res)
  println("--------------------------------")

def getPeriod(period: Int): (Double, Double) =
  if period < 0 then (Pi * period - Pi / 2 + eps, Pi * period + Pi / 2 - eps)
  else (Pi * period + Pi / 2 - eps, Pi * period - Pi / 2 + eps)

@main def m() =
  val period = 2
  printMethod(
    "bisection: ",
    bisectionMethod(f, period)
  )
  printMethod(
    "simple iterations: ",
    simpleIterations(f, finverse, period)
  )
  printMethod("newton method: ", newton(period, f, lfg))
  printMethod("secants method: ", secants(f, period))
