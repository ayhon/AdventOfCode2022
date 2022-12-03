#!/usr/bin/env python
import requests
import sys

url = "https://adventofcode.com/2022/day/"

def get_statement(day):
    response = requests.get(url+str(day))
    if response.status_code == 200:
        html = response.text
        statement = html[html.find("<main>") + len("<main>") : html.find("</article>") + len("</article>")]
        return statement
    else:
        print(f"Error: Received status code {response.status_code}",file=sys.stderr)

def main():
    day = 1 if len(sys.argv) <= 1 else int(sys.argv[1])
    print(get_statement(day))

if __name__ == "__main__":
    main()
