CC = gcc
CFLAGS = -Wall -Wextra -g
SOURCES = main.c chunk.c debug.c memory.c value.c vm.c scanner.c compiler.c object.c
OBJECTS = $(SOURCES:.c=.o)
EXECUTABLE = clox

all: $(EXECUTABLE)

$(EXECUTABLE): $(OBJECTS)
	$(CC) $(CFLAGS) -o $@ $(OBJECTS)

%.o: %.c
	$(CC) $(CFLAGS) -c -o $@ $<

clean:
	rm -f $(EXECUTABLE) $(OBJECTS)
