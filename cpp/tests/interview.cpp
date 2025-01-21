#include <gtest/gtest.h>

#include "larion_wq.hpp"

namespace {
TEST(InterviewTest, InfectedTest) {
  auto solution = larion_wq::Solution();
  {
    int n = 3;
    vector<int> initial_infected{0};
    vector<vector<int>> updates{{0, 1, 2}, {1, 0, 2}, {0, 2, 1}};
    vector<int> expected{0, -1, -1};
    EXPECT_EQ(solution.infected(n, initial_infected, updates), expected);
  }
  {
    int n = 5;
    vector<int> initial_infected{2, 3};
    vector<vector<int>> updates{{0, 0, 1},  {1, 1, -1}, {0, 3, 2},
                                {1, 0, -1}, {0, 4, 0},  {0, 2, 1}};
    vector<int> expected{-1, -1, 0, 0, 6};
    EXPECT_EQ(solution.infected(n, initial_infected, updates), expected);
  }
}
}  // namespace