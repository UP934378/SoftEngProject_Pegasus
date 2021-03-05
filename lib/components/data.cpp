#include "data.hpp"

// Data_obj returnData () {
//     Data_obj newData;
//     newData.cellVoltage = 3000;
//     newData.cellBallanceCurr = 1000;
//     newData.cellTemp = 37;
//     newData.battCurrent = 7000;
//     newData.battStateCharge = 200;
//     newData.battStateChargePerc = 74;
//     newData.bmsTemp = 39;
//     newData.battTemp = 43;
//     newData.invVoltage = 240;
//     newData. invPower = 350;
//     newData.invFreq = 50;
//     newData.invTemp = 42;
//     newData.gridPower = 2000;
//     newData.housePower= 2450;
//     return newData;

// }

char * dataString () {
    char * cellData = (char *) calloc(250 * 3, 1);
    int cellNums = 3;
    for (int i = 0; i < cellNums; i++){
        
        char * cellDataTemp = (char *) malloc(250 * 3);
        char * thirdpointer ;
        if (i == cellNums-1){
            snprintf(cellDataTemp, 250 * 3, "{\"id\":%d,\"voltage\":{\"voltage\":%ld,\"unit\":\"mV\"}}", i, random(3000,4000));
        } else {
            snprintf(cellDataTemp, 250 * 3, "{\"id\":%d,\"voltage\":{\"voltage\":%ld,\"unit\":\"mV\"}},", i, random(3000,4000));
        }
        
        int cellDataLen = strlen(cellData);
        int temCellData = strlen(cellDataTemp);
        thirdpointer = (char *) malloc(cellDataLen + temCellData + 1);
        strcpy(thirdpointer, cellData);
        strcpy(thirdpointer+cellDataLen, cellDataTemp);
        thirdpointer[cellDataLen + temCellData] = 0;
        free(cellDataTemp);
        free(cellData);
        cellData = thirdpointer;

        // cellData += cellDataTemp;
        
        //cellData += "{\"id\":" + i + ",\"voltage\":{\"voltage\":" + random(3000, 4000) + ",\"unit\":\"mV\"}},";
    }
    char* dataAsString = (char *) malloc(2500);
    snprintf(dataAsString, 2500, 
        "{"
        " \"cell\": {"
            "\"data\": ["
                "%s"
                    // "\"id\": 0,"
                    // "\"battery_id\": 0,"
                    // "\"voltage\": {"
                    //     "\"voltage\": 3500,"
                    //     "\"unit\": \"mV\""
                    // "},"
                    // "\"ballance_current\": {"
                    //     "\"current\": 1500,"
                    //     "\"unit\": \"mA\""
                    // "},"
                    // "\"cell_temp\": {"
                    //     "\"temp\": 35,"
                    //     "\"unit\": \"C\""
                    // "}"
            "]"
        "},"
        "\"battery\": {"
            "\"data\": {"
                "\"battery_current\": {"
                    "\"current\": 7000,"
                    "\"unit\": \"mA\""
                "},"
                "\"state_charge\": {"
                    "\"charge\": 200,"
                    "\"unit\": \"Wh\""
                "},"
                "\"charge_perc\": {"
                    "\"charge\": 79,"
                    "\"unit\": \"%\""
                "},"
                "\"battery_temp\": {"
                    "\"temp\": 40,"
                    "\"unit\": \"c\""
                "}"
            "}"
        "},"
        "\"BMS_temp\": {"
            "\"temp\": 32,"
            "\"unit\": \"C\""
        "},"
        "\"inverter\": {"
            "\"data\": ["
                "{"
                    "\"id\": 0,"
                    "\"inv_voltage\": {"
                        "\"voltage\": 240,"
                        "\"unit\": \"V\""
                    "},"
                    "\"inv_power\": {"
                        "\"power\": 349,"
                        "\"unit\": \"W\""
                    "},"
                    "\"inv_frequency\": {"
                        "\"frequency\": 50,"
                        "\"unit\": \"Hz\""
                    "}"
                "}"
            "]"
        "},"
        "\"grid_power\": {"
            "\"power\": 3000,"
            "\"unit\": \"W\""
        "},"
        "\"house_power\": {"
            "\"power\": 3000,"
            "\"unit\": \"W\""
        "}"
        "}"
    , cellData);
    return dataAsString;
}