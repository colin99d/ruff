# These SHOULD change
a = "hello"

if a == "foo":
    return "bar"
elif a == "bar":
    return "baz"
elif a == "boo":
    return "ooh"
else:
    return 42

if a == 1:
    return (1, 2, 3)
elif a == 2:
    return (4, 5, 6)
elif a == 3:
    return (7, 8, 9)
else:
    return (10, 11, 12)

if a == 1:
    return (1, 2, 3)
elif a == 2:
    return (4, 5, 6)
elif a == 3:
    return (7, 8, 9)

if a == "hello 'sir'":
    return (1, 2, 3)
elif a == 'goodbye "mam"':
    return (4, 5, 6)
elif a == """Fairwell 'mister'""":
    return (7, 8, 9)
else:
    return (10, 11, 12)

if a == b"one":
    return 1
elif a == b"two":
    return 2
elif a == b"three":
    return 3

# These Should NOT change
if a == "foo":
    return "bar"
elif a == "bar":
    return baz()
elif a == "boo":
    return "ooh"
else:
    return 42


if a == b"one":
    return 1
elif b == b"two":
    return 2
elif a == b"three":
    return 3
