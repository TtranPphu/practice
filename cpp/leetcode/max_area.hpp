#include <bits/stdc++.h>
using namespace std;

namespace max_area {
class Solution {
 public:
  int maxArea(vector<int>& height) {
    int i = 0, j = height.size() - 1;
    int res = (j - i) * min(height[j], height[i]);
    do {
      if (height[i] < height[j]) {
        i++;
      } else {
        j--;
      }
      res = max(res, (j - i) * min(height[j], height[i]));
    } while (i < j);
    return res;
  }
};
}  // namespace max_area