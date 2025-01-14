#pragma once
#include "spdlog/sinks/daily_file_sink.h"
#include "spdlog/sinks/stdout_color_sinks.h"
#include "spdlog/spdlog.h"

namespace log_util {
auto default_logger = spdlog::get("default_logger");
void init_logger() {
  auto console_sink = std::make_shared<spdlog::sinks::stdout_color_sink_mt>();
  console_sink.get()->set_level(spdlog::level::debug);
}
}  // namespace log_util
