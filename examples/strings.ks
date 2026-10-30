# KelpyShark Example: String Operations
# Demonstrates string manipulation and interpolation

name = "KelpyShark"
greeting = "Hello, {$name}! Welcome to the ocean."
print greeting

### String concatenation ###
first = "Hello"
second = " World"
combined = first + second + "!"
print combined

### String length ###
message = "The quick brown fox"
print "Message: \"{$message}\""
print "Length: " + str(len(message))

### Type conversions ###
x = 42
as_string = str(x)
print "Number " + as_string + " as string: " + str(type(as_string))

back_to_num = num("3.14")
print "String to number: " + str(back_to_num)

### Multi-line output ###
print "\n--- Recipe ---"
print "Ingredients:"
ingredients = ["flour", "sugar", "eggs", "butter"]
for item in ingredients {
    print "  * {$item}"
}
print "Mix well and bake at 350F for 30 minutes."
