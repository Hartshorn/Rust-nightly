#include <stdio.h>

typedef struct Bee {
    int speed;
    int friendly;
    int stinger;
} Bee;

void initialize(Bee *b);
void show(Bee b, char *name);
void give_stinger(Bee *b);
void make_mad(Bee *b);
void slow_down(Bee *b, int n);
Bee copy(Bee b);

int main (int args, char *argv[])
{
    Bee b, b2;
    Bee *link_to_b;
    
    initialize(&b);
    b2 = copy(b);
    link_to_b = &b;
    
    printf("Initialize:\n------------\n");
    show(b, "b");
    show(b2, "b2");
    show(*link_to_b, "link_to_b");
    
    printf("----------\nChange b:\n----------\n");
    give_stinger(&b);
    make_mad(&b);
    slow_down(&b, 10);
    
    show(b, "b");
    show(b2, "b2");
    show(*link_to_b, "link_to_b");
    
    printf("----------\nChange b2:\n----------\n");
    give_stinger(&b2);
    make_mad(&b2);
    slow_down(&b2, 100);
    
    show(b, "b");
    show(b2, "b2");
    show(*link_to_b, "link_to_b");
}

void initialize(Bee *b) {
    b->speed = 100;
    b->friendly = 1;
    b->stinger = 0;
    
}

void show(Bee b, char *name) {
    printf("Name: %s\nSpeed: %d\nFriendly: %s\nHas a Stinger: %s\n\n",
             name,
             b.speed,
            (b.friendly == 1) ? "Yes" : "No",
            (b.stinger  == 1) ? "Yes" : "No");
}

void give_stinger(Bee *b) {
    b->stinger = 1;
}

void make_mad(Bee *b) {
    b->friendly = 0;
}

void slow_down(Bee *b, int n) {
    b->speed -= n;
}

Bee copy(Bee b) {
    struct Bee new_b;
    
    new_b.speed    = b.speed;
    new_b.friendly = b.friendly;
    new_b.stinger  = b.stinger;
    
    return new_b;
}