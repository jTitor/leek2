use std::ops::Range;
use leek2::math::{Mat4x4, MatOps};

fn iterate_on(matrix: &mut Mat4x4, seed: u64, start_idx: u16, end_idx: u16) {
	if start_idx >= end_idx {
		return;
	}

	//Should be in range [-1, 1]
	let fill_value = (((seed + start_idx as u64 + end_idx as u64) % 3) as f32) - 1f32;
	for i in start_idx..end_idx {
		*matrix.mut_elem_at(i as usize) = fill_value;
	}

	//recurse here
	let sub_idx1 = start_idx + ((end_idx - start_idx) / 2);
	let mut sub_idx2 = sub_idx1;
	if sub_idx2 == start_idx {
		sub_idx2 += 1;
	}
	
	iterate_on(matrix, seed, start_idx, sub_idx1);
	iterate_on(matrix, seed, sub_idx2, end_idx);
}

pub fn generate_test_matrix(seed: u64) -> Mat4x4 {
	let mut result = Mat4x4::new();
	iterate_on(&mut result, seed, 0, 16);

	result
}

pub fn test_matrix_seed_range() -> Range<u64> {
	//0..43046721u64
	//0..4304u64
	0..400u64
}