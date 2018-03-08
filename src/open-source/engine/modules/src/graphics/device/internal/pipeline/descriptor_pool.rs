/*!
 * Defines the DescriptorPool struct.
 */
use gfx_hal as hal;
use gfx_hal::pso;

/**
 * Contains the actual data of descriptors used by
 * this pipeline. Can be reset() to invalidate all
 * descriptor sets generated from it; it has
 * to be destroyed by the device to free
 * the underlying resources it uses, however.
 */
pub type DescriptorPool<B: hal::Backend> = B::DescriptorPool;