// // use std::collections::HashMap;
// // use xxhash_rust::xxh3::Xxh3;
// // use rand::Rng;

// // // 全局的最大度数常量
// // pub const MAX_DEGREE: usize = 50;

// // #[derive(Debug, Clone, Copy)]
// // pub struct NodeData {
// //     pub vector: [f32; 128],
// //     pub neighbors: [usize; MAX_DEGREE], 
// // }

// // impl NodeData {
// //     pub fn new_empty() -> Self {
// //         NodeData {
// //             vector: [0.0; 128],
// //             neighbors: [usize::MAX; MAX_DEGREE], 
// //         }
// //     }
// // }

// // /// 计算最大公约数 (Euclidean algorithm)
// // /// 用于确保 LCG 参数 a 与 容量 M 互质
// // fn gcd(mut a: usize, mut b: usize) -> usize {
// //     while b != 0 {
// //         let temp = b;
// //         b = a % b;
// //         a = temp;
// //     }
// //     a
// // }

// // pub struct UnifiedPool {
// //     pub hub_map: HashMap<usize, NodeData>,
    
// //     pub flat_pool: Vec<NodeData>,             
    
// //     pub total_normal_nodes: usize, 
// //     pub normal_shares: usize,    // 每个节点的副本数 (K)
// //     pub size_m: usize,           // 物理容量总量 M = N * K
    
// //     pub node_nth_table: Vec<u8>, 
// //     pub hub_id_list: Vec<usize>, 
    
// //     // 完美哈希(LCG)参数：h(x) = (a * x + b) % M
// //     pub param_a: usize,
// //     pub param_b: usize,
// // }

// // impl UnifiedPool {

// //     pub fn new(total_normal_nodes: usize, normal_shares: usize, batch_number: usize) -> Self {
// //         let size_m = total_normal_nodes * normal_shares;
        
// //         // 利用 Xxh3 将 batch_number 散列以获取随机种子
// //         let mut hasher = Xxh3::new();
// //         hasher.update(&batch_number.to_le_bytes());
// //         let hash_val = hasher.digest() as usize;
        
// //         // 根据 Batch 生成互质的 a 和随机的 b
// //         let mut a = (hash_val >> 32) % size_m;
// //         let b = (hash_val & 0xFFFFFFFF) % size_m;
        
// //         // 兜底防御：防止 a 初始值为 0
// //         if a == 0 {
// //             a = 1;
// //         }

// //         // 动态寻找与 size_m 互质的 a
// //         while gcd(a, size_m) != 1 {
// //             a = (a + 1) % size_m;
// //             if a == 0 { a = 1; } 
// //         }

// //         UnifiedPool {
// //             hub_map: HashMap::new(),
// //             // 预分配整块连续内存，一次性占位
// //             flat_pool: vec![NodeData::new_empty(); size_m],
// //             total_normal_nodes,
// //             normal_shares,
// //             size_m,
// //             node_nth_table: vec![0; total_normal_nodes], 
// //             hub_id_list: Vec::new(),
// //             param_a: a,
// //             param_b: b,
// //         }
// //     }

// //     /// O(1) 的完美哈希映射函数
// //     #[inline(always)]
// //     fn perfect_hash(&self, node_id: usize, nth: usize) -> usize {
// //         // x' = id * K + copy_idx
// //         let x_prime = node_id * self.normal_shares + nth;
// //         // h(x') = (a * x' + b) mod M
// //         (self.param_a.wrapping_mul(x_prime).wrapping_add(self.param_b)) % self.size_m
// //     }

// //     pub fn insert_hub(&mut self, node_id: usize, node_data: NodeData) {
// //         self.hub_map.insert(node_id, node_data);
// //         self.hub_id_list.push(node_id);
// //     }

// //     pub fn insert_normal(
// //         &mut self, 
// //         node_id: usize, 
// //         node_nth: usize,     
// //         node_data: NodeData
// //     ) {
// //         // 直接计算绝对物理地址并写入
// //         let index = self.perfect_hash(node_id, node_nth); 
// //         self.flat_pool[index] = node_data;
// //     }

// //     /// Phase 1 专用 (前 15%)：带有混淆访问逻辑的获取方式
// //     pub fn get(&mut self, target_id: usize) -> NodeData {
// //         let mut rng = rand::thread_rng();
        
// //         // 1. 获取 mask（1: 中心节点, 0: 普通节点）
// //         let is_hub_mask = self.hub_map.contains_key(&target_id) as usize; 

// //         // 2. 伪装目标生成
// //         let random_normal_id = rng.gen_range(0..self.total_normal_nodes); 
        
// //         // 兜底防御，防止 hub_id_list 为空导致 panic
// //         let random_hub_id = if !self.hub_id_list.is_empty() {
// //             let random_hub_idx = rng.gen_range(0..self.hub_id_list.len());
// //             self.hub_id_list[random_hub_idx]
// //         } else {
// //             0
// //         };

// //         // 3. 确定双端查询目标
// //         let actual_hub_target = is_hub_mask * target_id + (1 - is_hub_mask) * random_hub_id;
// //         let actual_hash_target = is_hub_mask * random_normal_id + (1 - is_hub_mask) * target_id;

