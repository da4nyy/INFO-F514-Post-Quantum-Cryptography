from sage.all import *

# Parameters
q = 3  # Field size
n = 10  # Number of variables
m = 10  # Number of equations (adjusted to equal the number of variables)
o = 3   # Oil variables
v = n - o  # Vinegar variables
k = 2   # Whipping parameter

def random_uov_map(v, o, m, q):
    """
    Generates a random UOV (Unbalanced Oil and Vinegar) map.
    
    Parameters:
        v (int): Number of vinegar variables.
        o (int): Number of oil variables.
        m (int): Number of equations.
        q (int): Field size for the finite field GF(q).
    
    Returns:
        list: List of 'm' quadratic polynomials over GF(q).
    """
    R = PolynomialRing(GF(q), v + o, 'x')
    x = R.gens()
    P = []
    for i in range(m):
        fi = R(0)
        # Quadratic part in vinegar variables and mixed vinegar-oil terms
        for j in range(v):
            for l in range(j, v):
                coeff = GF(q).random_element()
                fi += coeff * x[j] * x[l]
        for j in range(v):
            for l in range(v, v + o):
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
        list: List of whipped-up quadratic polynomials.
    """
    R = PolynomialRing(GF(q), k * (v + o), 'x')
    x = R.gens()
    P_star = []
    for i in range(len(P)):
        fi_star = R(0)
        for j in range(k):
            subs_dict = {P[i].parent().gen(l): x[j * (v + o) + l] for l in range(v + o)}
            fi_star += P[i].subs(subs_dict)
        P_star.append(fi_star)
    return P_star
def direct_preimage_attack(P_star, target, n, q):
    """
    Attempts to find a preimage for a given target output using Gröbner bases.
    
    Parameters:
        P_star (list): List of whipped-up quadratic polynomials.
        target (list): Target output vector.
        n (int): Total number of variables.
        q (int): Field size for the finite field GF(q).
    
    Returns:
        vector or None: The preimage vector if successful, otherwise None.
    """
    R = PolynomialRing(GF(q), n, 'x')
    x = R.gens()
    
    # Create the system of equations P_star(x) = target
    equations = [P_star[i] - target[i] for i in range(len(P_star))]
    
    # Compute the Gröbner basis
    I = ideal(equations)
    G = I.groebner_basis()
    
    if I.dimension() == 0:
        # Solve the system using the Gröbner basis
        sols = I.variety()
        
        if sols:
            print("Found solution:", sols)
            return sols[0]
        else:
            print("No solution found.")
            return None
    else:
        print("The system does not have a unique solution. Ideal dimension:", I.dimension())
        return None
# Generate UOV map
P = random_uov_map(v, o, m, q)

# Display the UOV map for debugging
print("UOV map:")
for i, p in enumerate(P):
    print(f"Equation {i+1}: {p}")

# Whip up the map P* for MAYO
P_star = whipped_up_map(P, v, o, k, q)

# Display the whipped-up map for debugging
print("Whipped-up map P*:")
for i, p in enumerate(P_star):
    print(f"Whipped-up Equation {i+1}: {p}")

# Define a target output vector
target = [GF(q).random_element() for _ in range(len(P_star))]
print(f"Target output: {target}")

# Perform direct preimage attack
preimage = direct_preimage_attack(P_star, target, k * (v + o), q)

if preimage is not None:
    print("Recovered preimage vector:", preimage)
else:
    print("No solution found.")
