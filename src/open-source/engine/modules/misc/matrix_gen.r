# Note - matrix multiplication is
# "%*%" in R, "*" appears to be
# componentwise multiplication
random_mat = function() {
	matrix(ceiling(abs(rnorm(16, 0, .5)) * 10),4,4)
}

A = random_mat()
B = random_mat()

print("A =")
print(A)
print("B =")
print(B)
print("A*B =")
print(A %*% B)
print("B*A =")
print(B %*% A)