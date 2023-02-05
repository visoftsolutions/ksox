#[derive(Debug)]
pub enum DBWorkerRequest<K, V> {
    Insert(V),
    Get(K),
    Update(V),
    TerminateThread,
}

#[derive(Debug)]
pub enum DBWorkerResponse<V> {
    Some(V),
    None,
}
