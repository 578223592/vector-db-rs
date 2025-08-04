use log::debug;

pub struct FaissIndex {
    index: Box<dyn faiss::Index>,
}

impl FaissIndex {
    /// 创建一个新的FaissIndex实例
    pub fn new(index: Box<dyn faiss::Index>) -> Self {
        FaissIndex { index }
    }

    /// 插入单个向量到索引中
    pub fn insert_vectors(&mut self, data: &[f32], label: u64) -> faiss::error::Result<()> {
        let id = faiss::Idx::new(label);
        self.index.add_with_ids(data, &[id])
    }

    /// 搜索向量，返回最相似的k个结果
    pub fn search_vectors(
        &mut self,
        query: &[f32],
        k: usize,
    ) -> faiss::error::Result<(Vec<i64>, Vec<f32>)> {
        let dim = self.index.d() as usize;
        let num_queries = query.len() / dim;

        let search_result = self.index
            .search(query, k, )?;
        let distances = search_result.distances;
        let indices = search_result.labels;

        debug!("Retrieved values:");
        let indices_i64: Vec<i64> = indices
            .iter()
            .map(|idx| idx.get().unwrap_or(-1_i64 as u64) as i64)
            .collect();
        
        for i in 0..indices_i64.len() {
            if indices_i64[i] != -1 {
                debug!("ID: {}, Distance: {}", indices_i64[i], distances[i]);
            } else {
                debug!("No specific value found");
            }
        }

        Ok((indices_i64, distances))
    }
}