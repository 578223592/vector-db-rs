use faiss::{index_factory, MetricType};
use log::info;

mod faiss_index;
use faiss_index::FaissIndex;

fn main() -> faiss::error::Result<()> {
    env_logger::init();
    
    // 创建一个8维的Flat索引，使用L2距离
    let index = index_factory(8, "Flat", MetricType::L2)?;
    let mut faiss_index = faiss_index::FaissIndex::new(index);

    // 示例数据：插入一些8维向量
    let vectors = vec![
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0,
        8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
    ];
    
    // 插入向量
    faiss_index.insert_vectors(&vectors[..8], 0)?;
    faiss_index.insert_vectors(&vectors[8..], 1)?;

    info!("Inserted vectors with labels 0 and 1");

    // 搜索示例
    let query = vec![1.5, 2.5, 3.5, 4.5, 5.5, 6.5, 7.5, 8.5];
    let (indices, distances) = faiss_index.search_vectors(&query, 2)?;

    println!("Search results:");
    for (i, (idx, dist)) in indices.iter().zip(distances.iter()).enumerate() {
        println!("#{}: ID={}, Distance={}", i + 1, idx, dist);
    }

    Ok(())
}
