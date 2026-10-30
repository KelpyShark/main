# KelpyShark Example: Functions & Recursion
# Demonstrates function definitions, recursion, and higher-order patterns

### Factorial (recursive) ###
def factorial(n) {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}

print "Factorials:"
i = 1
while i <= 10 {
    result = factorial(i)
    print "  {$i}! = {$result}"
    i = i + 1
}

### Fibonacci (recursive) ###
def fibonacci(n) {
    if n <= 0 {
        return 0
    }
    if n == 1 {
        return 1
    }
    return fibonacci(n - 1) + fibonacci(n - 2)
}

print "\nFibonacci sequence (first 15):"
j = 0
while j < 15 {
    fib = fibonacci(j)
    print "  fib({$j}) = {$fib}"
    j = j + 1
}

### Power function ###
def power(base, exp) {
    result = 1
    count = 0
    while count < exp {
        result = result * base
        count = count + 1
    }
    return result
}

print "\nPowers of 2:"
k = 0
while k <= 10 {
    p = power(2, k)
    print "  2^{$k} = {$p}"
    k = k + 1
}

### GCD (Euclidean algorithm) ###
def gcd(a, b) {
    while b != 0 {
        temp = b
        b = a % b
        a = temp
    }
    return a
}

print "\nGCD examples:"
print "  gcd(48, 18) = " + str(gcd(48, 18))
print "  gcd(100, 75) = " + str(gcd(100, 75))
