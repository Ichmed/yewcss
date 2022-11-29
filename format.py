import sys

l = []

for element in sys.argv[1].split("|"):
    element = element.strip()
    if "-" in element:
        l.append(element.replace("-", " ").title().replace(" ", "") + " " + f'"{element}"')
    else:
        l.append(element.capitalize())

print(" | ".join(l))
    