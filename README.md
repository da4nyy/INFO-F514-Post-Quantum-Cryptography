# INFO-F514-Post-Quantum-Cryptography
# MAYO Signature Scheme Attacks

This project implements and tests cryptographic attacks on a reduced version of the MAYO signature scheme using SageMath. The implemented attacks are the Kipnis-Shamir attack and the direct preimage attack.

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
load("/sage-scripts/kipnisshamir-attack.sage")
```
Running the Direct Preimage Attack
To execute the direct preimage attack script, run the following command inside the Docker container:

```bash
load("/sage-scripts/directpreimageattack.sage)
```
# Script Details
## Kipnis-Shamir Attack
The kipnisshamir-attack.sage script generates a random MQ (multivariate quadratic) system based on the reduced parameters and attempts to solve it using the Kipnis-Shamir attack.

## Direct Preimage Attack
The directpreimageattack.sage script generates a random MQ system and attempts to solve it using Gröbner basis techniques.
