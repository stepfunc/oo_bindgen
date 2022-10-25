
std::chrono::steady_clock::duration from_sec_u64(uint64_t value) {
    return std::chrono::seconds(value);
}

std::chrono::steady_clock::duration from_milli_sec_u64(uint64_t value) {
    return std::chrono::milliseconds(value);
}

uint64_t to_sec_u64(std::chrono::steady_clock::duration value) {
    return std::chrono::duration_cast<std::chrono::seconds>(value).count();
}

uint64_t to_milli_sec_u64(std::chrono::steady_clock::duration value) {
    return std::chrono::duration_cast<std::chrono::milliseconds>(value).count();
}

