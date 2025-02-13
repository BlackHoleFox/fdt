// This Source Code Form is subject to the terms of the Mozilla Public License,
// v. 2.0. If a copy of the MPL was not distributed with this file, You can
// obtain one at https://mozilla.org/MPL/2.0/.

extern crate std;

use crate::*;

static TEST: &[u8] = include_bytes!("../test.dtb");

#[test]
fn returns_fdt() {
    assert!(Fdt::new(TEST).is_ok());
}

#[test]
fn finds_root_node() {
    let fdt = Fdt::new(TEST).unwrap();
    assert!(fdt.find_node("/").is_some(), "couldn't find root node");
}

#[test]
fn finds_root_node_properties() {
    let fdt = Fdt::new(TEST).unwrap();
    let prop = fdt
        .find_node("/")
        .unwrap()
        .properties()
        .any(|p| p.name == "compatible" && p.value == b"riscv-virtio\0");

    assert!(prop);
}

#[test]
fn finds_child_of_root_node() {
    let fdt = Fdt::new(TEST).unwrap();
    assert!(fdt.find_node("/cpus").is_some(), "couldn't find cpus node");
}

#[test]
fn correct_flash_regions() {
    let fdt = Fdt::new(TEST).unwrap();
    let regions = fdt.find_node("/soc/flash").unwrap().reg().unwrap().collect::<std::vec::Vec<_>>();

    assert_eq!(
        regions,
        &[
            MemoryRegion { starting_address: 0x20000000 as *const u8, size: Some(0x2000000) },
            MemoryRegion { starting_address: 0x22000000 as *const u8, size: Some(0x2000000) }
        ]
    );
}

#[test]
fn finds_with_addr() {
    let fdt = Fdt::new(TEST).unwrap();
    assert_eq!(fdt.find_node("/soc/virtio_mmio@10004000").unwrap().name, "virtio_mmio@10004000");
}

#[test]
fn compatibles() {
    let fdt = Fdt::new(TEST).unwrap();
    let res = fdt
        .find_node("/soc/test")
        .unwrap()
        .compatible()
        .unwrap()
        .all()
        .all(|s| ["sifive,test1", "sifive,test0", "syscon"].contains(&s));

    assert!(res);
}

#[test]
fn parent_cell_sizes() {
    let fdt = Fdt::new(TEST).unwrap();
    let regions = fdt.find_node("/memory").unwrap().reg().unwrap().collect::<std::vec::Vec<_>>();

    assert_eq!(
        regions,
        &[MemoryRegion { starting_address: 0x80000000 as *const u8, size: Some(0x20000000) }]
    );
}

#[test]
fn no_properties() {
    let fdt = Fdt::new(TEST).unwrap();
    let regions = fdt.find_node("/emptyproptest").unwrap();
    assert_eq!(regions.properties().count(), 0);
}

#[test]
fn finds_all_nodes() {
    let fdt = Fdt::new(TEST).unwrap();

    let mut all_nodes: std::vec::Vec<_> = fdt.all_nodes().map(|n| n.name).collect();
    all_nodes.sort_unstable();

    assert_eq!(
        all_nodes,
        &[
            "/",
            "chosen",
            "clint@2000000",
            "cluster0",
            "core0",
            "cpu-map",
            "cpu@0",
            "cpus",
            "emptyproptest",
            "flash@20000000",
            "interrupt-controller",
            "memory@80000000",
            "pci@30000000",
            "plic@c000000",
            "poweroff",
            "reboot",
            "rtc@101000",
            "soc",
            "test@100000",
            "uart@10000000",
            "virtio_mmio@10001000",
            "virtio_mmio@10002000",
            "virtio_mmio@10003000",
            "virtio_mmio@10004000",
            "virtio_mmio@10005000",
            "virtio_mmio@10006000",
            "virtio_mmio@10007000",
            "virtio_mmio@10008000"
        ]
    )
}

#[test]
fn required_nodes() {
    let fdt = Fdt::new(TEST).unwrap();
    fdt.cpus().next().unwrap();
    fdt.memory();
    fdt.chosen();
}

#[test]
fn doesnt_exist() {
    let fdt = Fdt::new(TEST).unwrap();
    assert!(fdt.find_node("/this/doesnt/exist").is_none());
}