// //         // 动作 A：查 Map
// //         let empty_hub_data = NodeData::new_empty();
// //         let hub_data_candidate = self.hub_map.get(&actual_hub_target).unwrap_or(&empty_hub_data);

// //         // 动作 B：查展平的完美哈希表
// //         let table_len = self.node_nth_table.len();
// //         let nth_index = actual_hash_target % table_len;
        
// //         let current_nth = self.node_nth_table[nth_index] as usize;
// //         self.node_nth_table[nth_index] = self.node_nth_table[nth_index].wrapping_add(1);

// //         // 利用完美哈希一步到位
// //         let index = self.perfect_hash(actual_hash_target, current_nth); 
// //         let normal_data_candidate = self.flat_pool[index];

// //         // 4. 数组索引选择
// //         let final_choices = [&normal_data_candidate, hub_data_candidate];
// //         *final_choices[is_hub_mask]
// //     }

// //     /// Phase 2 专用 (后 85%)：跳过中心节点混淆，直接从普通 Hash 表极速提取数据
// //     pub fn get_normal_only(&mut self, target_id: usize) -> NodeData {
// //         let table_len = self.node_nth_table.len();
// //         let nth_index = target_id % table_len;
        
// //         let current_nth = self.node_nth_table[nth_index] as usize;
// //         self.node_nth_table[nth_index] = self.node_nth_table[nth_index].wrapping_add(1);

// //         // 利用完美哈希计算位置
// //         let index = self.perfect_hash(target_id, current_nth); 
        
// //         self.flat_pool[index]
// //     }
// // }

// use std::collections::HashMap;
// use xxhash_rust::xxh3::Xxh3;
// use rand::Rng;
// use rayon::prelude::*;
// use rand::SeedableRng;
// use rand::rngs::SmallRng;

// // ==========================================
// // 全局配置与基础数据结构
// // ==========================================

// // 全局的最大度数常量 (Python 自动化脚本会动态替换这里的值)
// pub const MAX_DEGREE: usize = 50;

// #[derive(Debug, Clone, Copy)]
// pub struct NodeData {
//     pub vector: [f32; 128], // Python 脚本会匹配替换这里的 128
//     pub neighbors: [usize; MAX_DEGREE], 
// }

// impl NodeData {
//     pub fn new_empty() -> Self {
//         NodeData {
//             vector: [0.0; 128], // Python 脚本会匹配替换这里的 128
//             neighbors: [usize::MAX; MAX_DEGREE], 
//         }
//     }
// }

// // ==========================================
// // 底层密码学与无分支安全组件
// // ==========================================

// /// 计算最大公约数 (Euclidean algorithm)
// /// 用于确保 LCG 参数 a 与物理容量 M 互质，保证哈希映射的绝对双射（无碰撞）
// fn gcd(mut a: usize, mut b: usize) -> usize {
//     while b != 0 {
//         let temp = b;
//         b = a % b;
//         a = temp;
//     }
//     a
// }

// /// 无分支条件交换 (Oblivious Conditional Swap)
// /// 利用异或 (XOR) 和位掩码 (Bitmask) 实现
// #[inline(always)]
// fn conditional_swap(arr: &mut [usize], i: usize, j: usize, condition: bool) {
//     if i != j { 
//         // 这个 if 只依赖随机数索引，与真实数据无关，不会造成泄露
//         // 如果 condition 为 true，mask 全为 1 (usize::MAX)；否则全为 0
//         let mask = (0usize).wrapping_sub(condition as usize);
//         let temp = (arr[i] ^ arr[j]) & mask;
//         arr[i] ^= temp;
//         arr[j] ^= temp;
//     }
// }

// // ==========================================
// // 核心结构：统一内存池 (UnifiedPool)
// // ==========================================

// pub struct UnifiedPool {
//     pub hub_map: HashMap<usize, NodeData>,       // 驻留在 L3 Cache 的 Hub 节点池 (P_h0)
//     pub flat_pool: Vec<NodeData>,                // 驻留在 DRAM 的普通节点池 (P_n)，以一维平铺存放
//     pub total_normal_nodes: usize,               // 节点总数 N
//     pub normal_shares: usize,                    // 每个节点的副本数 K
//     pub size_m: usize,                           // 物理容量总量 M = N * K
//     pub node_nth_table: Vec<u8>,                 // 记录每个节点已被访问的次数，防止同一请求访问相同副本
//     pub hub_id_list: Vec<usize>,                 // Hub 节点 ID 列表，用于随机伪装
//     pub param_a: usize,                          // LCG 完美哈希参数 a
//     pub param_b: usize,                          // LCG 完美哈希参数 b
// }

// impl UnifiedPool {

//     /// 系统初始化：负责分配底层物理内存。
//     /// 触发操作系统的缺页中断和清零，耗时较长，但不应算作算法耗时。
//     pub fn new(total_normal_nodes: usize, normal_shares: usize, batch_number: usize) -> Self {
//         let size_m = total_normal_nodes * normal_shares;
        
