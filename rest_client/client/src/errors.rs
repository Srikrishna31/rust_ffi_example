error_chain! {
        types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        Reqwest(::reqwest::Error);
    }
}
