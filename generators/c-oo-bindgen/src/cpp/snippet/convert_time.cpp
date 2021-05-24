constexpr double NANOSEC_PER_SEC = 1000000000.0;

std::chrono::steady_clock::duration from_sec_u64(uint64_t value) {
    return std::chrono::seconds(value);
}

std::chrono::steady_clock::duration from_msec_u64(uint64_t value) {
    return std::chrono::milliseconds(value);
}

std::chrono::steady_clock::duration from_sec_float(float value) {
    return std::chrono::nanoseconds(static_cast<uint64_t>(value * NANOSEC_PER_SEC));
}

uint64_t to_sec_u64(std::chrono::steady_clock::duration value) {
    return std::chrono::duration_cast<std::chrono::seconds>(value).count();
}

uint64_t to_ms_u64(std::chrono::steady_clock::duration value) {
    return std::chrono::duration_cast<std::chrono::milliseconds>(value).count();
}

float to_sec_float(std::chrono::steady_clock::duration value) {
    const auto nanos = std::chrono::duration_cast<std::chrono::nanoseconds>(value).count();
    return static_cast<float>(nanos / NANOSEC_PER_SEC);
}