//         let mut pool = UnifiedPool {
//             hub_map: HashMap::new(),
//             flat_pool: vec![NodeData::new_empty(); size_m], // 向 OS 申请完整连续物理内存
//             total_normal_nodes,
//             normal_shares,
//             size_m,
//             node_nth_table: vec![0; total_normal_nodes], 
//             hub_id_list: Vec::new(),
//             param_a: 1, // 占位符，将在 refresh_for_new_batch 中计算
//             param_b: 0,
//         };

//         // 计算初始的 LCG 参数
//         pool.refresh_for_new_batch(batch_number);
//         pool
//     }

//     /// 轻量级批次刷新：用于在同一块物理内存上开启新批次的重构。
//     /// 只更新哈希参数，绝对不重新分配 91GB 的 flat_pool，耗时几乎为 0。
//     pub fn refresh_for_new_batch(&mut self, new_batch_number: usize) {
//         let mut hasher = Xxh3::new();
//         hasher.update(&new_batch_number.to_le_bytes());
//         let hash_val = hasher.digest() as usize;
        
//         let mut a = (hash_val >> 32) % self.size_m;
//         let b = (hash_val & 0xFFFFFFFF) % self.size_m;
        
//         if a == 0 { a = 1; }
//         // 动态寻找与 size_m 互质的 a
//         while gcd(a, self.size_m) != 1 {
//             a = (a + 1) % self.size_m;
//             if a == 0 { a = 1; } 
//         }

//         self.param_a = a;
//         self.param_b = b;
//         //清空所有节点的访问计数
//         self.node_nth_table.fill(0); 
//     }

//     /// O(1) 的完美哈希映射函数: f(x') = (a * x' + b) mod M
//     #[inline(always)]
//     fn perfect_hash(&self, node_id: usize, nth: usize) -> usize {
//         let x_prime = node_id * self.normal_shares + nth;
//         (self.param_a.wrapping_mul(x_prime).wrapping_add(self.param_b)) % self.size_m
//     }

//     /// =========================================================================
//     /// 核心算法 2：Oblivious Single-Use Index (OSUI) 重建 (极致性能版)
//     /// =========================================================================
//     pub fn oblivious_reconstruct(&mut self, database: &[NodeData], hub_nodes: &[usize]) {
        
//         println!("==> 开始执行 Oblivious Index Reconstruction (Algorithm 2)...");
//         let start_time = std::time::Instant::now();
        
//         // ---------------------------------------------------------
//         // Phase 1: Obliviously Build P_h0 (Hub 节点无分支提取)
//         // ---------------------------------------------------------
//         println!("    --> Phase 1: 构建 Hub Pool...");
//         let mut in_h = vec![0usize; self.total_normal_nodes];
//         for &hid in hub_nodes {
//             in_h[hid] = 1;
//         }

//         let mut p_h0 = vec![NodeData::new_empty(); hub_nodes.len()];
//         let mut j = 0;
        
//         // 遍历全体数据，无脑写入，利用游标 j 的有条件累加隐藏真实提取路径
//         for i in 0..self.total_normal_nodes {
//             let safe_j = if j < p_h0.len() { j } else { 0 };
//             p_h0[safe_j] = database[i]; 
//             j += in_h[i];               
//         }

//         self.hub_map.clear();
//         self.hub_id_list.clear();
//         for (idx, &hid) in hub_nodes.iter().enumerate() {
//             self.hub_map.insert(hid, p_h0[idx]);
//             self.hub_id_list.push(hid);
//         }

//         // ---------------------------------------------------------
//         // Phase 2: Obliviously Build POS Array (生成普通节点映射表)
//         // ---------------------------------------------------------
//         println!("    --> Phase 2: 计算随机映射 pos 数组...");
//         let mut a1: Vec<usize> = (0..self.total_normal_nodes).collect(); // 存 Node ID
//         let mut a2: Vec<usize> = vec![0; self.total_normal_nodes];       // 存副本已分配计数
//         let mut pos: Vec<usize> = vec![0; self.size_m];                  // 最终映射表
        
//         let mut remaining_nodes = self.total_normal_nodes;
//         // 使用非密码学的极速 SmallRng 大幅削减千万级循环的 CPU 耗时
//         let mut rng = SmallRng::from_entropy();

//         while remaining_nodes > 0 {
//             let tail = remaining_nodes - 1;
//             let i = rng.gen_range(0..=tail);

//             // 获取洗牌后的映射目标地址并记录
//             let mapped_idx = self.perfect_hash(a1[i], a2[i]);
//             pos[mapped_idx] = a1[i];
            
//             a2[i] += 1;
//             let is_full = a2[i] == self.normal_shares;

//             // 无分支条件交换：塞满的节点隐蔽地移出洗牌区
//             conditional_swap(&mut a1, i, tail, is_full);
//             conditional_swap(&mut a2, i, tail, is_full);

//             remaining_nodes -= is_full as usize;
//         }

//         // // ---------------------------------------------------------
//         // // Phase 3: Chunked Copy to P_n (利用 Rayon 多线程榨干带宽)
//         // // ---------------------------------------------------------
//         // println!("    --> Phase 3: 分块并行拷贝数据到 P_n (DRAM)...");
        
