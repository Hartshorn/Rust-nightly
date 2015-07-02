#include <stdio.h>
#include <string.h>
#include <stdbool.h>
#include <stdarg.h>
#include <assert.h>

#define WIDTH       100
#define HEIGHT      50
#define ARRAY_MAX   WIDTH * HEIGHT


typedef struct Pos {
    int x;
    int y;
} Pos;



//==================BEGIN DEBUG FUNCTIONS==========================

void debug_show_pos(Pos);
void debug_logger(int, ...);

void debug_show_pos(Pos p) {
    printf("Pos: x: %d, y: %d\n", p.x, p.y);
}

void debug_logger(int count_after_string, ...) {
    int i;
    Pos p;
    va_list ap;
    va_start(ap, count_after_string);
    printf("========%s=========\n", va_arg(ap, char*));
    for(i = 0; i < count_after_string; i++) {
        p = va_arg(ap, Pos);
        debug_show_pos(p);
    }
    va_end(ap);
}

//==================END DEBUG FUNCTIONS==============================

void run_tests();

void clear();
void go_to();
void writeat(Pos, char*);
void showcells(Pos*, int);
void wrap(Pos*);
void neighbors(Pos, Pos*);
int  liveneighbors(Pos*, Pos, int);
bool contains(Pos*, Pos, int);
int  survivors(Pos*, Pos*, int);
int  births(Pos*, Pos*, int);
int  remdup(Pos*, int);
Pos *nextgen(Pos*, int);
void initialize(Pos*, int);
void life(Pos*, int);
Pos *get_flat();

    
int main(int argc, char *argv[])
{
    if(argc > 1 && strcmp(argv[1], "-test") == 0) {
        run_tests();
    } else {
        int w = WIDTH / 2, h = HEIGHT / 2;
        // Pos board[3] = { 
        //     {w - 1, h}, {w, h}, {w + 1, h} 
        // };
        
        // ******** *****   ***      ******* ***** 
        // ******** *****   ***      ******* *****
        // ******** *****   ***      ******* *****
        // <1sp><8*><1sp><5*><3sp><3*><6sp><7*><1sp><5*><1sp>
        // 1 + 8 + 1 + 5 + 3 + 3 + 6 + 7 + 1 + 5 + 1 = 41
        // 8 + 5 + 3 + 7 + 5 = 28
        
        Pos board[100] = {
            {.x = w-19, .y = h}, {w-18,h}, {w-17,h}, {w-16,h}, {w-15,h}, {w-14,h}, {w-13,h}, {w-12,h},
            {w-10,h}, {w-9,h}, {w-8,h}, {w-7,h}, {w-6,h},
            {w-2,h}, {w-1,h}, {w,h},
            {w+7,h}, {w+8,h}, {w+9,h}, {w+10,h}, {w+11,h}, {w+12,h}, {w+13,h},
            {w+15,h}, {w+16,h}, {w+17,h}, {w+18,h}, {w+19,h}
        };
        
        
        Pos *hold = nextgen(board, 100);
        
        for(int i = 0; i < 100; i++) {
            hold = nextgen(hold, 100);
        }
        
        showcells(board, 100);
        // life(board, sizeof(board) / sizeof(Pos));
    }
    
    return 0;
}


void life(Pos *board, int length) {
    getchar();
    clear();
    showcells(board, length);
    life(nextgen(board, length), length);
}

void clear() {
    printf("%c[2J", 27);
}

void go_to(int x, int y) {
    printf("\033[%d;%dH", y, x);
}

void writeat(Pos p, char *str) {
    go_to(p.x, p.y);
    printf("%s", str);
}

void showcells(Pos *board, int length) {
    int i;
    for(i = 0; i < length; i++) {
        writeat(board[i], "*");
    }
}

void wrap(Pos *p) {
    
    int x, y;
    
    x = p->x;
    y = p->y;
    
    p->x = ((x - 1) % WIDTH) + 1;
    p->y = ((y - 1) % HEIGHT) + 1;
}

