extern crate blurz;

use std::error::Error;
use std::process::exit;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use blurz::bluetooth_adapter::BluetoothAdapter;
use blurz::bluetooth_device::BluetoothDevice;
use blurz::bluetooth_discovery_session::BluetoothDiscoverySession;
use blurz::bluetooth_gatt_characteristic::BluetoothGATTCharacteristic;
use blurz::bluetooth_gatt_descriptor::BluetoothGATTDescriptor;
use blurz::bluetooth_gatt_service::BluetoothGATTService;
use blurz::bluetooth_session::BluetoothSession;

const MUG_BASE_UUID: &str = "fc543622-236c-4c94-8fa9-944a3e5353fa";

const UUID_BATTERY: &str = "fc540007-236c-4c94-8fa9-944a3e5353fa";
const UUID_DRINK_TEMP: &str = "fc540002-236c-4c94-8fa9-944a3e5353fa";
const UUID_DSK: &str = "fc54000e-236c-4c94-8fa9-944a3e5353fa";
const UUID_LAST_LOCATION: &str = "fc54000a-236c-4c94-8fa9-944a3e5353fa";
const UUID_LED: &str = "fc540014-236c-4c94-8fa9-944a3e5353fa";
const UUID_LIQUID_LEVEL: &str = "fc540005-236c-4c94-8fa9-944a3e5353fa";
const UUID_LIQUID_STATE: &str = "fc540008-236c-4c94-8fa9-944a3e5353fa";
const UUID_STATISTICS: &str = "fc540013-236c-4c94-8fa9-944a3e5353fa";
const UUID_TARGET_TEMP: &str = "fc540003-236c-4c94-8fa9-944a3e5353fa";
const UUID_TEMP_UNIT: &str = "fc540004-236c-4c94-8fa9-944a3e5353fa";

const BATTERY_LEVEL_LOW: u32 = 11;

enum DrinkState {
    Unknown = 0,
    Empty = 1,
    ColdNoControl = 2,
    Cooling = 4,
    Heating = 5,
    TargetTemp = 6,
    WarmNocontrol = 7,
}

