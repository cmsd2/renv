#REnv

REnv is a tool for running commands with a set of environment variables.
It can be used to quickly switch the value of environment variables which is particularly useful for some devops tools.

Environment variable values are stored in .env files at a well known location:

1. Windows: %APPDATA%\renv\envs\
2. Linux/Unix: $HOME/.config/renv/envs/

## Installation

1. install rust from https://www.rust-lang.org/
2. ensure you have installed your platform's matching compiler toolchain (gcc, mingw or msvc)
2. run ```cargo install renv```

## Platforms

REnv supports windows, linux and unix.


## Commands
REnv supports the following commands:

###edit
Start $EDITOR to modify the contents of the named .env file

###exec
Run a command with the environment variables loaded from the named .env file
Add a "--" before any arguments that might contain hyphens and confuse renv's argument parsing.

###install
Copy a given .env file to the location calculated from the given environment name

###list
List installed environment .env files by environment name

###new
Create a blank installed environment .env file and stsart $EDITOR to edit the contents

###remove
Delete the installed environment .env file

###show
Print the contents of the named environment .env file to the console


## Examples

```
cat > prod.env <<EOF
RENV=prod
AWS_ACCESS_KEY_ID=foo
AWS_SECRET_ACCESS_KEY=bar
EOF

renv install prod prod.env

renv exec prod -- aws s3 ls
```