void neighbors(Pos p, Pos *result) {
    int i;
    
    result[0].x = p.x - 1;
    result[0].y = p.y - 1;
    result[1].x = p.x;
    result[1].y = p.y - 1;
    result[2].x = p.x + 1;
    result[2].y = p.y - 1;
    result[3].x = p.x - 1;
    result[3].y = p.y;
    result[4].x = p.x + 1;
    result[4].y = p.y;
    result[5].x = p.x - 1;
    result[5].y = p.y + 1;
    result[6].x = p.x;
    result[6].y = p.y + 1;
    result[7].x = p.x + 1;
    result[7].y = p.y + 1;
    
    for(i = 0; i < 8; i++) {
        wrap(&result[i]);
    }
}

int liveneighbors(Pos *board, Pos p, int length) {
    int alive, i;
    Pos neighbs[8];
    alive = 0;
    
    neighbors(p, neighbs);
    
    for(i = 0; i < sizeof(neighbs) / sizeof(Pos); i++) {
        if (contains(board, neighbs[i], length)) {
            alive += 1;
        }
    }
    return alive;
}

bool contains(Pos *board, Pos p, int length) {
    int i;
    
    for(i = 0; i < length; i++) {
        if (board[i].y == p.y && board[i].x == p.x) {
            return true;
        }
    }
    return false;
}

int survivors(Pos *board, Pos *result, int length) {
    int i, c = 0;
    
    for(i = 0; i < length; i++) {
        if(liveneighbors(board, board[i], length) == 2 || 
           liveneighbors(board, board[i], length) == 3) {
            result[c].x = board[i].x;
            result[c++].y = board[i].y;
        }
    }
    return c;
}

int births(Pos *board, Pos *result, int b_length) {
    int i, j, c, ret;
    Pos neighbs[8];
    c = 0;
    
    for(i = 0; i < b_length; i++) {
        neighbors(board[i], neighbs);
        for(j = 0; j < 8; j++) {
            if(!contains(board, neighbs[j], b_length) && 
               liveneighbors(board, neighbs[j], b_length) == 3)
               result[c++] = neighbs[j];
        }
    }
    ret = remdup(result, c);
    return ret;
}

int remdup(Pos *board, int length) {
    int i, c;
    Pos hold[length];
    c = 0;
    
    hold[c++] = board[0];
    
    for(i = 1; i < length; i++) {
        if(!contains(hold, board[i], sizeof(hold) / sizeof(Pos))) {
            hold[c].x   = board[i].x;
            hold[c++].y = board[i].y;
        }
    }
    board = hold;
    return c;
}

Pos *nextgen(Pos *board, int length) {
    
    int i, r_length, s_count = 0, b_count = 0, r_count = 0;
    Pos sur[length], bir[length];
    
    initialize(sur, length);
    initialize(bir, length);
    
    s_count = survivors(board, sur, length);
    b_count = births(board, bir, length);
    r_length = s_count + b_count;
    
    static Pos result[3];
    initialize(result, r_length);
    
    for(i = 0; i < s_count; i++) {
        result[r_count++] = sur[i];
    }
    for(i = 0; i < b_count; i++) {
        result[r_count++] = bir[i];
    }
    // showcells(result, r_count);
    board = result;
    return board;
}

void initialize(Pos *board, int length) {
    int i;
    
    for(i = 0; i < length; i++) {
        Pos p;
        p.x = 0;
        p.y = 0;
        board[i] = p;
    }
}



void run_tests() {
    debug_logger(0, "Running Tests: GLIDER . . .");
    
    int t_length;
    
    Pos t_board_glider[3] = { {3, 2}, {4, 2}, {5, 2} };
    
    t_length = sizeof(t_board_glider) / sizeof(Pos);
    
    Pos t_sur[t_length], t_bir[t_length];
    
    assert(survivors(t_board_glider, t_sur, t_length) == 1);
    assert(births(t_board_glider, t_bir, t_length) == 2);
    
    Pos t_pos_valid = {3,2};
    Pos t_pos_invalid = {100, 100};
    
    assert(contains(t_board_glider, t_pos_valid, t_length) == true);
    assert(contains(t_board_glider, t_pos_invalid, t_length) == false);
    
    Pos t_board_glider_result[3] = { {14, 12}, {14, 11}, {14, 13} };
    Pos *t_board_glider_result_pointer = nextgen(t_board_glider, t_length);
    
    assert(t_board_glider_result_pointer == t_board_glider_result);
    
    debug_logger(0, "All Tests Completed - GLIDER - Success!");
}