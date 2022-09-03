from datetime import datetime as d

while True:
    date = d.now()
    print(date.strftime("\r%H:%M:%S"),end="")

