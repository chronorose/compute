#include <cmath>
#include <concepts>
#include <ios>
#include <iostream>
#include <limits>
#include <numeric>
#include <string>

using namespace std;

template <std::floating_point T> void getMaxExp() {
  T f = 1;
  T inf = numeric_limits<T>::infinity();
  size_t exp = 0;
  while (f != inf) {
    exp++;
    f *= 2;
  }
  cout << "Max exp: " << exp << endl;
}

template <std::floating_point T> T getMachineEpsilon() {
  T eps = 1;
  size_t bits = 0;
  while (true) {
    bits++;
    bool first = (1 + eps) != 1;
    bool second = (1 + eps / 2) == 1;
    if (first && second) {
      cout << "mantissa bits: " << --bits << endl;
      break;
    }
    eps /= 2;
  }
  return eps;
}

template <std::floating_point T> void getMinExp() {
  T f = 1;
  T inf = numeric_limits<T>::infinity();
  long exp = 0;
  while (std::isnormal(f)) {
    --exp;
    f /= 2;
  }
  cout << "Min exp: " << exp << endl;
}

template <std::floating_point T>
void checkEqAndGt(T f1, T f2, string s1, string s2) {
  bool b1 = f1 > f2;
  bool b2 = f1 == f2;
  cout << "(" + s1 + " > " + s2 + "): " << boolalpha << b1 << endl;
  cout << "(" + s1 + " == " + s2 + "): " << boolalpha << b2 << endl;
}

template <std::floating_point T> void checkOrdering(T eps) {
  T epsdiv = eps / 2;
  T one = 1;
  checkEqAndGt(one, eps, "1", "eps");
  checkEqAndGt(one, one + epsdiv, "1", "1 + eps / 2");
  checkEqAndGt(one + epsdiv, one + eps, "1 + eps / 2", "1 + eps");
  checkEqAndGt(one + eps, one + eps + epsdiv, "1 + eps", "1 + eps + eps / 2");
  checkEqAndGt(one + eps, one + epsdiv + eps, "1 + eps", "1 + eps / 2 + eps");
}

int main() {
  cout << "For float:" << endl;
  float mef = getMachineEpsilon<float>();
  cout << "Machine epsilon(ULP): " << mef;
  cout << endl << "Float ordering:" << endl;
  checkOrdering(mef);
  getMaxExp<float>();
  getMinExp<float>();
  cout << endl << "For double:" << endl;
  double med = getMachineEpsilon<double>();
  cout << "Machine epsilon(ULP): " << med;
  cout << endl << "Double ordering:" << endl;
  checkOrdering(med);
  getMaxExp<double>();
  getMinExp<double>();
}
