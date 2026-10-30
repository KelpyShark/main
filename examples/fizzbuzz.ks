# KelpyShark Example: FizzBuzz
# Classic programming exercise

def fizzbuzz(n) {
    i = 1
    while i <= n {
        if i % 15 == 0 {
            print "FizzBuzz"
        } else {
            if i % 3 == 0 {
                print "Fizz"
            } else {
                if i % 5 == 0 {
                    print "Buzz"
                } else {
                    print i
                }
            }
        }
        i = i + 1
    }
}

print "FizzBuzz up to 20:"
fizzbuzz(20)
