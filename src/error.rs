error_chain! {
    foreign_links {
        Io(::std::io::Error);
    }
    errors {
        UnsupportedDeserializerMethod(t: String) {
            description("Deserializer method is not supported in ROSMSG")
                display("Deserializer method is not supported in ROSMSG: {}", t)
        }
        EndOfBuffer {
            description("Reached end of memory buffer")
                display("Reached end of memory buffer while reading data")
        }
        BadStringData {
            description("Strings need to be UTF-8")
                display("Strings need to be UTF-8")
        }
        UnexpectedType(t: String) {
            description("Type was not expected by the deserializer")
                display("Type was not expected by the deserializer: {}", t)
        }
        UnsupportedCharType {
            description("Chars are not supported in ROSMSG")
                display("Chars are not supported in ROSMSG")
        }
        UnsupportedEnumType {
            description("Enumerations are not supported in ROSMSG")
                display("Enumerations are not supported in ROSMSG")
        }
        UnsupportedMapType {
            description("Maps are not supported in ROSMSG")
                display("Maps are not supported in ROSMSG")
        }
        VariableArraySizeAnnotation {
            description("Size annotation in variable size array is missing")
                display("Size annotation in variable size array is missing")
        }
    }
}
