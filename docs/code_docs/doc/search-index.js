var searchIndex = JSON.parse('{\
"pegassas_aggregator":{"doc":"","i":[[0,"discovery","pegassas_aggregator","",null,null],[17,"DOMAIN","pegassas_aggregator::discovery","SSDP service domain",null,null],[17,"TYPE","","SSDP service type",null,null],[17,"PROBE_URN","","SSDP service Uniform Resource Name (URN)",null,null],[17,"ST","","SSDP search target",null,null],[5,"discover_probes","","Finds all the data probes connected to the network in the …",null,[[]]],[0,"worker","pegassas_aggregator","",null,null],[3,"ProbeWorker","pegassas_aggregator::worker","Worker structure that handles connecting to a data probe …",null,null],[12,"url","","",0,null],[12,"usn","","",0,null],[12,"ttl","","",0,null],[12,"request_interval","","",0,null],[12,"should_work","","",0,null],[12,"rt","","",0,null],[11,"new","","Build a new Worker ",0,[[["string",3],["runtime",3],["duration",3],["arc",3]],["probeworker",3]]],[11,"check_should_work","","",0,[[],["bool",15]]],[11,"check_ttl","","",0,[[],["bool",15]]],[11,"update","","Update Worker informations",0,[[["searchresponse",3]]]],[11,"run","","",0,[[]]],[0,"parser","pegassas_aggregator","",null,null],[4,"VoltageUnit","pegassas_aggregator::parser","Voltage metric",null,null],[13,"mV","","",1,null],[13,"V","","",1,null],[4,"CurrentUnit","","Current metric ",null,null],[13,"mA","","",2,null],[4,"ChargeUnit","","Charge metric ",null,null],[13,"Percentage","","",3,null],[13,"Wh","","",3,null],[4,"TemperatureUnit","","Temperature metric",null,null],[13,"C","","",4,null],[4,"PowerUnit","","Power unit metric",null,null],[13,"W","","",5,null],[4,"FrequencyUnit","","Frequency metric",null,null],[13,"Hz","","",6,null],[3,"Voltage","","A voltage reading with a unit",null,null],[12,"voltage","","",7,null],[12,"unit","","",7,null],[3,"Current","","A current reading with a unit",null,null],[12,"current","","",8,null],[12,"unit","","",8,null],[3,"Charge","","A charge reading with a unit",null,null],[12,"charge","","",9,null],[12,"unit","","",9,null],[3,"Temperature","","A temperature reading with a unit",null,null],[12,"temp","","",10,null],[12,"unit","","",10,null],[3,"Power","","A power reading with a unit",null,null],[12,"power","","",11,null],[12,"unit","","",11,null],[3,"Frequency","","A frequency reading with a unit",null,null],[12,"frequency","","",12,null],[12,"unit","","",12,null],[3,"CellData","","Cell probe information structure",null,null],[12,"id","","",13,null],[12,"voltage","","",13,null],[12,"balance_current","","",13,null],[3,"BatteryData","","Battery probe information structure",null,null],[12,"state_charge","","",14,null],[12,"charge_perc","","",14,null],[12,"battery_current","","",14,null],[12,"battery_temp","","",14,null],[3,"SolarData","","Solar Panel information structure",null,null],[12,"id","","",15,null],[12,"sol_inv_voltage","","",15,null],[12,"sol_inv_power","","",15,null],[12,"sol_inv_frequency","","",15,null],[3,"Cell","","Container structure for cell informations",null,null],[12,"data","","",16,null],[3,"Battery","","Container structure for battery informations",null,null],[12,"data","","",17,null],[3,"Solar","","Container structure for Solar panel informations",null,null],[12,"data","","",18,null],[3,"Data","","Top level structure to store deserialized data from probes",null,null],[12,"probe_id","","",19,null],[12,"cell","","",19,null],[12,"battery","","",19,null],[12,"solar","","",19,null],[12,"grid_power","","",19,null],[12,"house_power","","",19,null],[5,"parse_data","","Parse information from data probe and generate point to …",null,[[["data",3]],["points",3]]],[0,"request","pegassas_aggregator","",null,null],[5,"get_data_url","pegassas_aggregator::request","Read data url from ssdp client response",null,[[["searchresponse",3],["runtime",3]],[["option",4],["string",3]]]],[5,"parse_presentation_url","","Extract data url from XML tree",null,[[["element",3]],[["option",4],["string",3]]]],[5,"make_request","","",null,[[["runtime",3],["string",3]],[["string",3],["result",4],["box",3]]]],[3,"ProbeConfig","pegassas_aggregator","",null,null],[12,"frequency","","",20,null],[5,"main","","",null,[[],[["result",4],["box",3]]]],[11,"from","pegassas_aggregator::worker","",0,[[]]],[11,"into","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"vzip","","",0,[[]]],[11,"from","pegassas_aggregator::parser","",1,[[]]],[11,"into","","",1,[[]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"vzip","","",1,[[]]],[11,"from","","",2,[[]]],[11,"into","","",2,[[]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"vzip","","",2,[[]]],[11,"from","","",3,[[]]],[11,"into","","",3,[[]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"vzip","","",3,[[]]],[11,"from","","",4,[[]]],[11,"into","","",4,[[]]],[11,"borrow","","",4,[[]]],[11,"borrow_mut","","",4,[[]]],[11,"try_from","","",4,[[],["result",4]]],[11,"try_into","","",4,[[],["result",4]]],[11,"type_id","","",4,[[],["typeid",3]]],[11,"vzip","","",4,[[]]],[11,"from","","",5,[[]]],[11,"into","","",5,[[]]],[11,"borrow","","",5,[[]]],[11,"borrow_mut","","",5,[[]]],[11,"try_from","","",5,[[],["result",4]]],[11,"try_into","","",5,[[],["result",4]]],[11,"type_id","","",5,[[],["typeid",3]]],[11,"vzip","","",5,[[]]],[11,"from","","",6,[[]]],[11,"into","","",6,[[]]],[11,"borrow","","",6,[[]]],[11,"borrow_mut","","",6,[[]]],[11,"try_from","","",6,[[],["result",4]]],[11,"try_into","","",6,[[],["result",4]]],[11,"type_id","","",6,[[],["typeid",3]]],[11,"vzip","","",6,[[]]],[11,"from","","",7,[[]]],[11,"into","","",7,[[]]],[11,"borrow","","",7,[[]]],[11,"borrow_mut","","",7,[[]]],[11,"try_from","","",7,[[],["result",4]]],[11,"try_into","","",7,[[],["result",4]]],[11,"type_id","","",7,[[],["typeid",3]]],[11,"vzip","","",7,[[]]],[11,"from","","",8,[[]]],[11,"into","","",8,[[]]],[11,"borrow","","",8,[[]]],[11,"borrow_mut","","",8,[[]]],[11,"try_from","","",8,[[],["result",4]]],[11,"try_into","","",8,[[],["result",4]]],[11,"type_id","","",8,[[],["typeid",3]]],[11,"vzip","","",8,[[]]],[11,"from","","",9,[[]]],[11,"into","","",9,[[]]],[11,"borrow","","",9,[[]]],[11,"borrow_mut","","",9,[[]]],[11,"try_from","","",9,[[],["result",4]]],[11,"try_into","","",9,[[],["result",4]]],[11,"type_id","","",9,[[],["typeid",3]]],[11,"vzip","","",9,[[]]],[11,"from","","",10,[[]]],[11,"into","","",10,[[]]],[11,"borrow","","",10,[[]]],[11,"borrow_mut","","",10,[[]]],[11,"try_from","","",10,[[],["result",4]]],[11,"try_into","","",10,[[],["result",4]]],[11,"type_id","","",10,[[],["typeid",3]]],[11,"vzip","","",10,[[]]],[11,"from","","",11,[[]]],[11,"into","","",11,[[]]],[11,"borrow","","",11,[[]]],[11,"borrow_mut","","",11,[[]]],[11,"try_from","","",11,[[],["result",4]]],[11,"try_into","","",11,[[],["result",4]]],[11,"type_id","","",11,[[],["typeid",3]]],[11,"vzip","","",11,[[]]],[11,"from","","",12,[[]]],[11,"into","","",12,[[]]],[11,"borrow","","",12,[[]]],[11,"borrow_mut","","",12,[[]]],[11,"try_from","","",12,[[],["result",4]]],[11,"try_into","","",12,[[],["result",4]]],[11,"type_id","","",12,[[],["typeid",3]]],[11,"vzip","","",12,[[]]],[11,"from","","",13,[[]]],[11,"into","","",13,[[]]],[11,"borrow","","",13,[[]]],[11,"borrow_mut","","",13,[[]]],[11,"try_from","","",13,[[],["result",4]]],[11,"try_into","","",13,[[],["result",4]]],[11,"type_id","","",13,[[],["typeid",3]]],[11,"vzip","","",13,[[]]],[11,"from","","",14,[[]]],[11,"into","","",14,[[]]],[11,"borrow","","",14,[[]]],[11,"borrow_mut","","",14,[[]]],[11,"try_from","","",14,[[],["result",4]]],[11,"try_into","","",14,[[],["result",4]]],[11,"type_id","","",14,[[],["typeid",3]]],[11,"vzip","","",14,[[]]],[11,"from","","",15,[[]]],[11,"into","","",15,[[]]],[11,"borrow","","",15,[[]]],[11,"borrow_mut","","",15,[[]]],[11,"try_from","","",15,[[],["result",4]]],[11,"try_into","","",15,[[],["result",4]]],[11,"type_id","","",15,[[],["typeid",3]]],[11,"vzip","","",15,[[]]],[11,"from","","",16,[[]]],[11,"into","","",16,[[]]],[11,"borrow","","",16,[[]]],[11,"borrow_mut","","",16,[[]]],[11,"try_from","","",16,[[],["result",4]]],[11,"try_into","","",16,[[],["result",4]]],[11,"type_id","","",16,[[],["typeid",3]]],[11,"vzip","","",16,[[]]],[11,"from","","",17,[[]]],[11,"into","","",17,[[]]],[11,"borrow","","",17,[[]]],[11,"borrow_mut","","",17,[[]]],[11,"try_from","","",17,[[],["result",4]]],[11,"try_into","","",17,[[],["result",4]]],[11,"type_id","","",17,[[],["typeid",3]]],[11,"vzip","","",17,[[]]],[11,"from","","",18,[[]]],[11,"into","","",18,[[]]],[11,"borrow","","",18,[[]]],[11,"borrow_mut","","",18,[[]]],[11,"try_from","","",18,[[],["result",4]]],[11,"try_into","","",18,[[],["result",4]]],[11,"type_id","","",18,[[],["typeid",3]]],[11,"vzip","","",18,[[]]],[11,"from","","",19,[[]]],[11,"into","","",19,[[]]],[11,"borrow","","",19,[[]]],[11,"borrow_mut","","",19,[[]]],[11,"try_from","","",19,[[],["result",4]]],[11,"try_into","","",19,[[],["result",4]]],[11,"type_id","","",19,[[],["typeid",3]]],[11,"vzip","","",19,[[]]],[11,"from","pegassas_aggregator","",20,[[]]],[11,"into","","",20,[[]]],[11,"borrow","","",20,[[]]],[11,"borrow_mut","","",20,[[]]],[11,"try_from","","",20,[[],["result",4]]],[11,"try_into","","",20,[[],["result",4]]],[11,"type_id","","",20,[[],["typeid",3]]],[11,"vzip","","",20,[[]]],[11,"fmt","pegassas_aggregator::parser","",1,[[["formatter",3]],["result",6]]],[11,"fmt","","",2,[[["formatter",3]],["result",6]]],[11,"fmt","","",3,[[["formatter",3]],["result",6]]],[11,"fmt","","",4,[[["formatter",3]],["result",6]]],[11,"fmt","","",5,[[["formatter",3]],["result",6]]],[11,"fmt","","",6,[[["formatter",3]],["result",6]]],[11,"fmt","","",7,[[["formatter",3]],["result",6]]],[11,"fmt","","",8,[[["formatter",3]],["result",6]]],[11,"fmt","","",9,[[["formatter",3]],["result",6]]],[11,"fmt","","",10,[[["formatter",3]],["result",6]]],[11,"fmt","","",11,[[["formatter",3]],["result",6]]],[11,"fmt","","",12,[[["formatter",3]],["result",6]]],[11,"fmt","","",13,[[["formatter",3]],["result",6]]],[11,"fmt","","",14,[[["formatter",3]],["result",6]]],[11,"fmt","","",15,[[["formatter",3]],["result",6]]],[11,"fmt","","",16,[[["formatter",3]],["result",6]]],[11,"fmt","","",17,[[["formatter",3]],["result",6]]],[11,"fmt","","",18,[[["formatter",3]],["result",6]]],[11,"fmt","","",19,[[["formatter",3]],["result",6]]],[11,"serialize","","",1,[[],["result",4]]],[11,"serialize","","",2,[[],["result",4]]],[11,"serialize","","",3,[[],["result",4]]],[11,"serialize","","",4,[[],["result",4]]],[11,"serialize","","",5,[[],["result",4]]],[11,"serialize","","",6,[[],["result",4]]],[11,"serialize","","",7,[[],["result",4]]],[11,"serialize","","",8,[[],["result",4]]],[11,"serialize","","",9,[[],["result",4]]],[11,"serialize","","",10,[[],["result",4]]],[11,"serialize","","",11,[[],["result",4]]],[11,"serialize","","",12,[[],["result",4]]],[11,"serialize","","",13,[[],["result",4]]],[11,"serialize","","",14,[[],["result",4]]],[11,"serialize","","",15,[[],["result",4]]],[11,"serialize","","",16,[[],["result",4]]],[11,"serialize","","",17,[[],["result",4]]],[11,"serialize","","",18,[[],["result",4]]],[11,"serialize","","",19,[[],["result",4]]],[11,"deserialize","","",1,[[],["result",4]]],[11,"deserialize","","",2,[[],["result",4]]],[11,"deserialize","","",3,[[],["result",4]]],[11,"deserialize","","",4,[[],["result",4]]],[11,"deserialize","","",5,[[],["result",4]]],[11,"deserialize","","",6,[[],["result",4]]],[11,"deserialize","","",7,[[],["result",4]]],[11,"deserialize","","",8,[[],["result",4]]],[11,"deserialize","","",9,[[],["result",4]]],[11,"deserialize","","",10,[[],["result",4]]],[11,"deserialize","","",11,[[],["result",4]]],[11,"deserialize","","",12,[[],["result",4]]],[11,"deserialize","","",13,[[],["result",4]]],[11,"deserialize","","",14,[[],["result",4]]],[11,"deserialize","","",15,[[],["result",4]]],[11,"deserialize","","",16,[[],["result",4]]],[11,"deserialize","","",17,[[],["result",4]]],[11,"deserialize","","",18,[[],["result",4]]],[11,"deserialize","","",19,[[],["result",4]]]],"p":[[3,"ProbeWorker"],[4,"VoltageUnit"],[4,"CurrentUnit"],[4,"ChargeUnit"],[4,"TemperatureUnit"],[4,"PowerUnit"],[4,"FrequencyUnit"],[3,"Voltage"],[3,"Current"],[3,"Charge"],[3,"Temperature"],[3,"Power"],[3,"Frequency"],[3,"CellData"],[3,"BatteryData"],[3,"SolarData"],[3,"Cell"],[3,"Battery"],[3,"Solar"],[3,"Data"],[3,"ProbeConfig"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);