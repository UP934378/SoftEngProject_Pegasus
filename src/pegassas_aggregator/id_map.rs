use std::collections::HashMap;
use postgres::Client;

pub struct CellIDMap {
    map : HashMap<(i64, i16), i16>,
    db_client : Client
}

impl CellIDMap {
    pub fn new() -> Result<CellIDMap, postgres::Error> {
        match Client::connect("host=localhost", postgres::NoTls){
            Ok(client) => {
                let cell_id_map = CellIDMap {
                    map : HashMap::new(),
                    db_client : client
                };
                match cell_id_map.db_client.query("SELECT probe_id, local_cell_id, global_cell_id FROM cell_id_map", &[]){
                    Ok(rows) =>{
                        for row in rows {
                            let probe_id : i64 = row.get(0);
                            let local_cell_id : i16 = row.get(1);
                            let global_cell_id : i16 = row.get(2);
                            cell_id_map.map.insert((probe_id, local_cell_id), global_cell_id);
                        }
                    },
                    Err(e) => return Err(e)
                }

                Ok(cell_id_map)
            },
            Err(e) => Err(e)
        }
    }

    pub fn get_global_cell_id(&mut self, probe_id : i64, local_cell_id : i16){
        match self.map.get(&(probe_id, local_cell_id)){
            Some(global_cell_id) => global_cell_id,
            None => {
                // Add new mapping for probe_id and local_cell_id to db and return new value
            }
        }
    }
 
}