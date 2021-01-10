obj_files = foo.result bar.o lose.o
src_files = foo.raw bar.c lose.c

all: $(obj_files)

$(filter %.o,$(obj_files)): %.o: %.c
	echo "target: $@ prereq: $<"
	touch $@
$(filter %.result,$(obj_files)): %.result: %.raw
	echo "target: $@ prereq: $<"
	touch $@

%.c %.raw:
	echo "Create $@"
	touch $@

clean:
	rm -f $(src_files)
	rm -f $(obj_files)
