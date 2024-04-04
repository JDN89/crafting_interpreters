CC = gcc
CFLAGS = -Wall -Wextra -g
SOURCES = main.c chunk.c debug.c memory.c value.c vm.c scanner.c compiler.c object.c
EXECUTABLE = clox

all: $(EXECUTABLE)

$(EXECUTABLE): $(SOURCES)
	$(CC) $(CFLAGS) -o $@ $(SOURCES)

clean:
	rm -f $(EXECUTABLE)
