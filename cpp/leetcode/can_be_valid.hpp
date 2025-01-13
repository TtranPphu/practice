#include <bits/stdc++.h>
using namespace std;

namespace cbv {
class Solution {
 public:
  bool canBeValid(string s, string locked) {
    int n = s.size();
    if (n % 2) return false;
    int left = 0;
    int right = 0;
    for (int i = 0, j = n - 1; i < n; i++, j--) {
      if (s[i] == '(' || locked[i] == '0') {
        left++;
      } else {
        left--;
        if (left < 0) return false;
      }
      if (s[j] == ')' || locked[j] == '0') {
        right++;
      } else {
        right--;
        if (right < 0) return false;
      }
    }
    return true;
  }
};
}  // namespace cbv