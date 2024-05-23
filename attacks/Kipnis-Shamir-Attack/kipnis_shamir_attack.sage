from sage.all import *
import itertools

# Parameters
q = 3  # Field size (change this to test with different field sizes)
n = 20  # Number of variables
m = 10  # Number of equations
o = 5   # Oil variables
v = n - o  # Vinegar variables
k = 2   # Whipping parameter

def random_uov_map(v, o, q):
    """
    Generates a random UOV (Unbalanced Oil and Vinegar) map.
    
    Parameters:
        v (int): Number of vinegar variables.
        o (int): Number of oil variables.
        q (int): Field size for the finite field GF(q).
    
    Returns:
        list: List of 'o' quadratic polynomials over GF(q).
    """
    R = PolynomialRing(GF(q), v + o, 'x')
    x = R.gens()
    P = []
    for i in range(o):
        fi = R(0)
        # Quadratic part in vinegar variables
        for j in range(v):
            for l in range(j, v):
                coeff = GF(q).random_element()
                fi += coeff * x[j] * x[l]
        # Linear part in all variables
        for j in range(v + o):
            coeff = GF(q).random_element()
            fi += coeff * x[j]
        # Constant term
        coeff = GF(q).random_element()
        fi += coeff
        P.append(fi)
    return P
def whipped_up_map(P, v, o, k, q):
    """
    Creates a whipped-up map P* for the MAYO scheme.
    
    Parameters:
        P (list): List of quadratic polynomials.
        v (int): Number of vinegar variables.
        o (int): Number of oil variables.
        k (int): Whipping parameter.
        q (int): Field size for the finite field GF(q).
    
    Returns:
        list: List of 'o' whipped-up quadratic polynomials.
    """
    R = PolynomialRing(GF(q), k * (v + o), 'x')
    x = R.gens()
    P_star = []
    for i in range(o):
        fi_star = R(0)
        for j in range(k):
            subs_dict = {P[i].parent().gen(l): x[j * (v + o) + l] for l in range(v + o)}
            fi_star += P[i].subs(subs_dict)
        P_star.append(fi_star)
    return P_star

def kipnis_shamir_attack(P_star, n, o, q):
    """
    Attempts to recover the oil space vector using the Kipnis-Shamir attack.
    
    Parameters:
        P_star (list): List of whipped-up quadratic polynomials.
        n (int): Total number of variables.
        o (int): Number of oil variables.
        q (int): Field size for the finite field GF(q).
    
    Returns:
        vector or None: The recovered oil space vector if successful, otherwise None.
    """
    R = PolynomialRing(GF(q), n, 'x')
    x = R.gens()
    attempts = 20000  # Number of attempts
    for attempt in range(attempts):  # Try multiple random vectors
        v = vector(GF(q), [GF(q).random_element() for _ in range(n)])
        if all(p.subs({x[i]: v[i] for i in range(n)}) == 0 for p in P_star):
            print(f"Success on attempt {attempt + 1}")
            return v
        if attempt % 1000 == 0:
            print(f"Attempt {attempt + 1}: No solution found yet.")
    print("Failed to recover the oil space vector after all attempts.")
    return None
def try_variable_combinations(P_star, n, o, q):
    """
    Tries different combinations of variables to find the oil space.
    
    Parameters:
        P_star (list): List of whipped-up quadratic polynomials.
        n (int): Total number of variables.
        o (int): Number of oil variables.
        q (int): Field size for the finite field GF(q).
    
    Returns:
        vector or None: The recovered oil space vector if successful, otherwise None.
    """
    indices = list(range(n))
    for comb in itertools.combinations(indices, o):
        print(f"Trying combination: {comb}")
        P_star_comb = [p.subs({p.parent().gen(i): p.parent().gen(i) if i in comb else 0 for i in range(n)}) for p in P_star]
        oil_space = kipnis_shamir_attack(P_star_comb, n, o, q)
        if oil_space is not None:
            print(f"Recovered oil space vector for combination {comb}: {oil_space}")
            return oil_space
    print("No solution found for any combination.")
    return None

# Generate UOV map
print("Generating UOV map...")
P = random_uov_map(v, o, q)
print("UOV map generated.")

# Display the UOV map for debugging
print("UOV map:")
for i, p in enumerate(P):
    print(f"Equation {i+1}: {p}")

# Whip up the map P* for MAYO
print("Whipping up the map P* for MAYO...")
P_star = whipped_up_map(P, v, o, k, q)
print("Whipped-up map P* generated.")

# Display the whipped-up map for debugging
print("Whipped-up map P*:")
for i, p in enumerate(P_star):
    print(f"Whipped-up Equation {i+1}: {p}")

# Perform Kipnis-Shamir attack with variable combinations
print("Performing Kipnis-Shamir attack with variable combinations...")
oil_space = try_variable_combinations(P_star, k * (v + o), o, q)

if oil_space is not None:
    print("Recovered oil space vector:", oil_space)
else:
    print("No solution found.") 
