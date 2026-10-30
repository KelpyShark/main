# single line comment

### multi line comment ###
### used for documentation ###

bob = {
    "age": 27,
    "name": "Bob Smith"
}

example_list = ["apple", "banana", "orange"]

def example_function(value, thing) {
    print "You have {$value} {$thing}s!"

    if value >= 25 {
        print "You lost. You have " + value + " of " + thing
    }
}

example_function(bob["age"], "point")

# Demonstrate more features
print "---"
print "Bob's name: " + bob["name"]
print "Fruits: " + example_list[0] + ", " + example_list[1] + ", " + example_list[2]

for fruit in example_list {
    print "I like " + fruit
}

def factorial(n) {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}

print "5! = " + factorial(5)
