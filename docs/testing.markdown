---
title: "Testing"
permalink: /Testing
---

# Pegassas Energy Management System

[Home](https://m30819-2020.github.io/cw-code-t1)

## Unit Tests

- The following tests have been implmented in the code.
- Copies of the tests are listed below by section. This testing code is set to automatically run every time the base repo is updated.

### Rust - Aggregator

#### parser.rs

<details>
<summary> Test 1 - JSON to InfluxDB reading </summary>
<br>

``` Rust
#[cfg(test)]
mod test {
    use serde_json::Deserializer;
    use influx_db_client::Value;
    #[test]
    // First test -> Tests  to insure that parser can read a JSON string provided to it and provides InfluxDB points
    // Expected Result = Pass
    fn parser_test_cell() -> Result<(), Box<dyn std::error::Error>>{
        let json_string = "{
                                \"probe_id\":\"2\",
                                \"cell\": {
                                    \"data\": [{
                                        \"id\": 2, 
                                        \"battery_id\": 31,
                                        \"voltage\":{
                                            \"voltage\": 40,
                                            \"unit\": \"mV\"
                                        },
                                        \"balance_current\":{
                                            \"current\": 20,
                                            \"unit\": \"mA\"
                                        }
                                    }]
                                }
                            }";
        for data in Deserializer::from_str(json_string).into_iter::<super::Data>() {
            match data {
                Ok(data) => {
                    for point in super::parse_data(data){
                        if let Some(Value::String(probe_id)) = point.tags.get("probe_id") {
                            assert_eq!(probe_id, "2");
                        }
                        if let Some(Value::String(cell_id)) = point.tags.get("cell_id") {
                            assert_eq!(cell_id, "2");
                        }
                        if let Some(Value::Integer(cell_voltage)) = point.tags.get("cell_voltage") {
                            assert_eq!(cell_voltage, &(40 as i64));
                        }
                        if let Some(Value::Integer(balance_current)) = point.tags.get("balance_current") {
                            assert_eq!(balance_current, &(20 as i64));
                        }  
                    }
                },
                Err(e) => return Err(Box::new(e))
            }
            
        }
        Ok(())
    }
```

</details>
<br>
<details>
<summary> Test 2 - Test Battery Parser </summary>
<br>

```Rust
    #[test]
    fn parser_test_battery() -> Result<(), Box<dyn std::error::Error>>{
        let json_string = "{
                                \"probe_id\": \"1\",
                                \"battery\": {
                                    \"data\": {
                                        \"battery_current\": { 
                                            \"current\": 0,
                                            \"unit\": \"mA\"
                                        },
                                        \"state_charge\": {
                                            \"charge\": 0,
                                            \"unit\": \"Wh\"
                                        },
                                        \"charge_perc\": {
                                            \"charge\": 0,
                                            \"unit\": \"Percentage\"
                                        },
                                        \"battery_temp\": {
                                            \"temp\": 0,
                                            \"unit\": \"C\"
                                        }
                                    }
                                }
                            }";
        for data in Deserializer::from_str(json_string).into_iter::<super::Data>() {
            match data {
                Ok(data) => for point in super::parse_data(data){
                    if let Some(Value::String(probe_id)) = point.tags.get("probe_id") {
                        assert_eq!(probe_id, "1");
                    }
                    if let Some(Value::String(charge_perc)) = point.tags.get("charge_perc") {
                        assert_eq!(charge_perc, "0");
                    }
                    if let Some(Value::Integer(state_charge)) = point.tags.get("state_charge") {
                        assert_eq!(state_charge, &(0 as i64));
                    }
                    if let Some(Value::Integer(battery_current)) = point.tags.get("battery_current") {
                        assert_eq!(battery_current, &(0 as i64));
                    }  
                },
                Err(e) => return Err(Box::new(e))
            }
            
        }
        Ok(())
    }
```

</details>
<br>
<details>
<summary> Test 3 - Test Solar Parser </summary>
<br>

``` Rust
    #[test]
    fn parser_test_solar() -> Result<(), Box<dyn std::error::Error>>{
        let json_string = "{
                                \"probe_id\":\"2\",
                                \"solar\":{
                                    \"data\": [{
                                        \"id\": \"2\",
                                         \"sol_inv_voltage\":{
                                             \"voltage\": 0,
                                             \"unit\": \"mV\"
                                        },
                                         \"sol_inv_power\": {
                                             \"power\": 0,
                                             \"unit\": \"W\"
                                        },
                                        \"sol_inv_frequency\":{
                                            \"frequency\":0,
                                            \"unit\": \"Hz\"
                                        }
                                    }]
                                }
                            }";
        for data in Deserializer::from_str(json_string).into_iter::<super::Data>() {
            match data {
                Ok(data) => for point in super::parse_data(data){
                    if let Some(Value::String(probe_id)) = point.tags.get("probe_id") {
                        assert_eq!(probe_id, "2");
                    }
                    if let Some(Value::String(solar_id)) = point.tags.get("solar_id") {
                        assert_eq!(solar_id, "2");
                    }
                    if let Some(Value::String(sol_inv_voltage)) = point.tags.get("charge_perc") {
                        assert_eq!(sol_inv_voltage, "0");
                    }
                    if let Some(Value::Integer(sol_inv_power)) = point.tags.get("sol_inv_power") {
                        assert_eq!(sol_inv_power, &(0 as i64));
                    }
                    if let Some(Value::Integer(sol_inv_frequency)) = point.tags.get("sol_inv_frequency") {
                        assert_eq!(sol_inv_frequency, &(0 as i64));
                    }  
                },
                Err(e) => return Err(Box::new(e))
            }
        }
        Ok(())    
    }

```

</details>
<br>
<details>
<summary> Test 4 - Test Grid Power Parser </summary>
<br>

``` Rust
    #[test]
    fn parser_test_grid_power() -> Result<(), Box<dyn std::error::Error>>{
        let json_string = "{
                                \"probe_id\":\"2\",
                                \"grid_power\": {
                                    \"power\": 0,
                                    \"unit\": \"W\"
                                }
                           }";
        for data in Deserializer::from_str(json_string).into_iter::<super::Data>() {
            match data {
                Ok(data) => for point in super::parse_data(data){
                    if let Some(Value::String(probe_id)) = point.tags.get("probe_id") {
                        assert_eq!(probe_id, "2");
                    }
                    if let Some(Value::String(grid_power)) = point.tags.get("grid_power") {
                        assert_eq!(grid_power, "0");
                    }  
                },
                Err(e) => return Err(Box::new(e))
            }            
        }
        Ok(())
    }

```
</details>
<br>
<details>
<summary> Test 5 - Test House power Parser </summary>
<br>

``` Rust
    #[test]
    fn parser_test_house() -> Result<(), Box<dyn std::error::Error>>{
        let json_string = "{
                                \"probe_id\":\"2\",
                                    \"house_power\": {
                                            \"power\": 0,
                                            \"unit\": \"W\"
                                    }
                            }";        
       for data in Deserializer::from_str(json_string).into_iter::<super::Data>() {
            match data {
                Ok(data) => for point in super::parse_data(data){
                    if let Some(Value::String(probe_id)) = point.tags.get("probe_id") {
                        assert_eq!(probe_id, "2");
                    }
                    if let Some(Value::String(house_power)) = point.tags.get("house_power") {
                        assert_eq!(house_power, "0");
                    }  
                },
                Err(e) => return Err(Box::new(e))
            }
            
        }
        Ok(())
    }
}
```

</details>

#### worker.rs

<details>
<summary> Test 1 - XML Tree Test </summary>
<br>

``` Rust
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_presentation_url() -> Result<(), xmltree::ParseError> {
        let case1 = xmltree::Element::parse("<root></root>".as_bytes())?;
        let case2 = xmltree::Element::parse("<root><notdevice><child1></child1><presentationURL>sometext</presentationURL></notdevice></root>".as_bytes())?;
        let case3 = xmltree::Element::parse("<root><device><child1></child1><child2>sometext</child2></device></root>".as_bytes())?;
        let case4 = xmltree::Element::parse("<root><device><child1></child1><presentationURL>sometext</presentationURL></device></root>".as_bytes())?;
        let text: Option<String> = Some("sometext".to_string());
        assert_eq!(None, ProbeWorker::parse_presentation_url(&case1));
        assert_eq!(None, ProbeWorker::parse_presentation_url(&case2));
        assert_eq!(None, ProbeWorker::parse_presentation_url(&case3));
        assert_eq!(text, ProbeWorker::parse_presentation_url(&case4));
        Ok(())
    }
}
```

</details>

### Rust - Web Application

#### main.rs

<details>
<summary> Test 1 - Test password </summary>
<br>

``` Rust
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_pass_verify() -> Result<()>{
        let db_pass = String::from("testpassword");
        let hashed_pass = String::from("$2y$12$WXve5HUUGI19etxKGAh5q.DuMlgANQc13qTXbL/xG8041kTM/TovO");        
        assert_eq!(false,pass_verify(db_pass,hashed_pass));
        Ok(())
    }
}
```

</details>

### C++ - Probes

<details>
<summary> Test 1 - Hardware display test </summary>
<br>

``` C++
void test_display(){
    display.init();
    display.flipScreenVertically();
    display.setFont(ArialMT_Plain_10);
    display.drawString(0,0,"Hello!");
    display.display();
    delay(2000);
    UNITY_TEST_ASSERT(true, 15, "fail");
}
```

</details>

<details>
<summary> Test 2 - WiFi credential </summary>
<br>

``` C++
void test_save_wifi_creds(){
    UNITY_TEST_ASSERT(rc::check([](std::string SSID, std::string PSK){
        // Don't test for strings that are too long to store in the EEPROM
        RC_PRE((SSID.length() + PSK.length() + 3 <= 512));

        WiFi_creds creds = {
            SSID.c_str(),
            PSK.c_str()
        };
        save_wifi_creds(creds);
        // If SSID or PSK are invalid
        if (SSID.length() == 0 || PSK.length() < 8){
            // save_wifi_creds should set the first 3 bytes of EEPROM to 0 in the case of invalid SSID or PSK
            for (int i = 0; i < 3; ++i){
                if (EEPROM.read(i)){
                    return false;
                }
            }
            return true;
        }

        if (EEPROM.read(0) != 1){
            return false;
        }
        if (!compare_EEPROM(SSID.c_str(), 1)){
            return false;
        }
        if (!compare_EEPROM(PSK.c_str(), 2 + SSID.length())){
            return false;
        }
        return true;
    }), 69, "Rapidcheck test_save_wifi_creds fail");
}
```

</details>