/**
 * @brief Generic abstract iterator type that doesn't require a collection
 *
 * @example
 *
 * // always use this pattern to extract values from an iterator
 * while(iter.next()) {
 *     auto value = iter.get();
 * }
 */
template <class T>
class Iterator {
  public:
    virtual ~Iterator() = default;

    /// @brief move the iterator to the next value
    /// @return
    virtual bool next() = 0;

    /// @brief retrieve the current value or throw std::logic_error
    /// @throws std::logic_error if the last call to next() returns false or next() has never been called.
    virtual T get() const = 0;
};
