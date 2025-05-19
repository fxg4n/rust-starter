pub fn chunk_vec<T: Clone>(vec: &[T], chunk_size: usize) -> Vec<Vec<T>> {
    vec.chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect()
}
