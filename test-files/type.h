typedef struct _testStruct {
    int a;
    int b;
} testStruct;

typedef struct _myType8 {
    uint16 sparrow;
    uint16 penguin;
} myType8;

typedef struct _testStruct {
    int a;
    int b;
} testStruct;

typedef uint8 myType7;

typedef struct _testStruct {
    int a;
    int b;
} testStruct;

typedef struct _myType6 {
    uint16 owl;
    sint8 eagle;
    uint8 falcon;
    myType7 hawk;
    myType8 dove;
} myType6;

typedef myType6 myType5 [3];

typedef struct _testStruct {
    int a;
    int b;
} testStruct;

typedef struct _myType4 {
    uint16 goose;
    uint16 swan;
} myType4;

typedef struct _myType3 {
    myType5 pig;
    uint8 chicken;
    uint8 duck;
} myType3;

typedef struct _myType2 {
    uint32 deer;
    uint8 rabbit;
    uint8 squirrel;
    uint8 horse;
    uint8 cow;
    myType3 sheep;
    myType4 goat;

} myType2;

typedef myType2 myType1 [8];

typedef struct _testStruct {
    int a;
    int b;
} testStruct;

typedef struct _myType0 {
    uint8 tiger;
    uint8 wolf;
    myType1 fox;
} myType0;

typedef struct _testStruct {
    int a;
    int b;
} testStruct;

typedef struct _targetStruct {
    uint8 cat;
    boolean elephant;
    myType0 lion;
} targetStruct;