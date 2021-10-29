class <class_name>
{
    friend class CppStringCollectionFriend;
    foo_string_collection_t* self;
public:
    <class_name>(const std::vector<<value_type>>& values) : self(<create_funcion>(<reserve_size>))
    {
        for(auto x : values) {
             <add_function>(*self, x)
        }
    }

    ~<class_name>()
    {
        <destroy_function>(*self)
    }
};