CC = gcc
CFLAGS = -Wall -Wextra -std=c99 -g
TARGET = forensic 

all: $(TARGET)

$(TARGET): src/main.c
	$(CC) $(CFLAGS) -o $(TARGET) src/main.c

clean: 
	rm -f $(TARGET)

test: $(TARGET)
	./$(TARGET) tests/disk.img

.PHONY: all clean test