//         // // 动态获取数据集的真实维度和真实度数，适配泛化需求
//         // let dim = database.first().map_or(128, |n| n.vector.len());
//         // let max_deg = MAX_DEGREE;
        
//         // // 3.1 拷贝 Vector 数据
//         // let vec_chunk_size = 32;
//         // // 向上取整计算需要的轮数，确保不能被 32 整除的维度也能完全复制
//         // let vec_passes = (dim + vec_chunk_size - 1) / vec_chunk_size; 
        
//         // for pass in 0..vec_passes {
//         //     let start_dim = pass * vec_chunk_size;
//         //     // 动态边界保护，最后一次可能不够 chunk_size
//         //     let end_dim = std::cmp::min(start_dim + vec_chunk_size, dim);
//         //     let actual_chunk_size = end_dim - start_dim;
            
//         //     // 缓存层 A_3
//         //     let mut a3 = vec![0.0f32; self.total_normal_nodes * actual_chunk_size];
//         //     for i in 0..self.total_normal_nodes {
//         //         a3[i * actual_chunk_size .. (i + 1) * actual_chunk_size]
//         //             .copy_from_slice(&database[i].vector[start_dim..end_dim]);
//         //     }
            
//         //     // 利用 Rayon 并行化向 DRAM 写入，耗尽物理内存总线带宽
//         //     self.flat_pool.par_iter_mut().zip(pos.par_iter()).for_each(|(pool_node, &src_node_id)| {
//         //         pool_node.vector[start_dim..end_dim]
//         //             .copy_from_slice(&a3[src_node_id * actual_chunk_size .. (src_node_id + 1) * actual_chunk_size]);
//         //     });
//         // }

//         // // 3.2 拷贝 Neighbors 数据
//         // let neighbor_chunk_size = 25;
//         // let neighbor_passes = (max_deg + neighbor_chunk_size - 1) / neighbor_chunk_size;
        
//         // for pass in 0..neighbor_passes {
//         //     let start_dim = pass * neighbor_chunk_size;
//         //     let end_dim = std::cmp::min(start_dim + neighbor_chunk_size, max_deg);
//         //     let actual_chunk_size = end_dim - start_dim;
            
//         //     let mut a3_neighbors = vec![0usize; self.total_normal_nodes * actual_chunk_size];
//         //     for i in 0..self.total_normal_nodes {
//         //         a3_neighbors[i * actual_chunk_size .. (i + 1) * actual_chunk_size]
//         //             .copy_from_slice(&database[i].neighbors[start_dim..end_dim]);
//         //     }
            
//         //     self.flat_pool.par_iter_mut().zip(pos.par_iter()).for_each(|(pool_node, &src_node_id)| {
//         //         pool_node.neighbors[start_dim..end_dim]
//         //             .copy_from_slice(&a3_neighbors[src_node_id * actual_chunk_size .. (src_node_id + 1) * actual_chunk_size]);
//         //     });
//         // }

//                 // ---------------------------------------------------------
//         // Phase 3: Chunked Copy to P_n (动态感知 L3 Cache 防溢出)
//         // ---------------------------------------------------------
//         println!("    --> Phase 3: 分块并行拷贝数据到 P_n (DRAM)...");
        
//         let dim = database.first().map_or(128, |n| n.vector.len());
//         let max_deg = MAX_DEGREE;
        
//         // 【关键配置】：设定分配给本算法的 L3 Cache 安全上限。
//         // 假设你的一般服务器有 36MB L3，留一点给系统，这里设为 30MB。
//         // 如果你的服务器 L3 Cache 很大 (如 AMD EPYC 256MB)，可以调大这个值以大幅缩短重建时间！
//         let safe_l3_cache_bytes = 50 * 1024 * 1024; // 30 MB
        
//         // 3.1 动态计算向量的自适应分块
//         // 每次可容纳的 f32 元素个数 = 缓存上限 / (节点总数 * 4字节)
//         let vec_chunk_size = safe_l3_cache_bytes / (self.total_normal_nodes * 4);
//         let vec_chunk_size = std::cmp::max(1, vec_chunk_size); // 兜底至少搬 1 维
//         let vec_passes = (dim + vec_chunk_size - 1) / vec_chunk_size; 
        
//         println!("        [*] 动态参数 -> L3缓存限制: {}MB, 向量维度: {}, 每次拷贝维度: {}, 共需 {} 轮...", 
//                  safe_l3_cache_bytes / 1024 / 1024, dim, vec_chunk_size, vec_passes);

//         for pass in 0..vec_passes {
//             let start_dim = pass * vec_chunk_size;
//             let end_dim = std::cmp::min(start_dim + vec_chunk_size, dim);
//             let actual_chunk_size = end_dim - start_dim;
            
//             // 严格控制在安全缓存范围内的 a3
//             let mut a3 = vec![0.0f32; self.total_normal_nodes * actual_chunk_size];
//             for i in 0..self.total_normal_nodes {
//                 a3[i * actual_chunk_size .. (i + 1) * actual_chunk_size]
//                     .copy_from_slice(&database[i].vector[start_dim..end_dim]);
//             }
            
