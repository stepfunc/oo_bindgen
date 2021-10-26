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

    void* iter;
    void* current;

    <name>(void* iter) : iter(iter), current(nullptr) {}

public:

    <name>(<name>&&) = default;

    <name>() = delete; // no default construction
    <name>(const <name>&) = delete; // no copies
    <name>& operator=(const <name>&) = delete; // no self-assignment
    <name>& operator=(<name>&&) = delete; // no movie self-assignment


    /// @brief move the iterator to the next value
    /// @return
    bool next();

    /// @brief retrieve the current value
    /// @throws std::logic_error if the last call to next() returns false or next() has never been called.
    <iter_type> get();
};
