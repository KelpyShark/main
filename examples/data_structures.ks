# KelpyShark Example: Data Structures
# Demonstrates lists, dicts, and iteration

### Working with lists ###

fruits = ["apple", "banana", "cherry", "date"]
print "Fruits:"
for fruit in fruits {
    print "  - {$fruit}"
}
print "Total: {$fruits} items"

### Working with dictionaries ###

scores = {
    "Alice": 95,
    "Bob": 87,
    "Charlie": 92
}
print "\nScores:"
print scores

### Building a contact list ###

contacts = [
    {"name": "Alice", "email": "alice@example.com"},
    {"name": "Bob", "email": "bob@example.com"}
]

print "\nContacts:"
for contact in contacts {
    print contact
}

### Simple counter with while ###

total = 0
n = 1
while n <= 10 {
    total = total + n
    n = n + 1
}
print "\nSum of 1 to 10: {$total}"
