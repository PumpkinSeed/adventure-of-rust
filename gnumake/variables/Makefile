one = hello
one ?= will not be set
two ?= will be set
# ?= only set variables if they have not yet been set

one += two
# += append

override option_one = did_override
# override command line arguments

# list of commands, like function
define two
export blah=set
echo $$blah
endef

# target-specific variables
all: one = cool

all:
	echo one is defined: $(one)
