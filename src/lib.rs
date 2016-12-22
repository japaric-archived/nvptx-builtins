#![feature(platform_intrinsics)]
#![no_std]

pub mod intrinsics;

pub use intrinsics::nvptx_block_dim_x as block_dim_x;
pub use intrinsics::nvptx_block_dim_y as block_dim_y;
pub use intrinsics::nvptx_block_dim_z as block_dim_z;
pub use intrinsics::nvptx_block_idx_x as block_idx_x;
pub use intrinsics::nvptx_block_idx_y as block_idx_y;
pub use intrinsics::nvptx_block_idx_z as block_idx_z;
pub use intrinsics::nvptx_grid_dim_x as grid_dim_x;
pub use intrinsics::nvptx_grid_dim_y as grid_dim_y;
pub use intrinsics::nvptx_grid_dim_z as grid_dim_z;
pub use intrinsics::nvptx_syncthreads as syncthreads;
pub use intrinsics::nvptx_thread_idx_x as thread_idx_x;
pub use intrinsics::nvptx_thread_idx_y as thread_idx_y;
pub use intrinsics::nvptx_thread_idx_z as thread_idx_z;
