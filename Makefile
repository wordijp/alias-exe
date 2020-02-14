TARGET	= alias.exe

SRCS	= \
	src/do_alias.rs \
	src/do_exec.rs  \
	src/main.rs     \
	src/path.rs

ifeq ($(DEBUG), 1)
	MODE = debug
	RFLAGS =
else
	MODE = release
	RFLAGS = --release
endif

# ------------------------------------------------

all: bin bin/$(TARGET)

bin:
	@mkdir bin

bin/$(TARGET): target\$(MODE)\$(TARGET)
	cp -f $< $@
	
target\$(MODE)\$(TARGET): $(SRCS)
	cargo build $(RFLAGS)
	
clean:
	rm -f $(TARGET)

# ------------------------------------------------

.PHONY: clean
