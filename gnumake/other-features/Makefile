# WARNING: This Makefile incorrect, just for learning purposes.

# include other Makefiles
include filenames....


# vpath to specify where some set of prerequisites exist
vpath %.h ../headers ../other-directory

some_binary: ../headers blah.h
	touch some_binary

../headers:
	mkdir ../headers

blah.h:
	touch ../headers/blah.h

clean:
	rm -rf ../headers
	rm -f some_binary

# .PHONY to a target will prevent make from confusing the phony target with a file name
# if the file clean is created, make clean will still be run
.PHONY: clean
clean:
    rm -f some_file
    rm -f clean