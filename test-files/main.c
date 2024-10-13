#include "type.h"

targetStruct *test1;
testStruct *test2;

int main() {
    test1 = Rte_IRead_RcTApSF_P_20ms_PpVPE_Delta_Info_DeVPE_Delta_Info(); // it has to be
    test2 = 
    Rte_IRead_RcTApSF_P_20ms_PpVCU_01_01ms_DeVCU_01_10ms
    ();


    // under line must be filled like this

    // test1->cat = 0;
    // test1->elephant = 0;
    // test1->.lion.tiger = 0;
    // test1->.lion.wolf = 0;

    // for(int i=0; i<8; i++) {
    //     test1->.lion.fox[i].deer = 0;
    // }
    // for(int i=0; i<8; i++) {
    //     test1->.lion.fox[i].rabbit = 0;
    // }

    // for(int i=0; i<8; i++) {
    //     for(int j = 0; j<3; j++) {
    //         test1->.lion.fox[i].sheep.pig[j].owl = 0;
    //     }
    // }


    // 0 will be substituted with most simillar word in csv later
    
}