//             self.flat_pool.par_iter_mut().zip(pos.par_iter()).for_each(|(pool_node, &src_node_id)| {
//                 pool_node.vector[start_dim..end_dim]
//                     .copy_from_slice(&a3[src_node_id * actual_chunk_size .. (src_node_id + 1) * actual_chunk_size]);
//             });
//         }

//         // 3.2 动态计算邻居的自适应分块
//         // 每次可容纳的 usize 元素个数 = 缓存上限 / (节点总数 * 8字节)
//         let neighbor_chunk_size = safe_l3_cache_bytes / (self.total_normal_nodes * 8);
//         let neighbor_chunk_size = std::cmp::max(1, neighbor_chunk_size);
//         let neighbor_passes = (max_deg + neighbor_chunk_size - 1) / neighbor_chunk_size;
        
//         println!("        [*] 动态参数 -> 邻居度数: {}, 每次拷贝度数: {}, 共需 {} 轮...", 
//                  max_deg, neighbor_chunk_size, neighbor_passes);

//         for pass in 0..neighbor_passes {
//             let start_dim = pass * neighbor_chunk_size;
//             let end_dim = std::cmp::min(start_dim + neighbor_chunk_size, max_deg);
//             let actual_chunk_size = end_dim - start_dim;
            
//             let mut a3_neighbors = vec![0usize; self.total_normal_nodes * actual_chunk_size];
//             for i in 0..self.total_normal_nodes {
//                 a3_neighbors[i * actual_chunk_size .. (i + 1) * actual_chunk_size]
//                     .copy_from_slice(&database[i].neighbors[start_dim..end_dim]);
//             }
            
//             self.flat_pool.par_iter_mut().zip(pos.par_iter()).for_each(|(pool_node, &src_node_id)| {
//                 pool_node.neighbors[start_dim..end_dim]
//                     .copy_from_slice(&a3_neighbors[src_node_id * actual_chunk_size .. (src_node_id + 1) * actual_chunk_size]);
//             });
//         }
        
//         println!("==> Oblivious Index Reconstruction 完成！纯算法耗时: {:?}", start_time.elapsed());
//     }

//     // ==========================================
//     // 搜索期获取函数
//     // ==========================================

//    /// Phase 1 专用 (前 15%)：带有混淆访问逻辑的双端提取方式
//     pub fn get(&mut self, target_id: usize) -> NodeData {
//         let mut rng = rand::thread_rng();
//         let is_hub_mask = self.hub_map.contains_key(&target_id) as usize; 

//         let random_normal_id = rng.gen_range(0..self.total_normal_nodes); 
//         let random_hub_id = if !self.hub_id_list.is_empty() {
//             let random_hub_idx = rng.gen_range(0..self.hub_id_list.len());
//             self.hub_id_list[random_hub_idx]
//         } else {
//             0
//         };

//         let actual_hub_target = is_hub_mask * target_id + (1 - is_hub_mask) * random_hub_id;
//         let actual_hash_target = is_hub_mask * random_normal_id + (1 - is_hub_mask) * target_id;

//         let empty_hub_data = NodeData::new_empty();
//         let hub_data_candidate = self.hub_map.get(&actual_hub_target).unwrap_or(&empty_hub_data);

//         // 获取并递增访问次数，使用 saturating_add 防止 u8 溢出归零
//         let table_len = self.node_nth_table.len();
//         let nth_index = actual_hash_target % table_len;
//         let current_nth = self.node_nth_table[nth_index] as usize;
//         self.node_nth_table[nth_index] = self.node_nth_table[nth_index].saturating_add(1);

//         // 【安全拦截逻辑】如果该节点的访问次数耗尽
//         let normal_data_candidate = if current_nth >= self.normal_shares {
//             // 必须进行一次 Dummy Read（伪装读取），防止攻击者通过 DRAM 访问量的骤减猜出节点耗尽
//             // 我们随便读一个保证不会越界的位置（比如 0 号位置）
//             let dummy_index = self.perfect_hash(0, 0); 
            
//             // 使用 std::hint::black_box 防止编译器把这句没用的读取给优化掉 (需确信编译器没有过度优化，这里简单读取即可)
//             let _dummy_data = self.flat_pool[dummy_index]; 
            
//             // 返回死胡同节点
//             NodeData::new_empty()
//         } else {
//             // 份额正常，进行真实的 O(1) 映射提取
//             let index = self.perfect_hash(actual_hash_target, current_nth); 
//             self.flat_pool[index]
//         };

//         let final_choices = [&normal_data_candidate, hub_data_candidate];
//         *final_choices[is_hub_mask]
//     }

//     /// Phase 2 专用 (后 85%)：剥离多余动作，直接从普通 Hash 表极速提取数据
//     pub fn get_normal_only(&mut self, target_id: usize) -> NodeData {
//         let table_len = self.node_nth_table.len();
//         let nth_index = target_id % table_len;
        
