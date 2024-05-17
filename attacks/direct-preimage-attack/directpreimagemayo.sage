from sage.all import *
import time

# Define the parameters for the reduced MAYO scheme
n = 10  # number of variables
m = 20  # number of equations

# Create a polynomial ring with n variables over GF(2)
R = PolynomialRing(GF(2), 'x', n)
variables = R.gens()

# Generate a random system of multivariate quadratic equations (MQ system)
polynomials = []
for _ in range(m):
    poly = sum([R.random_element(degree=2) for _ in range(n)])  # Generate random quadratic polynomials
    polynomials.append(poly)

# Display the generated polynomials
print("Generated MQ System:")
for poly in polynomials:
    print(poly)

# Measure the time taken to solve the MQ system using Gröbner basis (libsingular algorithm)
start_time = time.time()
I = Ideal(polynomials)
G = I.groebner_basis(algorithm='libsingular')
end_time = time.time()

# Check if a solution exists and print the solutions
solutions = []
for g in G:
    if g.is_zero() or g.degree() == 0:
        solutions.append(g)
    elif g.is_constant() and g != 0:
        print("No solution exists for the given MQ system.")
        break

# Calculate time complexity
time_taken = end_time - start_time
print(f"Time taken to compute Gröbner basis: {time_taken:.4f} seconds")

# Check if solutions were found and print the success rate
if solutions:
    print("Solution(s) found:")
    for solution in solutions:
        print(solution)
    success_rate = 1.0
else:
    print("No solutions found.")
    success_rate = 0.0

print(f"Success rate: {success_rate * 100:.2f}%")
