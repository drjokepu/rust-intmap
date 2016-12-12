
struct Kv<V> {
    key: u64, 
    value: V
}

pub struct IntMap<V>{
    cache:  Vec<Vec<Kv<V>>>,
    size: u32,
    mod_mask: u64,
    count: usize,
}

impl<V> IntMap<V> {

    pub fn new() -> Self {
        IntMap::with_capacity(4)
    }


    pub fn with_capacity(capacity: usize) -> Self {
        
        let mut map = IntMap { cache: Vec::new(), size: 0, count: 0, mod_mask: 0 };

        map.increase_cache();
        
        while map.lim() < capacity {
            map.increase_cache();
        }

        map
    }


    pub fn insert(&mut self, key: u64, value: V) -> bool {
        let ix = self.calc_index(key);

        {
        let ref mut vals = self.cache[ix];
        for ref kv in vals.iter() {
            if kv.key == key {
                println!("Found duplicate!");
                return false;
            }
        }

        self.count += 1;
        vals.push(Kv { key: key, value: value });
        }
        if (self.count & 4) == 4 {
            self.ensure_load_rate();
        }

        true
    }


    pub fn get(&self, key: u64) -> Option<&V> {
        let ix = self.calc_index(key);

        let ref vals = self.cache[ix];

        if vals.len() > 0 {

            for kv in vals.iter() {
                if kv.key == key {
                    return Some(&kv.value);
                }
            }

            return None;

        } else {
            return None;
        }
    }


    pub fn remove(&mut self, key: u64) -> Option<V> {
        let ix = self.calc_index(key);

        let ref mut vals = self.cache[ix];

        if vals.len() > 0 {

            for i in 0..vals.len() {
                let peek = vals[i].key;

                if peek == key {
                    self.count -= 1;
                    let kv = vals.swap_remove(i);
                    return Some(kv.value);
                }
            }
            
            return None;

        } else {
            return None;
        }
    }


    pub fn contains_key(&self, key: u64) -> bool {
        match self.get(key) {
            Some(_) => true, 
            None    => false
        }
    }


    pub fn clear(&mut self) {
        for i in 0..self.cache.len() {
            self.cache[i].clear();
        }

        self.count = 0;
    }


    #[inline]
    fn hash_u64(seed: u64) -> u64 {
        let a = 11400714819323198549u64;
        let val = a.wrapping_mul(seed);
        val
    }

    #[inline]
    fn calc_index(&self, key: u64) -> usize {
        let hash = Self::hash_u64(key);
        // Faster modulus
        (hash & self.mod_mask) as usize
    }


    #[inline]
    fn lim(&self) -> usize {
        2u64.pow(self.size) as usize
    }


    fn increase_cache(&mut self) {
        self.size += 1;
        let new_lim = self.lim();
        self.mod_mask = (new_lim as u64) - 1;

        let mut vec: Vec<Vec<Kv<V>>> = Vec::new();

        vec.append(&mut self.cache);

        for _ in 0..new_lim {
            self.cache.push(Vec::with_capacity(0));
        }
        
        while vec.len() > 0 {
            let mut values = vec.pop().unwrap();
            while values.len() > 0 {
                if let Some(k) = values.pop() {
                    let ix = self.calc_index(k.key);

                    let ref mut vals = self.cache[ix];
                    vals.push(k);
                }   
            }
        }

        debug_assert!(self.cache.len() == self.lim(), "cache vector the wrong length, lim: {:?} cache: {:?}", self.lim(), self.cache.len());
    }


    fn ensure_load_rate(&mut self) {
        while ((self.count*100) / self.cache.len()) > 70 {
            self.increase_cache();
        }
    }


    pub fn count(&self) -> u64 {
        self.count as u64
    }


    pub fn load(&self) -> u64 {
        let mut count = 0;

        for i in 0..self.cache.len() {
            if self.cache[i].len() > 0 {
                count += 1;
            }
        }

        count
    }


    pub fn load_rate(&self) -> f64 {
        (self.count as f64) / (self.cache.len() as f64) * 100f64
    }


    pub fn capacity(&self) -> usize {
        self.cache.len()
    }


    pub fn assert_count(&self) -> bool {
        let mut count = 0;

        for i in 0..self.cache.len() {
            for _ in self.cache[i].iter() {
                count += 1;
            }
        }

        self.count == count
    }
    // pub fn collisions(&self) -> HashMap<u64, u64> {
    //     let mut map = HashMap::new();

    //     for s in self.cache.iter() {
    //         if s.len() > 1 {
    //             let counter = map.entry(s.len() as u64).or_insert(0);
    //             *counter += s.len() as u64;
    //             // vec.push(s.len() as u64);
    //         }
    //     }

    //     // map.sort();

    //     map
    // }

}
