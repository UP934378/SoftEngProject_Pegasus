#ifndef _DATA_H_
#define _DATA_H_
#include <Arduino.h>

struct Data_obj {
    int cellVoltage;
    int cellBallanceCurr;
    int cellTemp;
    int battCurrent;
    int battStateCharge;
    int battStateChargePerc;
    int bmsTemp;
    int battTemp;
    int invVoltage;
    int invPower;
    int invFreq;
    int invTemp;
    int gridPower;
    int housePower;
};
char * dataString();
#endif