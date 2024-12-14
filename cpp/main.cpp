#include "simple_logger/simple_logger.hpp"

using namespace simple_logger;

int main() {
  logger.Trace() << "This message shouldn't be printed by default!";
  logger.Debug() << "This message shouldn't be printed by default!";
  logger.Info() << "Info message: <" << 8888 << '>';
  logger.Warn() << "Warn message: <" << 8888 << '>';
  logger.Error() << "Error message: <" << 8888 << '>';
  logger.Fatal() << "Fatal message: <" << 8888 << '>';
  return 0;
}
