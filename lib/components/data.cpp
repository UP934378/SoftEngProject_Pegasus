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
    char* dataAsString;
    snprintf(dataAsString, 2000, 
        "{"
        " \"cell\": {"
            "\"data\": ["
                "{"
                    "\"id\": 0,"
                    "\"voltage\": {"
                        "\"voltage\": 3500,"
                        "\"unit\": \"mV\""
                    "},"
                    "\"ballance_current\": {"
                        "\"current\": 1500,"
                        "\"unit\": \"mA\""
                    "},"
                    "\"cell_temp\": {"
                        "\"temp\": 35,"
                        "\"unit\": \"C\""
                    "}"
                "}"
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
    );
    return dataAsString;
}