/*!
 * Defines the SubpassDependency struct.
 */
use gfx_hal::pass;

/**
 * Describes how the current subpass
 * is related to the inputs and outputs of
 * other subpasses and/or the current render pass.
 * 
 * Effectively the dependency is a graph edge;
 * all of the fields are Ranges, with their starts
 * representing one node and their ends another.
 * The ```passes``` and ```stages``` fields
 * describe properties of the nodes themselves,
 * while the ```accesses``` field describes
 * the access permissions the *opposite* node has
 * to the specified node.
 */
pub type SubpassDependency = pass::SubpassDependency;