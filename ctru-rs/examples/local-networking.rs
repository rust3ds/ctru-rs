//! Local networking example.
//!
//! This example showcases local networking using the UDS module.

use ctru::prelude::*;
use ctru::services::uds::*;

fn handle_status_event(uds: &Uds, prev_node_mask: u16) -> ctru::Result<u16> {
    println!("Connection status event signalled");
    let status = uds.connection_status()?;
    println!("Status: {status:#02X?}");
    let left = prev_node_mask & (status.node_bitmask() ^ prev_node_mask);
    let joined = status.node_bitmask() & (status.node_bitmask() ^ prev_node_mask);
    for i in 0..16 {
        if left & (1 << i) != 0 {
            println!("Node {} disconnected", i + 1);
        }
    }
    for i in 0..16 {
        if joined & (1 << i) != 0 {
            println!(
                "Node {} connected: {:?}",
                i + 1,
                uds.node_info(NodeID::Node(i + 1))
            );
        }
    }
    Ok(status.node_bitmask())
}

fn main() -> Result<(), Error> {
    let apt = Apt::new().unwrap();
    let mut hid = Hid::new().unwrap();
    let gfx = Gfx::new().unwrap();
    let console = Console::new(gfx.top_screen.borrow_mut());

    println!("Local networking demo");

    let mut uds = Uds::new(None).unwrap();

    println!("UDS initialised");

    enum State {
        Initialised,
        Scanning,
        DrawList,
        List,
        Connect,
        Connected,
        Create,
        Created,
    }

    let mut state = State::Initialised;

    println!("Press A to start scanning or B to create a new network");

    let mut networks = vec![];
    let mut selected_network = 0;

    let mut mode = ConnectionType::Client;

    let mut channel: u8;
    let data_channel: u8 = 1;

    let mut prev_node_mask = 0;

    while apt.main_loop() {
        gfx.wait_for_vblank();

        hid.scan_input();
        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        match state {
            State::Initialised => {
                if hid.keys_down().contains(KeyPad::A) {
                    state = State::Scanning;
                    console.clear();
                    prev_node_mask = 0;
                } else if hid.keys_down().contains(KeyPad::B) {
                    state = State::Create;
                    console.clear();
                    prev_node_mask = 0;
                }
            }
            State::Scanning => {
                println!("Scanning...");

                let nwks = uds.scan(b"HBW\x10", None, None);

                match nwks {
                    Ok(n) => {
                        if n.is_empty() {
                            state = State::Initialised;
                            console.clear();
                            println!("Scanned successfully; no networks found");
                            println!("Press A to start scanning or B to create a new network");
                        } else {
                            networks = n;
                            selected_network = 0;
                            state = State::DrawList;
                        }
                    }
                    Err(e) => {
                        state = State::Initialised;
                        console.clear();
                        eprintln!("Error while scanning: {e}");
                        println!("Press A to start scanning or B to create a new network");
                    }
                }
            }
            State::DrawList => {
                console.clear();

                println!(
                    "Scanned successfully; {} network{} found",
                    networks.len(),
                    if networks.len() == 1 { "" } else { "s" }
                );

                println!(
                    "D-Pad to select, A to connect as client, R + A to connect as spectator, B to create a new network"
                );

                for (index, n) in networks.iter().enumerate() {
                    println!(
                        "{} Username: {}",
                        if index == selected_network { ">" } else { " " },
                        n.nodes()[0].unwrap().username()
                    );
                }

                state = State::List;
            }
            State::List => {
                if hid.keys_down().contains(KeyPad::UP) && selected_network > 0 {
                    selected_network -= 1;
                    state = State::DrawList;
                } else if hid.keys_down().contains(KeyPad::DOWN)
                    && selected_network < networks.len() - 1
                {
                    selected_network += 1;
                    state = State::DrawList;
                } else if hid.keys_down().contains(KeyPad::A) {
                    state = State::Connect;
                    mode = if hid.keys_held().contains(KeyPad::R) {
                        ConnectionType::Spectator
                    } else {
                        ConnectionType::Client
                    };
                } else if hid.keys_down().contains(KeyPad::B) {
                    state = State::Create;
                }
            }
            State::Connect => {
                let appdata = uds.network_appdata(&networks[selected_network], None)?;
                println!("App data: {:02X?}", appdata);

                if let Err(e) = uds.connect_network(
                    &networks[selected_network],
                    b"udsdemo passphrase c186093cd2652741\0",
                    mode,
                    data_channel,
                ) {
                    console.clear();
                    eprintln!("Error while connecting to network: {e}");
                    state = State::Initialised;
                    println!("Press A to start scanning or B to create a new network");
                } else {
                    channel = uds.channel()?;
                    println!("Connected using channel {}", channel);

                    let appdata = uds.appdata(None)?;
                    println!("App data: {:02X?}", appdata);

                    if uds.wait_status_event(false, false)? {
                        prev_node_mask = handle_status_event(&uds, prev_node_mask)?;
                    }

                    println!("Press A to stop data transfer");
                    state = State::Connected;
                }
            }
            State::Connected => {
                let packet = uds.pull_packet();

                match packet {
                    Ok(p) => {
                        if let Some((pkt, node)) = p {
                            println!(
                                "{:02X}{:02X}{:02X}{:02X} from {:?}",
                                pkt[0], pkt[1], pkt[2], pkt[3], node
                            );
                        }

                        if uds.wait_status_event(false, false)? {
                            prev_node_mask = handle_status_event(&uds, prev_node_mask)?;
                        }

                        if hid.keys_down().contains(KeyPad::A) {
                            uds.disconnect_network()?;
                            state = State::Initialised;
                            console.clear();
                            println!("Press A to start scanning or B to create a new network");
                        } else if !hid.keys_down().is_empty() || !hid.keys_up().is_empty() {
                            let transfer_data = hid.keys_held().bits();
                            if mode != ConnectionType::Spectator {
                                uds.send_packet(
                                    &transfer_data.to_le_bytes(),
                                    NodeID::Broadcast,
                                    data_channel,
                                    SendFlags::Default,
                                )?;
                            }
                        }
                    }
                    Err(e) => {
                        uds.disconnect_network()?;
                        console.clear();
                        eprintln!("Error while grabbing packet from network: {e}");
                        state = State::Initialised;
                        println!("Press A to start scanning or B to create a new network");
                    }
                }
            }
            State::Create => {
                console.clear();
                println!("Creating network...");

                match uds.create_network(
                    b"HBW\x10",
                    None,
                    None,
                    b"udsdemo passphrase c186093cd2652741\0",
                    data_channel,
                ) {
                    Ok(_) => {
                        let appdata = [0x69u8, 0x8a, 0x05, 0x5c]
                            .into_iter()
                            .chain((*b"Test appdata.").into_iter())
                            .chain(std::iter::repeat_n(0, 3))
                            .collect::<Vec<_>>();

                        uds.set_appdata(&appdata)?;

                        println!("Press A to stop data transfer");
                        state = State::Created;
                    }
                    Err(e) => {
                        console.clear();
                        eprintln!("Error while creating network: {e}");
                        state = State::Initialised;
                        println!("Press A to start scanning or B to create a new network");
                    }
                }
            }
            State::Created => {
                let packet = uds.pull_packet();

                match packet {
                    Ok(p) => {
                        if let Some((pkt, node)) = p {
                            println!(
                                "{:02X}{:02X}{:02X}{:02X} from {:?}",
                                pkt[0], pkt[1], pkt[2], pkt[3], node
                            );
                        }

                        if uds.wait_status_event(false, false)? {
                            prev_node_mask = handle_status_event(&uds, prev_node_mask)?;
                        }

                        if hid.keys_down().contains(KeyPad::A) {
                            uds.destroy_network()?;
                            state = State::Initialised;
                            console.clear();
                            println!("Press A to start scanning or B to create a new network");
                        } else if !hid.keys_down().is_empty() || !hid.keys_up().is_empty() {
                            let transfer_data = hid.keys_held().bits();
                            uds.send_packet(
                                &transfer_data.to_le_bytes(),
                                NodeID::Broadcast,
                                data_channel,
                                SendFlags::Default,
                            )?;
                        }
                    }
                    Err(e) => {
                        uds.destroy_network()?;
                        console.clear();
                        eprintln!("Error while grabbing packet from network: {e}");
                        state = State::Initialised;
                        println!("Press A to start scanning or B to create a new network");
                    }
                }
            }
        }
    }

    Ok(())
}
