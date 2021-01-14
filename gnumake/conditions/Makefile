bar =
foo = $(bar)

all:
ifdef foo
	@echo "foo is defined"
endif
ifdef bar
	@echo "but bar is not"
endif

ifneq (,$(findstring i, $(MAKEFLAGS)))
	@echo "i was passed to MAKEFLAGS"
else
	@echo "i wasn't passed"
endif