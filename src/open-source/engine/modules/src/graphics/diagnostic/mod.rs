/*!
 * Holds debug drawing commands.
 * 
 * Debugging commands do not need to be passed a
 * command buffer. This makes it more convenient to display them,
 * but slow to actually display. When possible, these should be
 * disabled.
 */

pub mod diagnostic_primitive;