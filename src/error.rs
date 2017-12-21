error_chain! {
    foreign_links {
        Db(::diesel::result::Error);
    }

    errors {
        InvalidRecord {}
    }
}

pub type HResult<T> = Result<T>;