//         let current_nth = self.node_nth_table[nth_index] as usize;
//         self.node_nth_table[nth_index] = self.node_nth_table[nth_index].saturating_add(1);

//         // 【安全拦截逻辑】
//         if current_nth >= self.normal_shares {
//             // 执行伪装访存掩盖行为
//             let dummy_index = self.perfect_hash(0, 0); 
//             let _dummy_data = self.flat_pool[dummy_index]; 
            
//             return NodeData::new_empty();
//         }

//         let index = self.perfect_hash(target_id, current_nth); 
//         self.flat_pool[index]
//     }

// }



use std::collections::HashMap;
use xxhash_rust::xxh3::Xxh3;
use rand::Rng;
use rayon::prelude::*;
use rand::SeedableRng;
use rand::rngs::SmallRng;

// ==========================================
// 全局配置与基础数据结构
// ==========================================

pub const MAX_DEGREE: usize = 50;

#[derive(Debug, Clone, Copy)]
pub struct NodeData {
    pub vector: [f32; 128], 
    pub neighbors: [usize; MAX_DEGREE], 
}

impl NodeData {
    pub fn new_empty() -> Self {
        NodeData {
            vector: [0.0; 128], 
            neighbors: [usize::MAX; MAX_DEGREE], 
        }
    }
}

// 确保完美哈希的互质性
fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// 无分支条件交换 (对齐伪代码 Line 20-21: ConditionalSwap)
#[inline(always)]
fn conditional_swap(arr: &mut [usize], i: usize, j: usize, condition: bool) {
    if i != j { 
        let mask = (0usize).wrapping_sub(condition as usize);
        let temp = (arr[i] ^ arr[j]) & mask;
        arr[i] ^= temp;
        arr[j] ^= temp;
    }
}

// ==========================================
// 核心结构：统一内存池 (UnifiedPool)
// ==========================================

pub struct UnifiedPool {
    pub hub_map: HashMap<usize, NodeData>,       // L3 Cache 中的 Hub 节点池 (P_h)
    pub flat_pool: Vec<NodeData>,                // DRAM 中的普通节点池 (P_n)
    pub total_normal_nodes: usize,               // 节点总数 |D|
    pub size_m: usize,                           // 动态物理容量总量 M = sum(A2[i])
    pub node_nth_table: Vec<u32>,                // 当前 Epoch 中各节点已使用的副本数
    pub hub_id_list: Vec<usize>,                 // Hub 节点 ID 列表
    pub param_a: usize,                          // LCG 参数 a
    pub param_b: usize,                          // LCG 参数 b
    
    pub replica_limits: Vec<u32>,                // 每个节点分配的动态副本数 A2[i]
    pub replica_prefix_sum: Vec<usize>,          // 前缀和，用于完美哈希的基址偏移
    pub epoch_access_counts: Vec<u32>,           // 记录每个节点在当前 Epoch 的真实访问量
}

impl UnifiedPool {

    /// 系统冷启动初始化 (不再需要预分配，起手为空)
    pub fn new(total_normal_nodes: usize) -> Self {
        UnifiedPool {
            hub_map: HashMap::new(),
            flat_pool: Vec::new(),  // 初始化为空，严格按照伪代码在运行中动态分配
            total_normal_nodes,
            size_m: 0,
            node_nth_table: vec![0; total_normal_nodes], 
            hub_id_list: Vec::new(),
            param_a: 1, 
            param_b: 0,
            replica_limits: vec![0; total_normal_nodes],
            replica_prefix_sum: vec![0; total_normal_nodes],
            epoch_access_counts: vec![0; total_normal_nodes],
        }
    }

    /// 对齐伪代码 Line 16: f(seed, id, c)
    #[inline(always)]
    fn perfect_hash(&self, node_id: usize, nth: usize) -> usize {
        let x_prime = self.replica_prefix_sum[node_id] + nth;
        (self.param_a.wrapping_mul(x_prime).wrapping_add(self.param_b)) % self.size_m
    }

    /// 导出当前 Epoch 的真实频率分布并清零
    pub fn export_and_reset_frequencies(&mut self, y_queries_executed: usize) -> Vec<f64> {
        let mut frequencies = Vec::with_capacity(self.total_normal_nodes);
        for i in 0..self.total_normal_nodes {
            let freq = self.epoch_access_counts[i] as f64 / y_queries_executed as f64;
            frequencies.push(freq);
        }
        self.epoch_access_counts.fill(0); 
        frequencies
    }

