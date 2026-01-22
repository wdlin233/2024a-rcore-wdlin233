/*
 * Simple ASCII Number Guessing Game
 * Demonstrates interactive gameplay with user input and feedback
 * 
 * Shows that you CAN create simple games on rCore OS!
 */

#include "syscall.h"

#define SYS_read 63
#define SYS_gettimeofday 169
#define STDIN 0

extern int write(int fd, const void *buf, int len);
extern void exit(int code);

// Simple time structure
struct timeval {
    long sec;
    long usec;
};

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

// Simple pseudo-random number generator
unsigned int seed = 42;

int simple_rand() {
    seed = seed * 1103515245 + 12345;
    return (seed / 65536) % 32768;
}

void print_game_header() {
    puts("\n****************************************\n");
    puts("*    NUMBER GUESSING GAME             *\n");
    puts("*    Guess the number between 1-10    *\n");
    puts("****************************************\n\n");
}

void print_attempts(int attempts) {
    puts("\n[Attempts used: ");
    char num[2] = {'0' + attempts, '\0'};
    write(STDOUT, num, 1);
    for (int i = 0; i < attempts; i++) {
        puts("*");
    }
    puts("]\n\n");
}

int main(int argc, char *argv[]) {
    print_game_header();
    
    // Try to get time for better randomness
    struct timeval tv;
    if (syscall(SYS_gettimeofday, &tv, 0) == 0) {
        seed = tv.sec + tv.usec;
    }
    
    // Generate random number between 1 and 10
    int secret = (simple_rand() % 10) + 1;
    int attempts = 0;
    int max_attempts = 5;
    int found = 0;
    
    puts("I'm thinking of a number between 1 and 10.\n");
    puts("You have 5 attempts to guess it!\n\n");
    
    while (attempts < max_attempts && !found) {
        puts("Attempt ");
        char num[2] = {'0' + (attempts + 1), '\0'};
        write(STDOUT, num, 1);
        puts("/5 - Enter your guess (1-10): ");
        
        // Read user input
        char guess_char;
        char newline;
        if (read_char(&guess_char) <= 0) {
            puts("\nInput error!\n");
            break;
        }
        
        // Consume newline
        read_char(&newline);
        
        // Convert char to number
        int guess = guess_char - '0';
        
        // Validate input
        if (guess < 1 || guess > 10 || guess_char < '0' || guess_char > '9') {
            puts("Invalid input! Please enter a number between 1 and 10.\n\n");
            continue;
        }
        
        attempts++;
        
        // Check guess
        if (guess == secret) {
            found = 1;
            puts("\n");
            puts("CONGRATULATIONS!\n");
            puts("You guessed the number ");
            char s[2] = {'0' + secret, '\0'};
            write(STDOUT, s, 1);
            puts(" correctly!\n");
            print_attempts(attempts);
            
            if (attempts == 1) {
                puts("Amazing! First try!\n");
            } else if (attempts <= 3) {
                puts("Great job! You're good at this!\n");
            } else {
                puts("You made it! Well done!\n");
            }
        } else if (guess < secret) {
            puts("Too low! Try a higher number.\n");
            print_attempts(attempts);
        } else {
            puts("Too high! Try a lower number.\n");
            print_attempts(attempts);
        }
    }
    
    if (!found) {
        puts("\nGame Over! You've used all attempts.\n");
        puts("The secret number was: ");
        char s[2] = {'0' + secret, '\0'};
        write(STDOUT, s, 1);
        puts("\nBetter luck next time!\n");
    }
    
    puts("\nThanks for playing!\n");
    puts("****************************************\n");
    
    return 0;
}
