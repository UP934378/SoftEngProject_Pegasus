use serde::{Deserialize, Serialize};
use std::fmt::Debug;

// Unit enums

/// Voltage metric
#[derive(Serialize, Deserialize, Debug)]
pub enum VoltageUnit {
    mV,
    V
}

/// Current metric 
#[derive(Serialize, Deserialize, Debug)]
pub enum CurrentUnit {
    mA
}

/// Charge metric 
#[derive(Serialize, Deserialize, Debug)]
pub enum ChargeUnit {
    Percentage,
    Wh
}

/// Temperature metric
#[derive(Serialize, Deserialize, Debug)]
pub enum TemperatureUnit {
    C
}

/// Power unit metric
#[derive(Serialize, Deserialize, Debug)]
pub enum PowerUnit {
    W
}

/// Frequency metric
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

/// Cell probe information structure
#[derive(Serialize, Deserialize, Debug)]
pub struct CellData {
    id : u16,
    battery_id : u16,
    voltage : Voltage,
    balance_current : Option<Current>
}

/// Battery probe information structure
#[derive(Serialize, Deserialize, Debug)]
pub struct BatteryData {
    id : u16,
    state_charge : Charge,
    charge_perc : Charge,
    balance_current : Current,
    battery_temp : Option<Temperature>
}

/// Solar Panel information structure
#[derive(Serialize, Deserialize, Debug)]
pub struct SolarData {
    id: String,
    sol_inv_voltage : Voltage,
    sol_inv_power : Power,
    sol_inv_frequency : Frequency
}

/// Container structure for cell informations
#[derive(Serialize, Deserialize, Debug)]
pub struct Cell {
    data : std::vec::Vec<CellData>
}

/// Container structure for battery informations
#[derive(Serialize, Deserialize, Debug)]
pub struct Battery {
    data : BatteryData
}

/// Container structure for Solar panel informations
#[derive(Serialize, Deserialize, Debug)]
pub struct Solar {
    data : std::vec::Vec<SolarData>
}

/// Top level structure to store deserialized data from probes
#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    probe_id : String,
    cell : Option<Cell>,
    battery : Option<Battery>,
    solar : Option<Solar>,
    grid_power : Option<Power>,
    house_power : Option<Power>
}

/// Parse information from data probe and generate point to be added to Influx database
pub fn parse_data(data: Data) -> influx_db_client::Points {
    let mut points_vector = std::vec::Vec::new();
    // Cell Data - pointers to InfluxDB - Uses data_schema.json
    if let Some(cell) = data.cell {
        for cell_data in cell.data {
            let mut point = influx_db_client::Point::new("cell")
                        .add_tag("cell_id", i64::from(cell_data.id))
                        .add_tag("probe_id", data.probe_id.clone())
                        .add_field("cell_voltage", i64::from(cell_data.voltage.voltage));
            if let Some(balance_current) = cell_data.balance_current {
                point = point.add_field("balance_current", i64::from(balance_current.current));
            }
                        
            points_vector.push(point);
        }
    }
    // Battery Data - pointers to InfluxDB - Uses data_schema.json
    if let Some(battery) = data.battery {
        let point = influx_db_client::Point::new("battery")
            .add_tag("probe_id", data.probe_id.clone())
            .add_field("battery_current", i64::from(battery.data.balance_current.current))
            .add_field("state_charge", i64::from(battery.data.state_charge.charge))
            .add_field("charge_perc", i64::from(battery.data.charge_perc.charge));
        let point = match battery.data.battery_temp { //Temp here = temperature of battery
            Some(temp) => point.add_field("battery_temp", i64::from(temp.temp)),
            None => point
        };
        points_vector.push(point);
    }

    // Solar Data - pointers to InfluxDB - Uses data_schema.json
    if let Some(solar) = data.solar {
        for solar_data in solar.data {
            let point = influx_db_client::Point::new("inverter")
                        .add_tag("solar_data", solar_data.id)
                        .add_tag("probe_id", data.probe_id.clone())
                        .add_field("sol_inv_voltage", i64::from(solar_data.sol_inv_voltage.voltage))
                        .add_field("sol_inv_power", i64::from(solar_data.sol_inv_power.power))
                        .add_field("sol_inv_frequency", i64::from(solar_data.sol_inv_frequency.frequency));
            points_vector.push(point);
        }
    }

    // Grid Power Data - pointers to InfluxDB - Uses data_schema.json
     if let Some(grid_power) = data.grid_power {
        let point = influx_db_client::Point::new("grid_power")
            .add_tag("probe_id", data.probe_id.clone())
            .add_field("grid_power", i64::from(grid_power.power));
        points_vector.push(point);
    }

    // House Power Data - pointers to InfluxDB - Uses data_schema.json
    if let Some(house_power) = data.house_power {
        let point = influx_db_client::Point::new("house_power")
            .add_tag("probe_id", data.probe_id.clone())
            .add_field("house_power", i64::from(house_power.power));
        points_vector.push(point);
    }

    influx_db_client::Points::create_new(points_vector)
}