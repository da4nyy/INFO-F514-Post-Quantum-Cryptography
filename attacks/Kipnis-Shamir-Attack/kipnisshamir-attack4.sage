from sage.all import *
import time

def generate_mayo_system(n, o, v):
    R = PolynomialRing(GF(2), 'x', n)
    variables = R.gens()
    
    polynomials = []
    for _ in range(o):  # Generate o quadratic polynomials
        poly = sum([R.random_element(degree=2) for _ in range(v)])  # Only vinegar variables contribute freely
        polynomials.append(poly)
    
    return polynomials, variables

def kipnis_shamir_attack(polynomials, n, o, v):
    R = PolynomialRing(GF(2), 'x', n)
    variables = R.gens()
    
    # Construct the matrix representation of the system
    M = Matrix(GF(2), o, n * (n + 1) // 2 + n + 1)
    
    for row, poly in enumerate(polynomials):
        for monomial, coeff in poly.dict().items():
            if coeff == 0:
                continue
            monomial_idx = 0
            for idx, power in enumerate(monomial):
                if power > 0:
                    monomial_idx += 2**idx
            if monomial_idx < M.ncols():
                M[row, monomial_idx] = coeff
    
    # Solve the linear system to find the solution
    try:
        solution = M.right_kernel().basis()
        return solution, M
    except ValueError:
        return None, M
def run_multiple_tests(n, o, v, num_runs):
    total_time = 0
    total_success = 0
    all_results = []
    
    for _ in range(num_runs):
        polynomials, _ = generate_mayo_system(n, o, v)
        start_time = time.time()
        solution, matrix = kipnis_shamir_attack(polynomials, n, o, v)
        end_time = time.time()
        time_taken = end_time - start_time
        total_time += time_taken
        
        all_results.append((polynomials, matrix, solution, time_taken))
        
        if solution:
            total_success += 1
    
    avg_time = total_time / num_runs
    success_rate = (total_success / num_runs) * 100
    
    return float(avg_time), float(success_rate), all_results
def vary_parameters(n_values, o_values, v_values, num_runs):
    results = []
    total_successful_runs = 0
    total_runs = 0
    
    for n in n_values:
        for o in o_values:
            v = n - o
            avg_time, success_rate, all_results = run_multiple_tests(n, o, v, num_runs)
            results.append((n, o, v, avg_time, success_rate))
            if success_rate == 100.0:
                print(f"Success for n: {n}, o: {o}, v: {v} -> Average time: {float(avg_time):.4f} seconds")
                polynomials, matrix, solution, time_taken = all_results[0]
                print(f"Run 1 Details:")
                print("Generated MQ System:")
                for poly in polynomials:
                    print(poly)
                print("Matrix:")
                print(matrix)
                print("Solution(s) found:")
                if solution:
                    for sol in solution:
                        print(sol)
                else:
                    print("No solution found.")
                print(f"Time taken: {float(time_taken):.4f} seconds")
                print("="*50)
            total_successful_runs += num_runs if success_rate == 100.0 else 0
            total_runs += num_runs
    
    total_success_rate = (total_successful_runs / total_runs) * 100
    print(f"Total success rate for all values: {float(total_success_rate):.2f}%")
    return results

# Define the range of parameters for the reduced MAYO scheme
n_values = [8, 10, 12]  # Different values for the total number of variables
o_values = [4, 5, 6]  # Different values for the number of oil variables
v_values = [4, 5, 6]  # Corresponding vinegar variables calculated as n - o
num_runs = 10  # Number of runs to average

# Run the tests with varying parameters
results = vary_parameters(n_values, o_values, v_values, num_runs)