    /// =========================================================================
    /// 核心算法 2：Oblivious Single-Use Index 重建 (严格对齐版)
    /// =========================================================================
    pub fn oblivious_reconstruct(
        &mut self, 
        database: &[NodeData], 
        hub_nodes: &[usize], 
        y_queries: usize,              // 伪代码 Line 1 中的参数 y
        visit_frequencies: &[f64],     // 伪代码 Line 10 中的 F[i]
        new_batch_number: usize        // 伪代码 Line 1 中的 seed
    ) {
        let algo_start_time = std::time::Instant::now();
        
        // ---------------------------------------------------------
        // 伪代码 Lines 2-6: 构建 Hub Pool P_h
        // ---------------------------------------------------------
        let mut in_h = vec![0usize; self.total_normal_nodes];
        for &hid in hub_nodes { in_h[hid] = 1; }

        let mut p_h0 = vec![NodeData::new_empty(); hub_nodes.len()];
        let mut j = 0;
        
        for i in 0..self.total_normal_nodes {
            let safe_j = if j < p_h0.len() { j } else { 0 };
            p_h0[safe_j] = database[i]; 
            j += in_h[i]; // 伪代码 Line 6: ObliAppend           
        }

        self.hub_map.clear();
        self.hub_id_list.clear();
        for (idx, &hid) in hub_nodes.iter().enumerate() {
            self.hub_map.insert(hid, p_h0[idx]);
            self.hub_id_list.push(hid);
        }

        // ---------------------------------------------------------
        // 伪代码 Lines 7-10: 根据 Chernoff Bound 动态分配副本
        // ---------------------------------------------------------
        let c_const = 60.0 * std::f64::consts::LN_2; // 常数 C = 60 * ln(2)
        let mut current_prefix = 0;

        for i in 0..self.total_normal_nodes {
            let r_i = if in_h[i] == 1 { 0 } else {
                // 伪代码 Line 10: 解析解计算 A2[i] = ⌈(1 + \gamma_i) y F[i]⌉
                let mu = y_queries as f64 * visit_frequencies[i];
                let replicas_f64 = mu + (c_const + (c_const * c_const + 8.0 * c_const * mu).sqrt()) / 2.0;
                replicas_f64.ceil() as u32
            };
            
            self.replica_limits[i] = r_i;
            self.replica_prefix_sum[i] = current_prefix;
            current_prefix += r_i as usize;
        }
        
        self.size_m = current_prefix; 
        
        // 更新 LCG 参数
        let mut hasher = Xxh3::new();
        hasher.update(&new_batch_number.to_le_bytes());
        let hash_val = hasher.digest() as usize;
        let mut a = (hash_val >> 32) % self.size_m;
        let b = (hash_val & 0xFFFFFFFF) % self.size_m;
        if a == 0 { a = 1; }
        while gcd(a, self.size_m) != 1 {
            a = (a + 1) % self.size_m;
            if a == 0 { a = 1; } 
        }
        self.param_a = a;
        self.param_b = b;
        
        // 伪代码 Line 28: initialize cnt[id] = 0 (用 node_nth_table 代表 cnt)
        self.node_nth_table.fill(0); 

        // 伪代码 Line 8: initialize arrays A1 and A2
        let mut a1 = Vec::with_capacity(self.total_normal_nodes);
        let mut a2 = Vec::with_capacity(self.total_normal_nodes);
        for i in 0..self.total_normal_nodes {
            if in_h[i] == 0 {
                a1.push(i);
                a2.push(self.replica_limits[i] as usize);
            }
        }
        
        // ---------------------------------------------------------
        // 伪代码 Lines 11-22: Compute the replica-position array pos
        // ---------------------------------------------------------
        // 伪代码 Line 11: initialize pos of size \sum_i A_2[i]
        let mut pos: Vec<usize> = vec![0; self.size_m];
        let mut active = a1.len(); 
        let mut rng = SmallRng::from_entropy();

        while active > 0 {
            let tail = active - 1;
            let idx = rng.gen_range(0..=tail);
            
            let id = a1[idx];                 
            let remaining_c = a2[idx];        
            
            let nth = self.replica_limits[id] as usize - remaining_c;
            let mapped_idx = self.perfect_hash(id, nth); 
            pos[mapped_idx] = id;                        
            
            a2[idx] -= 1;                                
            let is_full = a2[idx] == 0;                  
            
            conditional_swap(&mut a1, idx, tail, is_full); 
            conditional_swap(&mut a2, idx, tail, is_full); 
            active -= is_full as usize;                    
        }

        // ---------------------------------------------------------
        // 伪代码 Line 23: initialize P_n with |pos| fixed-size slots
        // ---------------------------------------------------------
        println!("    [*] 正在精确申请 {} 个节点的物理内存...", self.size_m);
        let alloc_start = std::time::Instant::now();
        
        // 💡 严格遵循伪代码，在此处精确分配所需的物理内存。
        self.flat_pool = vec![NodeData::new_empty(); self.size_m];
        
        // 💡 单独记录操作系统分配和清零这块内存耗费的时间
        let alloc_duration = alloc_start.elapsed();
        println!("    [*] 操作系统物理内存分配完成，耗时: {:?}", alloc_duration);

        // ---------------------------------------------------------
        // 伪代码 Lines 24-27: segment copy
        // ---------------------------------------------------------
        let dim = database.first().map_or(960, |n| n.vector.len());
        let max_deg = MAX_DEGREE;
        let safe_l3_cache_bytes = 96 * 1024 * 1024; // 80MB
        
        let vec_chunk_size = std::cmp::max(1, safe_l3_cache_bytes / (self.total_normal_nodes * 4));
        let vec_passes = (dim + vec_chunk_size - 1) / vec_chunk_size; 

        for pass in 0..vec_passes {
            let start_dim = pass * vec_chunk_size;
            let end_dim = std::cmp::min(start_dim + vec_chunk_size, dim);
            let actual_chunk_size = end_dim - start_dim;
            
            let mut a3 = vec![0.0f32; self.total_normal_nodes * actual_chunk_size];
            for i in 0..self.total_normal_nodes {
                a3[i * actual_chunk_size .. (i + 1) * actual_chunk_size]
                    .copy_from_slice(&database[i].vector[start_dim..end_dim]);
            }
            
            self.flat_pool.par_iter_mut().zip(pos.par_iter()).for_each(|(pool_node, &src_node_id)| {
                pool_node.vector[start_dim..end_dim]
                    .copy_from_slice(&a3[src_node_id * actual_chunk_size .. (src_node_id + 1) * actual_chunk_size]);
            });
        }

        let neighbor_chunk_size = std::cmp::max(1, safe_l3_cache_bytes / (self.total_normal_nodes * 8));
        let neighbor_passes = (max_deg + neighbor_chunk_size - 1) / neighbor_chunk_size;

        for pass in 0..neighbor_passes {
            let start_dim = pass * neighbor_chunk_size;
            let end_dim = std::cmp::min(start_dim + neighbor_chunk_size, max_deg);
            let actual_chunk_size = end_dim - start_dim;
            
            let mut a3_neighbors = vec![0usize; self.total_normal_nodes * actual_chunk_size];
            for i in 0..self.total_normal_nodes {
                a3_neighbors[i * actual_chunk_size .. (i + 1) * actual_chunk_size]
                    .copy_from_slice(&database[i].neighbors[start_dim..end_dim]);
            }
            
            self.flat_pool.par_iter_mut().zip(pos.par_iter()).for_each(|(pool_node, &src_node_id)| {
                pool_node.neighbors[start_dim..end_dim]
                    .copy_from_slice(&a3_neighbors[src_node_id * actual_chunk_size .. (src_node_id + 1) * actual_chunk_size]);
            });
        }
        
        let total_duration = algo_start_time.elapsed();
        // 💡 算法开销 = 总耗时 - 操作系统物理内存申请开销
        let pure_algo_duration = total_duration.saturating_sub(alloc_duration);
        
        println!("==> Algorithm 2 重建完成！");
        println!("    --> 真实经过时间: {:?}", total_duration);
        println!("    --> 纯算法执行耗时 (已扣除OS分配): {:?}", pure_algo_duration);
    }