fn main() {
    let bt_session = BluetoothSession::create_session(None).unwrap();
    let adapter = BluetoothAdapter::init(&bt_session).unwrap();

    println!("Scanning for ember mug...");
    let mug_ids = mug_scan(&bt_session, &adapter, 60000);
    if mug_ids.is_none() {
        println!("Didn't find an Ember mug.");
        exit(1);
    }
    let mug_ids = mug_ids.unwrap();
    let device = BluetoothDevice::new(&bt_session, mug_ids[0].to_string());

    if !mug_connect(&device, 60000) {
        println!("Couldn't connect to the mug. Make sure its in pair mode.");
        exit(1);
    }

    //println!("Device is ready!");

    //let gatt_services = device.get_gatt_services().unwrap();

    
    let services: Vec<BluetoothGATTService> = device.get_gatt_services().unwrap().iter()
        .map(|svc| {
            BluetoothGATTService::new(&bt_session, svc.to_owned())
        }).collect();
    
    let mut temp_unit = 1;
    for svc in services {
        //println!("svc.get_id(): {}", svc.get_uuid().unwrap());
        if svc.get_uuid().unwrap().contains(MUG_BASE_UUID) {
            for chr in svc.get_gatt_characteristics().unwrap() {
                let gatt = BluetoothGATTCharacteristic::new(&bt_session, chr);
                if gatt.get_uuid().unwrap().contains(UUID_LIQUID_STATE) {
                    print!("drink_state=");
                    let state = gatt.read_value(None).unwrap()[0];

                    match state {
                        1 => {println!("empty")}
                        2 => {println!("cold_no_control")}
                        4 => {println!("cooling")}
                        5 => {println!("heating")}
                        6 => {println!("target_temp")}
                        7 => {println!("warm_no_control")}
                        _ => {println!("unknown")}
                    }
                } else if gatt.get_uuid().unwrap().contains(UUID_BATTERY) {
                    let battery = gatt.read_value(None).unwrap()[0];
                    println!("battery={}", battery);
                } else if gatt.get_uuid().unwrap().contains(UUID_TEMP_UNIT) {
                    temp_unit = gatt.read_value(None).unwrap()[0];
                } else if gatt.get_uuid().unwrap().contains(UUID_DRINK_TEMP) {
                    loop {
                        let level = gatt.read_value(None).unwrap()[0];
                        print!("liquid_temp={}", level);
                        if temp_unit == 1 {
                            println!("f");
                        } else {
                            println!("c");
                        }
                        thread::sleep(Duration::from_millis(100));
                    }
                } else if gatt.get_uuid().unwrap().contains(UUID_TARGET_TEMP) {
                    let level = gatt.read_value(None).unwrap()[0];
                    print!("liquid_temp_target={}", level);
                    if temp_unit == 1 {
                        println!("f");
                    } else {
                        println!("c");
                    }
                } else if gatt.get_uuid().unwrap().contains(UUID_LIQUID_LEVEL) {
                    let level = gatt.read_value(None).unwrap()[0];
                    println!("liquid_level={}", level);
                    println!("liquid_level_pct={}", (level as u32)*100/30);
                } 
            }
        }
    }
    /*let service = BluetoothGATTService::new(&bt_session, MUG_BASE_UUID.to_string());

    let chars = service.get_gatt_characteristics().unwrap();*/

    //service.

    //let temp_gatt = BluetoothGATTCharacteristic(&bt_session, UUID_DRINK_TEMP.to_owned());
    /*


    
    


    for service in gatt_services {
        let gatt_service = BluetoothGATTService::new(&bt_session, service.to_string());

        println!("  Gatt service Id: {} UUID: {:?} Device : {:?} Is primary: {:?} - service {}",
                    gatt_service.get_id(),
                    gatt_service.get_uuid(),
                    gatt_service.get_device(),
                    gatt_service.is_primary(),
                    service);

        match gatt_service.get_gatt_characteristics() {
            Ok(ref gat_chars) => {
                for characteristics in gat_chars {
                    let gatt_char = BluetoothGATTCharacteristic::new(&bt_session, characteristics.to_owned());

                    if characteristics.contains(UUID_LIQUID_STATE) {
                        println!("Here!");
                        println!("Here!");
                        println!("Here!");
                        println!("Here!");
                        println!("Here!");
                    }

                    println!("    Characteristic Name: {} UUID: {:?} Flags: {:?}",
                                characteristics, gatt_char.get_uuid(),
                                gatt_char.get_flags());
                }
            },
            Err(e) => println!("    Error get_gatt_characteristics(): {:?}", e)
        }
    }*/
}

fn mug_scan(bt_session: &BluetoothSession, adapter: &BluetoothAdapter, ms_timeout: u128) -> Option<Vec<String>> {
    let disc_session = BluetoothDiscoverySession::create_session(bt_session, adapter.get_id()).unwrap();

    disc_session.start_discovery().unwrap();

    let start = Instant::now();
    loop {
        let dev_ids = adapter.get_device_list().unwrap();
        let devices: Vec<String> = dev_ids.iter().filter_map(
            |devaddr| {
                let dev = BluetoothDevice::new(bt_session, devaddr.clone());
                let devname = dev.get_name().unwrap_or("".to_string());
                if devname.contains("Ember Ceramic Mug") {
                    Some(devaddr.to_string())
                } else {
                    None
                }
            }
        ).collect();

        if devices.len() > 0 {
            return Some(devices);
        }

        if start.elapsed().as_millis() > ms_timeout {
            return None;
        }
        thread::sleep(Duration::from_millis(100));
    }
}

fn mug_connect(device: &BluetoothDevice, ms_timeout: u128) -> bool {
    if let Err(e) = device.connect(ms_timeout as i32) {
        println!("Error on connecting: {:?}", e);
    }

    let start = Instant::now();
    loop {
        if !device.is_paired().unwrap() {
            println!("Device isn't paired, attempting pairing");
            if let Err(e) = device.pair() {
                println!("Error on pairing: {:?}", e);
            } else {
                break;
            }
        } else {
            break;
        }
        
        if start.elapsed().as_millis() > ms_timeout {
            return false;
        }
        thread::sleep(Duration::from_millis(100));
    }

    assert!(device.is_ready_to_receive().unwrap());

    device.set_trusted(true).unwrap();

    true
}
