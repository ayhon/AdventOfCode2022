#!/usr/bin/env python
import toml
import subprocess as sp
import sys
from download_statement import get_statement

def main():
    workspace_file = toml.load("Cargo.toml")

    new_day = int(workspace_file["workspace"]["members"][-1].split("_")[-1]) + 1
    new_member = f"day_{new_day}"

    workspace_file["workspace"]["members"].append(new_member)
    with open("Cargo.toml","w") as f:
        toml.dump(workspace_file,f)

    day_statement = get_statement(new_day)
    if day_statement:
        with open(f"{new_member}/statement.html","w") as f:
            f.write(day_statement)
    else:
        print("Got nothing yet", file=sys.stderr)
        exit(1)

    sp.run(f"cargo new --vcs none {new_member}", shell=True)

    print(f"Good luck on day {new_day}!")
    
if __name__ == "__main__":
    main()
