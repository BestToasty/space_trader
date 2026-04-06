use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Meta {
    pub total: i32,
    pub page: i32,
    pub limit: i32,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FrameSymbol {
    FRAME_PROBE,
    FRAME_DRONE,
    FRAME_INTERCEPTOR,
    FRAME_RACER,
    FRAME_FIGHTER,
    FRAME_FRIGATE,
    FRAME_SHUTTLE,
    FRAME_EXPLORER,
    FRAME_MINER,
    FRAME_LIGHT_FREIGHTER,
    FRAME_HEAVY_FREIGHTER,
    FRAME_TRANSPORT,
    FRAME_DESTROYER,
    FRAME_CRUISER,
    FRAME_CARRIER,
    FRAME_BULK_FREIGHTER,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ReactorSymbol {
    REACTOR_SOLAR_I,
    REACTOR_FUSION_I,
    REACTOR_FISSION_I,
    REACTOR_CHEMICAL_I,
    REACTOR_ANTIMATTER_I,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EngineSymbol {
    ENGINE_IMPULSE_DRIVE_I,
    ENGINE_ION_DRIVE_I,
    ENGINE_ION_DRIVE_II,
    ENGINE_HYPER_DRIVE_I,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Reactor {
    pub condition: i32,
    pub description: String,
    pub integrity: i32,
    pub name: String,
    pub power_output: i32,
    pub quality: i32,
    pub requirements: Requirement,
    pub symbol: ReactorSymbol,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Requirement {
    pub crew: Option<i32>,
    pub power: Option<i32>,
    pub slots: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Engine {
    pub condition: i32,
    pub description: String,
    pub integrity: i32,
    pub name: String,
    pub quality: i32,
    pub requirements: Requirement,
    pub speed: i32,
    pub symbol: EngineSymbol,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Frame {
    pub condition: i32,
    pub description: String,
    pub fuel_capacity: i32,
    pub integrity: i32,
    pub module_slots: i32,
    pub mounting_points: i32,
    pub name: String,
    pub quality: i32,
    pub requirements: Requirement,
    pub symbol: FrameSymbol,
}
