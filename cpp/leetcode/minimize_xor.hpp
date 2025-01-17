#include <bits/stdc++.h>
using namespace std;

namespace minimize_xor {
class Solution {
 public:
  int minimizeXor(int num1, int num2) {
    auto n1 = popcount(uint(num1));
    auto n2 = popcount(uint(num2));
    if (n1 > n2) {
      auto n = n1 - n2;
      while (n > 0) {
        num1 &= num1 - 1;
        n--;
      }
    } else if (n2 > n1) {
      auto n = n2 - n1;
      while (n > 0) {
        num1 |= num1 + 1;
        n--;
      }
    }
    return num1;
  }
};
}  // namespace minimize_xor