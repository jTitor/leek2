//The memory allocator.
//Not all data can be handled on-stack;
//at most we get several MB to play in at each given stack frame,
//and almost any graphical/audio data far outweighs this.
//To handle large assets in memory, we must specify our allocation system.
//LeEK used a combination pool and freelist,
//where (relatively) small data was kept in a series of larger pools while
//large data buffers were stored in pages split into a free list.
//If a large buffer was needed, we would search the current page for enough
//free nodes in the list, allocating and linking a new page if needed.