error_chain! {
    foreign_links {
        Db(::diesel::result::Error);
    }
}

pub type HResult<T> = Result<T>;
