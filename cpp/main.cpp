#include <chrono>
#include <fstream>
#include <iostream>
#include <thread>

#include "log_util.hpp"

int main(int argc, char* argv[]) {
  using namespace std;
  int iter = 1e8;

  fstream cpp_file("cpp_log.txt", ios::out);
  auto c_file = fopen("c_log.txt", "w");

  std::ios::sync_with_stdio(false);

  cpp_file.rdbuf()->pubsetbuf(nullptr, 0);  // Disable buffering for fstream
  setvbuf(c_file, nullptr, _IONBF, 0);      // Disable buffering for stdio

  auto cpp_start = chrono::high_resolution_clock::now();
  for (int i = 0; i < iter; ++i) {
    cpp_file << i;
  }
  auto cpp_time = chrono::high_resolution_clock::now() - cpp_start;

  auto c_start = chrono::high_resolution_clock::now();
  for (int i = 0; i < iter; ++i) {
    fprintf(c_file, "%d", i);
  }
  auto c_time = chrono::high_resolution_clock::now() - c_start;

  const auto ns = 1e9;

  cout << "cpp time: " << (float)cpp_time.count() / ns << endl;
  cout << "c time: " << (float)c_time.count() / ns << endl;
  cout << (float)cpp_time.count() / c_time.count() << endl;

  cpp_file.close();
  fclose(c_file);
}
