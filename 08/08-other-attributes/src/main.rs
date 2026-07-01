use other_attributes_macro::analyze;

analyze!(
    /** comment block */
    /// outer comment
    struct Example {
        /*! inner comment block */
        //! inner comment
        val: String
    }
);

fn main() {}
