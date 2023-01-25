pub enum DBWorkerRequest<K, V> {
    Insert(K, V),
    Get(K),
}

pub enum DBWorkerResponse<V> {
    Some(V),
    None,
}
