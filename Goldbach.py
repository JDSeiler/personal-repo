'''
The Weak Goldbach conjecture states that any odd integer greater than 5 can be represented by the sum of 3 prime numbers.
This code can find such a preresentation given a large enough list of prime numbers. 
'''


primes = [2,3,5,7,13,17,19,23,29,31,37,41,43,47,53,59,61,67,71,73,79,83,
          89,97,101,103,107,109,113,127,131,137,139,149,151]
N = int(input("Input an odd integer greater than 5: "))
for prime in primes:
    if prime > (N/3):
        P = primes[(primes.index(prime) - 1)]
        break

sigma = [P, P, P] # This list represents our 3 prime numebrs

def increment(ind, sigma, primes): # This function changes the prime number at position 'ind' to the next highest prime: [5,5,5] -> [7,5,5] provided ind was 0.
    newIndex = primes.index(sigma[ind]) + 1
    newPrime = primes[newIndex]
    sigma.pop(ind)
    sigma.insert(ind, newPrime)

def decrement(ind, sigma, primes): # Does the reverse of the increment function: [5,5,5] -> [3,5,5] provided ind was 0. 
    newIndex = primes.index(sigma[ind]) - 1
    newPrime = primes[newIndex]
    sigma.pop(ind)
    sigma.insert(ind, newPrime)

def main(sigma, N, primes): # This recursive function increments and decrements the primes in sigma until the sum == the odd number. 
    if sum(sigma) == N:
        return sigma
    elif sum(sigma) > N:
        decrement(2, sigma, primes)
        return main(sigma, N, primes)
    elif sum(sigma) < N:
        increment(0, sigma, primes)
        return main(sigma, N, primes)

result = main(sigma, N, primes)
print(result)
