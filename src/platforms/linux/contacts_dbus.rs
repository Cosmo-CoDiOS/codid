use std::io::BufReader;
use std::time::Duration;

use dbus::blocking::Connection;

use crate::State;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub(crate) struct CoDiContactNumber {
    pub(crate) phone_type: String,
    pub(crate) number: String,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub(crate) struct CoDiContact {
    pub(crate) name: String,
    pub(crate) phone: Vec<CoDiContactNumber>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContactsDbusRetrieveError {
    SessionBusConnectFailure,
    ListInterfacesFailure,
    FindEdsInterfaceFailure,
    OpenEdsAddressBookFailure,
    GetEdsContactsFailure,
    GetEdsContactFailure,
}

pub(crate) type CoDiContacts = Vec<CoDiContact>;

#[allow(unreachable_code)]
#[allow(dead_code)]
pub(crate) fn get_dbus_contacts(s: &State) -> CoDiDbusContactsResult {
    trace!("Get connection to session bus...");

    let bus = match Connection::new_session() {
        Ok(res) => res,
        Err(_e) => {
            return Err(ContactsDbusRetrieveError::SessionBusConnectFailure)
        }
    };

    let proxy =
        bus.with_proxy("org.freedesktop.DBus", "/", Duration::from_millis(500));

    let (buses,): (Vec<String>,) = match proxy.method_call(
        "org.freedesktop.DBus",
        "ListActivatableNames",
        (),
    ) {
        Ok(buses) => buses,
        Err(_e) => {
            return Err(ContactsDbusRetrieveError::ListInterfacesFailure)
        }
    };

    let mut selected_bus: String = String::new();

    for potential in buses {
        if potential.contains("org.gnome.evolution.dataserver.AddressBook") {
            selected_bus = potential;
            break;
        }
    }

    if selected_bus == "" {
        return Err(ContactsDbusRetrieveError::FindEdsInterfaceFailure);
    }

    let proxy = bus.with_proxy(
        selected_bus,
        "/org/gnome/evolution/dataserver/AddressBookFactory",
        Duration::from_millis(500),
    );

    let (obj_path, eds_bus): (String, String) = match proxy.method_call(
        "org.gnome.evolution.dataserver.AddressBookFactory",
        "OpenAddressBook",
        ("system-address-book",),
    ) {
        Ok(res) => res,
        Err(_e) => {
            return Err(ContactsDbusRetrieveError::OpenEdsAddressBookFailure)
        }
    };

    let proxy = bus.with_proxy(eds_bus, obj_path, Duration::from_millis(500));
    let (contacts,): (Vec<String>,) = match proxy.method_call(
        "org.gnome.evolution.dataserver.AddressBook",
        "GetContactListUids",
        ("",),
    ) {
        Ok(res) => res,
        Err(_e) => {
            return Err(ContactsDbusRetrieveError::GetEdsContactsFailure)
        }
    };

    let mut codi_contacts: Vec<CoDiContact> = Vec::new();

    for contact_id in contacts {
        let (contact,): (String,) = match proxy.method_call(
            "org.gnome.evolution.dataserver.AddressBook",
            "GetContact",
            (contact_id,),
        ) {
            Ok(res) => res,
            Err(_e) => {
                return Err(ContactsDbusRetrieveError::GetEdsContactFailure)
            }
        };

        let reader = ical::VcardParser::new(BufReader::new(contact.as_bytes()));

        let mut contact_name = String::new();
        let mut contact_numbers: Vec<CoDiContactNumber> = Vec::new();
        'vcardloop: for line in reader {
            println!("{:?}", line);

            let line = match line {
                Ok(res) => res,
                _ => continue 'vcardloop,
            };

            for prop in line.clone().properties {
                let params = match prop.clone().params {
                    Some(res) => res,
                    _ => continue 'vcardloop,
                };
                let value = match prop.clone().value {
                    Some(res) => res,
                    _ => continue 'vcardloop,
                };

                if prop.name == "FN" {
                    contact_name = match prop.clone().value {
                        Some(res) => res.clone(),
                        _ => continue 'vcardloop,
                    };
                }

                if prop.name == "TEL" {
                    for entry in params {
                        if &entry.0 == "TYPE" {
                            let phone_type = entry.1[0].clone();
                            let phone_number = value.clone();

                            contact_numbers.push(CoDiContactNumber {
                                phone_type: phone_type.to_string(),
                                number: phone_number.to_string(),
                            });
                        }
                    }
                }

                break 'vcardloop;
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

    Ok(codi_contacts)
}
