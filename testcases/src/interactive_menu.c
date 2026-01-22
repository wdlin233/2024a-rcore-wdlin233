/*
 * Interactive Menu Demo
 * Demonstrates interactive CLI with user input
 * 
 * Shows stdin reading and menu-based interaction on rCore OS
 */

#include "syscall.h"

#define SYS_read 63
#define STDIN 0

extern int write(int fd, const void *buf, int len);
extern void exit(int code);

// Helper function to write strings
void puts(const char *str) {
    int len = 0;
    while (str[len]) len++;
    write(STDOUT, str, len);
}

// Read one character from stdin
int read_char(char *c) {
    return syscall(SYS_read, STDIN, c, 1);
}

void print_menu() {
    puts("\n====================================\n");
    puts("     INTERACTIVE MENU DEMO\n");
    puts("====================================\n");
    puts("1. Display system info\n");
    puts("2. Display ASCII art\n");
    puts("3. Simple counter\n");
    puts("4. Exit\n");
    puts("====================================\n");
    puts("Enter your choice (1-4): ");
}

void display_ascii_art() {
    puts("\n   /\\_/\\  \n");
    puts("  ( o.o ) \n");
    puts("   > ^ <  \n");
    puts("  /|   |\\ \n");
    puts(" (_|   |_)\n");
    puts("\nASCII Cat says: Meow!\n");
}

void display_system_info() {
    puts("\nSystem Information:\n");
    puts("==================\n");
    puts("OS: rCore (RISC-V)\n");
    puts("Architecture: RISC-V 64-bit\n");
    puts("User Mode: Enabled\n");
}

void simple_counter() {
    puts("\nCounting from 0 to 9:\n");
    for (int i = 0; i < 10; i++) {
        char num[2] = {'0' + i, ' '};
        write(STDOUT, num, 2);
    }
    puts("\nDone!\n");
}

int main(int argc, char *argv[]) {
    int running = 1;
    char choice;
    char newline;
    
    puts("\nWelcome to rCore Interactive Demo!\n");
    
    while (running) {
        print_menu();
        
        // Read user choice
        if (read_char(&choice) <= 0) {
            puts("\nInput error, exiting...\n");
            break;
        }
        
        // Consume the newline
        read_char(&newline);
        
        puts("\n");
        
        if (choice == '1') {
            display_system_info();
        } else if (choice == '2') {
            display_ascii_art();
        } else if (choice == '3') {
            simple_counter();
        } else if (choice == '4') {
            puts("Thank you for using rCore Interactive Demo!\n");
            puts("Goodbye!\n");
            running = 0;
        } else {
            puts("Invalid choice! Please enter 1-4.\n");
        }
    }
    
    return 0;
}
