//! Dead lock detection(banker's algorithm)

use alloc::vec;
use alloc::vec::Vec;

struct HashMap {
    buckets: Vec<Option<(usize, usize)>>,
    size: usize,
    capacity: usize,
}

impl HashMap {
    // 创建一个新的 HashMap，初始容量为 capacity
    fn new(capacity: usize) -> Self {
        HashMap {
            buckets: vec![None; capacity],
            size: 0,
            capacity,
        }
    }

    // 简单的哈希函数，可以根据需要改进
    fn hash(key: usize, capacity: usize) -> usize {
        key % capacity
    }

    // 插入键值对
    fn insert(&mut self, key: usize, value: usize) {
        let index = Self::hash(key, self.capacity);
        let bucket = &mut self.buckets[index];

        match bucket {
            Some(pair) => {
                if pair.0 == key {
                    pair.1 = value; // 如果键已存在，更新值
                } else {
                    // 线性探测解决冲突
                    let mut i = index + 1;
                    while i < self.capacity {
                        let next_bucket = &mut self.buckets[i];
                        match next_bucket {
                            Some((k, _)) if *k == key => {
                                next_bucket.as_mut().unwrap().1 = value; // 更新值
                                break;
                            }
                            None => {
                                *next_bucket = Some((key, value)); // 插入新键值对
                                self.size += 1;
                                break;
                            }
                            _ => {}
                        }
                        i = (i + 1) % self.capacity;
                    }
                }
            }
            None => {
                *bucket = Some((key, value)); // 插入新键值对
                self.size += 1;
            }
        }
    }

    // 获取与键关联的值
    fn get(&self, key: usize) -> Option<&usize> {
        let index = Self::hash(key, self.capacity);
        let mut current = index;

        loop {
            let bucket = &self.buckets[current];
            match bucket {
                Some((k, v)) if *k == key => return Some(v),
                None => return None,
                _ => current = (current + 1) % self.capacity,
            }
        }
    }
}

/// Detector
pub struct Detector {
    /// The number of available resources
    pub available: Vec<usize>,
    /// The amount currently allocated to each customer
    pub allocation: Vec<Vec<usize>>,
    /// The remaining needs of each customer
    pub need: Vec<Vec<usize>>,
    /// mutex map
    m_map: HashMap,
    /// sem map
    s_map: HashMap
}


impl Detector {
    /// Create a new detector
    pub fn new() -> Self {
        Self {
            available: Vec::new(),
            allocation: vec![vec![]],
            need: vec![vec![]],
            m_map: HashMap::new(10),
            s_map: HashMap::new(10),
        }
    }
    
    /// ...
    pub fn create_mutex(&mut self, mutex_id: usize) {
        self.available.push(1);
        self.allocation.iter_mut().for_each(|v| v.push(0));
        self.need.iter_mut().for_each(|v| v.push(0));
        self.m_map.insert(mutex_id, self.available.len() - 1);
    }

    /// ...
    pub fn create_sem(&mut self, sem_id: usize, res_count: usize) {
        self.available.push(res_count);
        self.allocation.iter_mut().for_each(|v| v.push(0));
        self.need.iter_mut().for_each(|v| v.push(0));
        self.s_map.insert(sem_id, self.available.len() - 1);
    }
    
    /// ...
    pub fn get_id(&self, if_mutex: bool, id : usize) -> usize {
        *if if_mutex {
            self.m_map.get(id).unwrap()
        } else {
            self.s_map.get(id).unwrap()
        }
    }

    /// Detect dead lock
    pub fn detect(&mut self) -> bool {
        let n = self.available.len();
        let m = self.allocation.len();
        let mut work = self.available.clone();
        let mut finish = vec![false; m];
        while!finish.iter().all(|&x| x) {
            let mut find = false;
            for i in 0..m {
                if finish[i] {
                    continue;
                }
                let mut j = 0;
                while j < n {
                    if self.need[i][j] > work[j] {
                        break;
                    }
                    j += 1;
                }
                if j == n {
                    finish[i] = true;
                    find = true;
                    for j in 0..n {
                        work[j] += self.allocation[i][j];
                    }
                }
            }
            if!find {
                return true;
            }
        }
        false
    }    
}