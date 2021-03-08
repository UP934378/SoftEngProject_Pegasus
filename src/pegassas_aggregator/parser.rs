use serde::{Deserialize, Serialize};
use std::fmt::Debug;

// Unit enums

#[derive(Serialize, Deserialize, Debug)]
pub enum VoltageUnit {
    mV,
    V
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CurrentUnit {
    mA
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChargeUnit {
    Percentage,
    Wh
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TemperatureUnit {
    C
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PowerUnit {
    W
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FrequencyUnit {
    Hz
}

// Reading structs

/// A voltage reading with a unit
#[derive(Serialize, Deserialize, Debug)]
pub struct Voltage {
    voltage : u16,
    unit : VoltageUnit
}

/// A current reading with a unit
#[derive(Serialize, Deserialize, Debug)]
pub struct Current {
    current : u16,
    unit : CurrentUnit
}

/// A charge reading with a unit
#[derive(Serialize, Deserialize, Debug)]
pub struct Charge {
    charge : u16,
    unit : ChargeUnit
}

/// A temperature reading with a unit
#[derive(Serialize, Deserialize, Debug)]
pub struct Temperature {
    temp : i16,
    unit : TemperatureUnit
}

/// A power reading with a unit
#[derive(Serialize, Deserialize, Debug)]
pub struct Power {
    power : i16,
    unit : PowerUnit
}

/// A frequency reading with a unit
#[derive(Serialize, Deserialize, Debug)]
pub struct Frequency {
    frequency : u16,
    unit : FrequencyUnit
}

// Probe reading structs


#[derive(Serialize, Deserialize, Debug)]
pub struct CellData {
    id : u16,
    battery_id : u16,
    voltage : Voltage,
    balance_current : Current
}


#[derive(Serialize, Deserialize, Debug)]
pub struct BatteryData {
    id : u16,
    state_charge : Charge,
    charge_perc : Charge,
    balance_current : Current,
    battery_temp : Temperature
}


#[derive(Serialize, Deserialize, Debug)]
pub struct SolarData {
    id: String,
    inv_voltage : Voltage,
    inv_power : Power,
    inv_frequency : Frequency
}







#[derive(Serialize, Deserialize, Debug)]
pub struct Cell {
    data : std::vec::Vec<CellData>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Battery {
    data : BatteryData
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Solar {
    data : std::vec::Vec<SolarData>
}






/// Top level structure to store deserialised data from probes
#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    probe_id : i64,
    cell : Option<Cell>,
    battery : Option<Battery>,
    solar : Option<Solar>,
    grid_power : Option<Power>,
    house_power : Option<Power>
}

pub fn parse_data(data: Data) -> influx_db_client::Points {
    let mut points_vector = std::vec::Vec::new();
    if let Some(cell) = data.cell {
        for cell_data in cell.data {
            let point = influx_db_client::Point::new("cell")
                        .add_tag("cell_id", i64::from(cell_data.id))
                        .add_tag("probe_id", data.probe_id)
                        .add_field("cell_voltage", i64::from(cell_data.voltage.voltage))
                        .add_field("balance_current", i64::from(cell_data.balance_current.current));
            points_vector.push(point);
        }
    }
    // TODO: add point creation for:
    // - data.battery
    // - data.solar
    // - data.grid_power
    // - data.house_power

    influx_db_client::Points::create_new(points_vector)
}