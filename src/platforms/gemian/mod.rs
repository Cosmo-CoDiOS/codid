use std::io::BufReader;
use std::time::Duration;

use dbus::blocking::Connection;

use codi_contact_schema::{CoDiContact, CoDiContactNumber};

use crate::State;

mod codi_contact_schema;

#[allow(dead_code)]
pub fn get_dbus_contacts(s: &State) -> Vec<CoDiContact> {
    let log = s
        .lock()
        .expect("Unable to get a lock on the shared state.")
        .log
        .new(o!("task" => "get_dbus_contacts"));

    trace!(log, "Get connection to session bus...");

    let bus = Connection::new_session()
        .expect("Unable to open a connection to D-Bus"); // open d-bus

    let proxy =
        bus.with_proxy("org.freedesktop.DBus", "/", Duration::from_millis(500));

    let (avail_buses, ): (Vec<String>, ) = proxy
        .method_call("org.freedesktop.DBus", "ListActivatableNames", ())
        .expect("Unable to get a list of D-Bus interfaces!");

    let mut selected_bus: String = String::new();

    for bus in avail_buses {
        if bus.contains("org.gnome.evolution.dataserver.AddressBook") {
            selected_bus = bus;
            break;
        }
    }

    let proxy = bus.with_proxy(
        selected_bus,
        "/org/gnome/evolution/dataserver/AddressBookFactory",
        Duration::from_millis(500),
    );

    let (obj_path, eds_bus): (String, String) = proxy
        .method_call(
            "org.gnome.evolution.dataserver.AddressBookFactory",
            "OpenAddressBook",
            ("system-address-book", ),
        )
        .unwrap();

    let proxy = bus.with_proxy(eds_bus, obj_path, Duration::from_millis(500));
    let (contacts, ): (Vec<String>, ) = proxy
        .method_call(
            "org.gnome.evolution.dataserver.AddressBook",
            "GetContactListUids",
            ("", ),
        )
        .unwrap();

    let mut codi_contacts: Vec<CoDiContact> = Vec::new();

    for contact_id in contacts {
        let (contact_vcard, ): (String, ) = proxy
            .method_call(
                "org.gnome.evolution.dataserver.AddressBook",
                "GetContact",
                (contact_id, ),
            )
            .unwrap();

        let vcard = contact_vcard;
        let reader = ical::VcardParser::new(BufReader::new(vcard.as_bytes()));

        let mut contact_name = String::new();
        let mut contact_numbers: Vec<CoDiContactNumber> = Vec::new();

        for line in reader {
            for prop in line.unwrap().properties {
                if prop.name == "FN" {
                    contact_name =
                        prop.clone().value.expect("Unable to get contact name");
                }

                if prop.name == "TEL" {
                    let params = prop.params.unwrap().clone();
                    let value = prop.value.unwrap();
                    for entry in params {
                        if entry.0.clone() == "TYPE" {
                            let phone_type = entry.1[0].clone();
                            let phone_number = value.clone();

                            contact_numbers.push(CoDiContactNumber {
                                phone_type: phone_type.to_string(),
                                number: phone_number.to_string(),
                            });
                        }
                    }
                }
            }
        }

        if contact_name.is_empty() {
            continue;
        }

        if contact_numbers.is_empty() {
            continue;
        }

        let codi_contact = CoDiContact {
            name: contact_name,
            phone: contact_numbers,
        };

        codi_contacts.push(codi_contact);
    }

    // release lock
    drop(log);

    codi_contacts
}
