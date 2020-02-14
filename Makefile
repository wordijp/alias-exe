TARGET	= alias.exe

SRCS	= \
	src/do_alias.rs   \
	src/do_exec.rs    \
	src/lib/alias.rs  \
	src/lib/encode.rs \
	src/lib/mod.rs    \
	src/lib/path.rs   \
	src/main.rs

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
