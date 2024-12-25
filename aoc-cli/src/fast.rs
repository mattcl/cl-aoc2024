use std::env;

use aoc_plumbing::Problem;
use bridge_repair::BridgeRepair;
use ceres_search::CeresSearch;
use chronospatial_computer::ChronospatialComputer;
use claw_contraption::ClawContraption;
use code_chronicle::CodeChronicle;
use crossed_wires::CrossedWires;
use disk_fragmenter::DiskFragmenter;
use garden_groups::GardenGroups;
use guard_gallivant::GuardGallivant;
use historian_hysteria::HistorianHysteria;
use hoof_it::HoofIt;
use keypad_conundrum::KeypadConundrum;
use lan_party::LanParty;
use linen_layout::LinenLayout;
use monkey_market::MonkeyMarket;
use mull_it_over::MullItOver;
use plutonium_pebbles::PlutoniumPebbles;
use print_queue::PrintQueue;
use race_condition::RaceCondition;
use ram_run::RamRun;
use red_nosed_reports::RedNosedReports;
use reindeer_maze::ReindeerMaze;
use resonant_collinearity::ResonantCollinearity;
use restroom_redoubt::RestroomRedoubt;
use warehouse_woes::WarehouseWoes;

pub fn run() -> anyhow::Result<()> {
    let day: u8 = env::var("AOC_DAY")?.parse()?;
    let input_file = env::var("AOC_INPUT")?;
    let input = std::fs::read_to_string(&input_file)?;

    let out = match day {
        1 => serde_json::to_string(&HistorianHysteria::solve(&input)?)?,
        2 => serde_json::to_string(&RedNosedReports::solve(&input)?)?,
        3 => serde_json::to_string(&MullItOver::solve(&input)?)?,
        4 => serde_json::to_string(&CeresSearch::solve(&input)?)?,
        5 => serde_json::to_string(&PrintQueue::solve(&input)?)?,
        6 => serde_json::to_string(&GuardGallivant::solve(&input)?)?,
        7 => serde_json::to_string(&BridgeRepair::solve(&input)?)?,
        8 => serde_json::to_string(&ResonantCollinearity::solve(&input)?)?,
        9 => serde_json::to_string(&DiskFragmenter::solve(&input)?)?,
        10 => serde_json::to_string(&HoofIt::solve(&input)?)?,
        11 => serde_json::to_string(&PlutoniumPebbles::solve(&input)?)?,
        12 => serde_json::to_string(&GardenGroups::solve(&input)?)?,
        13 => serde_json::to_string(&ClawContraption::solve(&input)?)?,
        14 => serde_json::to_string(&RestroomRedoubt::solve(&input)?)?,
        15 => serde_json::to_string(&WarehouseWoes::solve(&input)?)?,
        16 => serde_json::to_string(&ReindeerMaze::solve(&input)?)?,
        17 => serde_json::to_string(&ChronospatialComputer::solve(&input)?)?,
        18 => serde_json::to_string(&RamRun::solve(&input)?)?,
        19 => serde_json::to_string(&LinenLayout::solve(&input)?)?,
        20 => serde_json::to_string(&RaceCondition::solve(&input)?)?,
        21 => serde_json::to_string(&KeypadConundrum::solve(&input)?)?,
        22 => serde_json::to_string(&MonkeyMarket::solve(&input)?)?,
        23 => serde_json::to_string(&LanParty::solve(&input)?)?,
        24 => serde_json::to_string(&CrossedWires::solve(&input)?)?,
        25 => serde_json::to_string(&CodeChronicle::solve(&input)?)?,
        _ => "\"not implemented\"".into(),
    };

    println!("{}", out);

    Ok(())
}
