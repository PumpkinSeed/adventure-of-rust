CC = gcc
CFLAGS = -g
# C++ compilation implicit rule: $(CXX) -c $(CPPFLAGS) (CXXFLAGS) 
# Linker implicit rule: $(CC) ($LDFLAGS) n.o $(LOADLIBES) $(LDLIBS)

# Implicit rule #1: blah is built via the C linker implicit rule
# Implicit rule #2: blah.o is built via the C++ compilation implicit rule, because blah.cpp exists
blah: blah.o

blah.cpp:
	echo "int main() { return 0; }" > blah.cpp

clean:
	rm -f blah*
