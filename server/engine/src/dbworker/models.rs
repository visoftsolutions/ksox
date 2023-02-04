#[derive(Debug)]
pub enum DBWorkerRequest<K, V> {
    Insert(V),
    Get(K),
    TerminateThread,
}

#[derive(Debug)]
pub enum DBWorkerResponse<V> {
    Some(V),
    None,
}
