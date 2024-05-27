# INFO-F514-Post-Quantum-Cryptography
## Repository Structure
**attacks**/: Contains scripts to implement and test cryptographic attacks on a reduced version of the MAYO signature scheme using SageMath.
**implementation**/: Contains the implementation of the MAYO signature scheme using Rust.
# MAYO Signature Scheme Attacks

The files in the attacks/ serves to implement and test cryptographic attacks on a reduced version of the MAYO signature scheme using SageMath. The implemented attacks are the Kipnis-Shamir attack and the direct preimage attack.

## Requirements

- Docker
- SageMath Docker image

## Setup

To set up the environment, ensure Docker is installed on your machine. Then, pull the latest SageMath Docker image:

```bash
docker pull sagemath/sage:latest
```
# Running the SageMath Scripts
Clone the repository or copy the scripts to a local directory.
Create a directory for the SageMath scripts if not already done:
```bash
mkdir -p sage-scripts
```
Copy the SageMath scripts (kipnisshamir-attack.sage and directpreimageattack.sage) into the sage-scripts directory.

Run the SageMath Docker container with the scripts directory mounted:

```bash
docker run -it -v  $(pwd)/sage-scripts:/sage-scripts sagemath/sagemath:latest
```
Running the Kipnis-Shamir Attack
To execute the Kipnis-Shamir attack script, run the following command inside the Docker container:

```bash
load("/sage-scripts/kipnis_shamir_attack.sage")
```
Running the Direct Preimage Attack
To execute the direct preimage attack script, run the following command inside the Docker container:

```bash
load("/sage-scripts/directpreimagemayo.sage)
```
# Script Details
## Kipnis-Shamir Attack
The kipnisshamir-attack.sage script generates a random MQ (multivariate quadratic) system based on the reduced parameters and attempts to solve it using the Kipnis-Shamir attack.

## Direct Preimage Attack
The directpreimageattack.sage script generates a random MQ system and attempts to solve it using Gröbner basis techniques.

# Implementation of the MAYO alogrithm in Rust

The files in the /implementation/ folder serves to implement the MAYO signature scheme using Rust.  

The implemenation is based on the official mayo specs available here : https://pqmayo.org/assets/specs/mayo.pdf  
We also used these two implementations :
- https://github.com/PQCMayo/MAYO-C
- https://github.com/PQCMayo/MAYO-sage


## Requirements   
- Rust  (https://www.rust-lang.org/fr/tools/install)

## Running 

First, you need to build the project :
```
cargo build
```

Then to launch the implmentation :
```
cargo run
```

