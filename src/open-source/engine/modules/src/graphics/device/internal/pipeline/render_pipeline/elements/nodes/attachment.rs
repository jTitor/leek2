/*!
 * Defines the Attachment struct.
 */
use gfx_hal::pass;

/**
 * Struct that acts as a reference to
 * an image or framebuffer a pipeline
 * can read from or write to.
 * 
 * Attachments don't directly contain
 * their referring image/framebuffer.
 */
pub type Attachment = pass::Attachment;