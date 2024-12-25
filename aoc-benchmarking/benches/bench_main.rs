use criterion::criterion_main;

use aoc_benchmarking::aoc_benches;
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
// import_marker

criterion_main! {
    benches
}

aoc_benches! {
    5,
    (
        day_001,
        "../day-001-historian-hysteria/input.txt",
        HistorianHysteria,
        "Combined"
    ),
    (
        day_002,
        "../day-002-red-nosed-reports/input.txt",
        RedNosedReports,
        "Combined"
    ),
    (
        day_003,
        "../day-003-mull-it-over/input.txt",
        MullItOver,
        "Combined"
    ),
    (
        day_004,
        "../day-004-ceres-search/input.txt",
        CeresSearch,
        "Part 1",
        "Part 2"
    ),
    (
        day_005,
        "../day-005-print-queue/input.txt",
        PrintQueue,
        "Part 1",
        "Part 2"
    ),
    (
        day_006,
        "../day-006-guard-gallivant/input.txt",
        GuardGallivant,
        "Combined"
    ),
    (
        day_007,
        "../day-007-bridge-repair/input.txt",
        BridgeRepair,
        "Combined"
    ),
    (
        day_008,
        "../day-008-resonant-collinearity/input.txt",
        ResonantCollinearity,
        "Part 1",
        "Part 2"
    ),
    (
        day_009,
        "../day-009-disk-fragmenter/input.txt",
        DiskFragmenter,
        "Combined"
    ),
    (
        day_010,
        "../day-010-hoof-it/input.txt",
        HoofIt,
        "Combined"
    ),
    (
        day_011,
        "../day-011-plutonium-pebbles/input.txt",
        PlutoniumPebbles,
        "Combined"
    ),
    (
        day_012,
        "../day-012-garden-groups/input.txt",
        GardenGroups,
        "Combined"
    ),
    (
        day_013,
        "../day-013-claw-contraption/input.txt",
        ClawContraption,
        "Part 1",
        "Part 2"
    ),
    (
        day_014,
        "../day-014-restroom-redoubt/input.txt",
        RestroomRedoubt,
        "Part 1",
        "Part 2"
    ),
    (
        day_015,
        "../day-015-warehouse-woes/input.txt",
        WarehouseWoes,
        "Combined"
    ),
    (
        day_016,
        "../day-016-reindeer-maze/input.txt",
        ReindeerMaze,
        "Combined"
    ),
    (
        day_017,
        "../day-017-chronospatial-computer/input.txt",
        ChronospatialComputer,
        "Part 1",
        "Part 2"
    ),
    (
        day_018,
        "../day-018-ram-run/input.txt",
        RamRun,
        "Combined"
    ),
    (
        day_019,
        "../day-019-linen-layout/input.txt",
        LinenLayout,
        "Combined"
    ),
    (
        day_020,
        "../day-020-race-condition/input.txt",
        RaceCondition,
        "Combined"
    ),
    (
        day_021,
        "../day-021-keypad-conundrum/input.txt",
        KeypadConundrum,
        "Combined"
    ),
    (
        day_022,
        "../day-022-monkey-market/input.txt",
        MonkeyMarket,
        "Combined"
    ),
    (
        day_023,
        "../day-023-lan-party/input.txt",
        LanParty,
        "Combined"
    ),
    (
        day_024,
        "../day-024-crossed-wires/input.txt",
        CrossedWires,
        "Combined"
    ),
    (
        day_025,
        "../day-025-code-chronicle/input.txt",
        CodeChronicle,
        "Combined"
    ),
    // bench_marker
}
