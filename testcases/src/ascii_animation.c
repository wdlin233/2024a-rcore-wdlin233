/*
 * ASCII Animation Demo - Similar to "Bad Apple" style animations
 * Demonstrates console output with timing control for frame-based animation
 * 
 * This shows that you CAN create ASCII animations on rCore OS!
 */

#include "syscall.h"

#define SYS_read 63
#define SYS_sleep 101
#define STDIN 0

extern int write(int fd, const void *buf, int len);
extern void exit(int code);

// Helper function to write strings
void puts(const char *str) {
    int len = 0;
    while (str[len]) len++;
    write(STDOUT, str, len);
}

// Simple sleep using syscall
void sleep_ms(int ms) {
    syscall(SYS_sleep, ms);
}

// Simple frames for a rotating line animation
const char* frame1 = "    |    \n    |    \n    |    \n";
const char* frame2 = "    /    \n   /     \n  /      \n";
const char* frame3 = "  _____  \n         \n         \n";
const char* frame4 = "      \\  \n     \\   \n    \\    \n";
const char* frame5 = "    |    \n    |    \n    |    \n";

int main(int argc, char *argv[]) {
    puts("=================================\n");
    puts("   ASCII ANIMATION DEMO\n");
    puts("=================================\n\n");
    
    int iterations = 0;
    
    // Run animation for a limited time (20 frames)
    while (iterations < 20) {
        const char* frame;
        int frame_num = iterations % 5;
        
        if (frame_num == 0) frame = frame1;
        else if (frame_num == 1) frame = frame2;
        else if (frame_num == 2) frame = frame3;
        else if (frame_num == 3) frame = frame4;
        else frame = frame5;
        
        // Display current frame
        puts("Frame ");
        // Simple number output (digits only for 0-9)
        if (iterations < 10) {
            char num[2] = {'0' + iterations, '\0'};
            write(STDOUT, num, 1);
        } else {
            char num[3] = {'1', '0' + (iterations - 10), '\0'};
            write(STDOUT, num, 2);
        }
        puts(":\n");
        puts(frame);
        puts("---\n\n");
        
        iterations++;
        
        // Frame delay - 200ms per frame
        sleep_ms(200);
    }
    
    puts("\nAnimation complete!\n");
    
    return 0;
}
