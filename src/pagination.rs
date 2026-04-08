use std::future::Future;

use serde::de::DeserializeOwned;

use crate::errors::RolloverError;
use crate::types::{ListOptions, Page};

/// Lazily iterates through pages from a list endpoint.
pub struct Iter<T, F, Fut>
where
    F: FnMut(ListOptions) -> Fut,
    Fut: Future<Output = Result<Page<T>, RolloverError>>,
{
    fetch: F,
    opts: ListOptions,
    page: Option<Page<T>>,
    err: Option<RolloverError>,
    done: bool,
}

/// Returns an iterator that lazily fetches one page at a time, defaulting to
/// 100 items per page.
pub fn pages<T, F, Fut>(fetch: F, opts: Option<ListOptions>) -> Iter<T, F, Fut>
where
    F: FnMut(ListOptions) -> Fut,
    Fut: Future<Output = Result<Page<T>, RolloverError>>,
{
    let mut o = opts.unwrap_or_default();
    if o.limit <= 0 {
        o.limit = 100;
    }
    Iter {
        fetch,
        opts: o,
        page: None,
        err: None,
        done: false,
    }
}

impl<T, F, Fut> Iter<T, F, Fut>
where
    T: DeserializeOwned,
    F: FnMut(ListOptions) -> Fut,
    Fut: Future<Output = Result<Page<T>, RolloverError>>,
{
    /// Fetches the next page, returning true if there are results to read.
    pub async fn next(&mut self) -> bool {
        if self.err.is_some() || self.done {
            return false;
        }

        match (self.fetch)(self.opts.clone()).await {
            Ok(page) => {
                let count = page.data.len() as i64;
                self.opts.offset += count;

                if count < self.opts.limit || self.opts.offset >= page.total {
                    self.done = true;
                }

                let has_data = count > 0;
                self.page = Some(page);
                has_data
            }
            Err(e) => {
                self.err = Some(e);
                false
            }
        }
    }

    /// Returns the most recently fetched page.
    pub fn page(&self) -> Option<&Page<T>> {
        self.page.as_ref()
    }

    /// Returns the first error encountered during iteration.
    pub fn err(&self) -> Option<&RolloverError> {
        self.err.as_ref()
    }

    /// Takes ownership of the error, if any.
    pub fn take_err(&mut self) -> Option<RolloverError> {
        self.err.take()
    }
}

/// Fetches all pages and returns every item in a single Vec.
pub async fn collect_all<T, F, Fut>(
    fetch: F,
    opts: Option<ListOptions>,
) -> Result<Vec<T>, RolloverError>
where
    T: DeserializeOwned,
    F: FnMut(ListOptions) -> Fut,
    Fut: Future<Output = Result<Page<T>, RolloverError>>,
{
    let mut iter = pages(fetch, opts);
    let mut all = Vec::new();

    while iter.next().await {
        if let Some(page) = iter.page.take() {
            if all.is_empty() {
                all.reserve(page.total as usize);
            }
            all.extend(page.data);
        }
    }

    if let Some(e) = iter.take_err() {
        return Err(e);
    }

    Ok(all)
}
