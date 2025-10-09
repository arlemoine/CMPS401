#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#define MAX_LEN 1024
#define MAX_ARGS 64
#define DELIM " \t\n"

extern char** environ;


int get_line(char* line) {
    printf("=> ");
    if (fgets(line, MAX_LEN, stdin) == NULL)
        return 0;
    return strlen(line);
}

int parse_line(char* line, char** args) {
    int i = 0;

    if ((args[i++] = strtok(line, DELIM)) == NULL)
        return 0;    
    while ((args[i++] = strtok(NULL, DELIM)));

    return i-1;
}

void print_environ() {
    char** env = environ;
    while (*env)
        printf("%s\n", *env++);
}

void exec_external_command(char** args) {
    char command[MAX_LEN] = "";

    while (*args) {
        strcat(command, *args++);
        strcat(command, " ");
    }
    
    system(command);
}

void exec_mydir(char** args) {
    char* command = args[0];
    char* directory = args[1];
    char ls[MAX_LEN] = "";

    strcat(ls, "ls -al ");
    if (directory != NULL)
        strcat(ls, directory);
    
    system(ls);
}

void exec_command(char** args) {
    char* command = args[0];

    if (strcmp(command, "myclear") == 0)
        system("clear");
    else if (strcmp(command, "mydir") == 0)
        exec_mydir(args);
    else if (strcmp(command, "myenviron") == 0)
        print_environ();
    else if (strcmp(command, "myquit") == 0)
        exit(0);
    else
        exec_external_command(args);
}

int main(void) {
    char line[MAX_LEN];
    char* args[MAX_ARGS];
    
    while (get_line(line) != 0)
        if (parse_line(line, args) > 0)
            exec_command(args);

    return 0;
}

