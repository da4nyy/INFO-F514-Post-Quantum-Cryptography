from sage.all import *
import time

# Génère le système de polynômes pour la version réduite du schéma MAYO
def generate_mayo_system(n, m, o):
    R = PolynomialRing(GF(2), 'x', n)  # Crée un anneau polynomial avec n variables sur GF(2)
    variables = R.gens()  # Génère les variables
    v = n - o  # Calcule le nombre de variables de vinaigre
    polynomials = []
    for _ in range(m):
        # Génère un polynôme quadratique aléatoire avec des variables de vinaigre
        poly = sum([R.random_element(degree=2) for _ in range(v)])
        polynomials.append(poly)
    return polynomials, variables

# Implémentation de l'attaque Kipnis-Shamir
def kipnis_shamir_attack(polynomials, n, m, o):
    R = PolynomialRing(GF(2), 'x', n)  # Crée un anneau polynomial avec n variables sur GF(2)
    variables = R.gens()  # Génère les variables
    # Construit la représentation matricielle du système
    M = Matrix(GF(2), m, n * (n + 1) // 2 + n + 1)
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
    try:
        # Tente de résoudre le système linéaire pour trouver la solution
        solution = M.right_kernel().basis()
        return solution, M
    except ValueError:
        return None, M

# Exécute plusieurs tests pour évaluer la performance et le taux de réussite
def run_multiple_tests(n, m, o, num_runs):
    total_time = 0
    total_success = 0
    all_results = []
    for _ in range(num_runs):
        polynomials, _ = generate_mayo_system(n, m, o)
        start_time = time.time()
        solution, matrix = kipnis_shamir_attack(polynomials, n, m, o)
        end_time = time.time()
        time_taken = end_time - start_time
        total_time += time_taken
        all_results.append((polynomials, matrix, solution, time_taken))
        if solution:
            total_success += 1
    avg_time = total_time / num_runs
    success_rate = (total_success / num_runs) * 100
    return float(avg_time), float(success_rate), all_results

# Varie les paramètres pour observer la performance et le taux de réussite sous différentes configurations
def vary_parameters(n_values, m_values, o_values, num_runs):
    results = []
    total_successful_runs = 0
    total_runs = 0
    for n in n_values:
        for m in m_values:
            for o in o_values:
                avg_time, success_rate, all_results = run_multiple_tests(n, m, o, num_runs)
                results.append((n, m, o, avg_time, success_rate))
                if success_rate == 100.0:
                    print(f"Success for n: {n}, m: {m}, o: {o} -> Average time: {float(avg_time):.4f} seconds")
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

# Définir la plage de paramètres pour la version réduite du schéma MAYO
n_values = [20, 25, 30, 35]  # Nombre total de variables
m_values = [10, 12, 15, 18]  # Nombre d'équations
o_values = [6, 8, 10, 12]    # Nombre de variables oil
num_runs = 10                # Nombre de tests pour faire la moyenne

# Exécute les tests avec les paramètres variés
results = vary_parameters(n_values, m_values, o_values, num_runs)