    // ==========================================
    // 搜索期获取函数
    // ==========================================

    pub fn get(&mut self, target_id: usize) -> NodeData {
        if target_id < self.total_normal_nodes {
            self.epoch_access_counts[target_id] = self.epoch_access_counts[target_id].saturating_add(1);
        }

        let mut rng = rand::thread_rng();
        let is_hub_mask = self.hub_map.contains_key(&target_id) as usize; 

        let random_normal_id = rng.gen_range(0..self.total_normal_nodes); 
        let random_hub_id = if !self.hub_id_list.is_empty() {
            let random_hub_idx = rng.gen_range(0..self.hub_id_list.len());
            self.hub_id_list[random_hub_idx]
        } else {
            0
        };

        let actual_hub_target = is_hub_mask * target_id + (1 - is_hub_mask) * random_hub_id;
        let actual_hash_target = is_hub_mask * random_normal_id + (1 - is_hub_mask) * target_id;

        let empty_hub_data = NodeData::new_empty();
        let hub_data_candidate = self.hub_map.get(&actual_hub_target).unwrap_or(&empty_hub_data);

        let table_len = self.node_nth_table.len();
        let nth_index = actual_hash_target % table_len;
        let current_nth = self.node_nth_table[nth_index];
        self.node_nth_table[nth_index] = self.node_nth_table[nth_index].saturating_add(1);

        let normal_data_candidate = if current_nth >= self.replica_limits[nth_index] {
            let dummy_index = self.perfect_hash(0, 0); 
            let _dummy_data = self.flat_pool[dummy_index]; 
            NodeData::new_empty()
        } else {
            let index = self.perfect_hash(actual_hash_target, current_nth as usize); 
            self.flat_pool[index]
        };

        let final_choices = [&normal_data_candidate, hub_data_candidate];
        *final_choices[is_hub_mask]
    }

    pub fn get_normal_only(&mut self, target_id: usize) -> NodeData {
        if target_id < self.total_normal_nodes {
            self.epoch_access_counts[target_id] = self.epoch_access_counts[target_id].saturating_add(1);
        }

        let table_len = self.node_nth_table.len();
        let nth_index = target_id % table_len;
        
        let current_nth = self.node_nth_table[nth_index];
        self.node_nth_table[nth_index] = self.node_nth_table[nth_index].saturating_add(1);

        if current_nth >= self.replica_limits[nth_index] {
            let dummy_index = self.perfect_hash(0, 0); 
            let _dummy_data = self.flat_pool[dummy_index]; 
            return NodeData::new_empty();
        }

        let index = self.perfect_hash(target_id, current_nth as usize); 
        self.flat_pool[index]
    }
}


 