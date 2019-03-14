//! "platform-intrinsics" bindings

extern "C" {
    #[link_name = "llvm.nvvm.shfl.idx.f32"]
    pub fn __nvvm_shfl_idx_f32(var: f32, src_lane: i32, width: i32) -> f32;
    #[link_name = "llvm.nvvm.shfl.up.f32"]
    pub fn __nvvm_shfl_up_f32(var: f32, delta: u32, width: i32) -> f32;
    #[link_name = "llvm.nvvm.shfl.down.f32"]
    pub fn __nvvm_shfl_down_f32(var: f32, delta: u32, width: i32) -> f32;
    #[link_name = "llvm.nvvm.shfl.bfly.f32"]
    pub fn __nvvm_shfl_bfly_f32(var: f32, delta: i32, width: i32) -> f32;

    #[link_name = "llvm.nvvm.shfl.idx.i32"]
    pub fn __nvvm_shfl_idx_i32(var: i32, src_lane: i32, width: i32) -> i32;
    #[link_name = "llvm.nvvm.shfl.up.i32"]
    pub fn __nvvm_shfl_up_i32(var: i32, delta: u32, width: i32) -> i32;
    #[link_name = "llvm.nvvm.shfl.down.i32"]
    pub fn __nvvm_shfl_down_i32(var: i32, delta: u32, width: i32) -> i32;
    #[link_name = "llvm.nvvm.shfl.bfly.i32"]
    pub fn __nvvm_shfl_bfly_i32(var: i32, delta: i32, width: i32) -> i32;
}

extern "platform-intrinsic" {
    /// `blockDim.x`
    pub fn nvptx_block_dim_x() -> i32;

    /// `blockDim.y`
    pub fn nvptx_block_dim_y() -> i32;

    /// `blockDim.z`
    pub fn nvptx_block_dim_z() -> i32;

    /// `blockIdx.x`
    pub fn nvptx_block_idx_x() -> i32;

    /// `blockIdx.x`
    pub fn nvptx_block_idx_y() -> i32;

    /// `blockIdx.x`
    pub fn nvptx_block_idx_z() -> i32;

    /// `gridDim.x`
    pub fn nvptx_grid_dim_x() -> i32;

    /// `gridDim.y`
    pub fn nvptx_grid_dim_y() -> i32;

    /// `gridDim.z`
    pub fn nvptx_grid_dim_z() -> i32;

    /// `__syncthreads`
    pub fn nvptx_syncthreads() -> ();

    /// `threadIdx.x`
    pub fn nvptx_thread_idx_x() -> i32;

    /// `threadIdx.y`
    pub fn nvptx_thread_idx_y() -> i32;

    /// `threadIdx.z`
    pub fn nvptx_thread_idx_z() -> i32;
}
