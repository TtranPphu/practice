#include <bits/stdc++.h>
using namespace std;

namespace lols {
class Solution {
 public:
  int lengthOfLongestSubstring(string s) {
    int n = s.length(), ans = 0;
    unordered_map<char, int> mp;
    for (int i = 0, j = 0; j < n; j++) {
      if (mp.find(s[j]) != mp.end()) {
        i = max(mp[s[j]], i);
      }
      ans = max(ans, j - i + 1);
      mp[s[j]] = j + 1;
    }
    return ans;
  }
};
}  // namespace lols