#include <bits/stdc++.h>
using namespace std;

namespace solve_n_queen {
class Solution {
 public:
  vector<vector<string>> solveNQueens(int n) {
    vector<vector<int>> results;

    auto is_valid = [&](int row, int col, const vector<int>& result) {
      for (int i = 0; i < row; i++) {
        if (result[i] == col || abs(i - row) == abs(result[i] - col)) {
          return false;
        }
      }
      return true;
    };

    function<void(int, vector<int>&)> next;
    next = [&](int row, vector<int>& result) {
      if (row == n) {
        results.push_back(result);
        return;
      }
      for (int col = 0; col < n; col++) {
        if (is_valid(row, col, result)) {
          result.push_back(col);
          next(row + 1, result);
          result.pop_back();
        }
      }
    };

    auto result = vector<int>();
    next(0, result);

    vector<vector<string>> return_results;
    for (const auto& result : results) {
      vector<string> return_result;
      for (int i = 0; i < n; i++) {
        string row(n, '.');
        row[result[i]] = 'Q';
        return_result.push_back(row);
      }
      return_results.push_back(return_result);
    }
    return return_results;
  }
};
}  // namespace solve_n_queen