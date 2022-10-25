class Cpp<cpp_type>Friend final {

public:

    static <cpp_type> init(<c_type>* value)
    {
        return <cpp_type>(value);
    }

    static <c_type>* get(const <cpp_type>& value)
    {
        return reinterpret_cast<<c_type>*>(value.self);
    }
};
