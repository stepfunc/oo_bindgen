/**
 * @brief Iterator over @ref <iter_type> instances
 *
 * @example
 *
 * // always use this pattern to extract values from the iterator
 * while(iter.next()) {
 *     <iter_type> value = iter.get();
 * }
 */
class <name> final {

    friend class Cpp<name>Friend;

    // underlying opaque c type
    void* iter;
    // pointer to the last retrieved c value
    void* current;

    // internal constructor
    <name>(void* iter) : iter(iter), current(nullptr) {}

    <name>() = delete; // no default construction
    <name>(const <name>&) = delete; // no copies
    <name>& operator=(const <name>&) = delete; // no self-assignment
    <name>& operator=(<name>&&) = delete; // no move self-assignment

public:

    /// @brief
    <name>(<name>&&) = default;

    /// @brief move the iterator to the next value
    /// @return true if another value is available
    bool next();

    /// @brief retrieve the current value
    /// @return current value of the iterator
    /// @throws std::logic_error if the last call to next() returns false or next() has never been called
    <iter_type> get();
};
