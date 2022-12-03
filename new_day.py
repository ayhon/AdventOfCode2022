#!/usr/bin/env python
import toml
import subprocess as sp

def main():
    workspace_file = toml.load("Cargo.toml")

    new_day = int(workspace_file["workspace"]["members"][-1].split("_")[-1]) + 1
    new_member = f"day_{new_day}"

    workspace_file["workspace"]["members"].append(new_member)
    with open("Cargo.toml","w") as f:
        toml.dump(workspace_file,f)
    sp.run(f"cargo new --vcs none {new_member}", shell=True)
    print(f"Good luck on day {new_day}!")
    
if __name__ == "__main__":
    main()
