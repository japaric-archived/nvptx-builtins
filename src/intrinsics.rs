//! "platform-intrinsics" bindings

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
