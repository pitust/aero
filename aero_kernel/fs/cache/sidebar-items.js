initSidebarItems({"fn":[["clear_dir_cache",""],["clear_inode_cache",""],["dcache",""],["icache",""],["init","This function is responsible for initializing the inode cache."]],"static":[["DIR_CACHE",""],["INODE_CACHE",""]],"struct":[["Cache","Structure representing a cache with a key of `K` and value of `V`. The cache key is used to get the cache from the cache index. This structure basically contains the cache index (protected by a mutex) and a weak self reference to itself."],["CacheArc",""],["CacheIndex",""],["CacheItem","Structure representing a cache item in the cache index. See the documentation of [CacheIndex] and the fields of this struct for more information."],["CacheWeak",""],["CachedINode",""]],"trait":[["CacheDropper",""],["CacheKey",""],["Cacheable",""],["DirCacheImpl",""]],"type":[["DirCache",""],["DirCacheItem",""],["DirCacheKey","The cache key for the directory entry cache used to get the cache item. The cache key is the tuple of the parent’s cache marker (akin [usize]) and the name of the directory entry (akin [String])."],["INodeCache",""],["INodeCacheItem",""],["INodeCacheKey",""],["INodeCacheWeakItem",""]]});