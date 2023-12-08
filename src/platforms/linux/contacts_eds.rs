//! `contacts_eds` gets contacts from Evolution's EDS interface, for inserting into CoDi over UART.

#![allow(dead_code)]

use std::io::BufReader;

use thiserror::Error;
use zbus::blocking::{Connection, Proxy};
use zbus::Error as ZbusError;

#[derive(Clone, Debug)]
pub(crate) struct CoDiContactNumber {
    phone_type: String,
    number: String,
}

impl CoDiContactNumber {
    pub(crate) fn get_phone_type(&self) -> Option<String> {
        if self.phone_type.is_empty() {
            return None;
        }

        Some(String::from(&self.phone_type))
    }

    pub(crate) fn get_phone_number(&self) -> Option<String> {
        if self.number.is_empty() {
            return None;
        }

        Some(String::from(&self.number))
    }
}

#[derive(Clone, Debug)]
pub(crate) struct CoDiContact {
    name: String,
    phone: CoDiContactNumbers,
}

impl CoDiContact {
    pub(crate) fn get_contact_name(&self) -> Option<String> {
        if self.name.is_empty() {
            return None;
        }

        Some(String::from(&self.name))
    }

    pub(crate) fn get_numbers(&self) -> Option<CoDiContactNumbers> {
        if self.phone.is_empty() {
            return None;
        }

        Some(self.phone.clone())
    }
}

#[derive(Debug, Error, PartialEq)]
pub(crate) enum DbusContactsError {
    #[error("Failed to connect to the D-Bus User Session bus.")]
    SessionBusConnectFailure(#[source] ZbusError),
    #[error("Error retrieving D-Bus activatable interfaces.")]
    ListInterfacesFailure(#[source] ZbusError),
    #[error("EDS interface missing.")]
    EdsInterfaceMissingFailure,
    #[error("Error searching for the EDS interface.")]
    EdsInterfaceSearchFailure(#[source] ZbusError),
    #[error("Error opening EDS address book.")]
    AddressBookOpenFailure(#[source] ZbusError),
    #[error("Error getting `Vec<T>` of contacts")]
    GetContactsFailure(#[source] ZbusError),
    #[error("Error getting single contact.")]
    GetContactFailure(#[source] ZbusError),
}

pub(crate) type CoDiContacts = Vec<CoDiContact>;
pub(crate) type CoDiContactNumbers = Vec<CoDiContactNumber>;
pub(crate) type CoDiDbusContactsResult<T = CoDiContacts, E = DbusContactsError> = anyhow::Result<T, E>;

pub(crate) fn get_dbus_contacts() -> CoDiDbusContactsResult {
    let bus = Connection::session()
        .map_err(|e| DbusContactsError::SessionBusConnectFailure(e))?;

    let all_ifaces =
        Proxy::new(&bus, "org.freedesktop.DBus", "/", "org.freedesktop.DBus")
            .map_err(|e| DbusContactsError::EdsInterfaceSearchFailure(e))?;

    let dbus_all_names: Vec<String> = all_ifaces
        .call("ListActivatableNames", &())
        .map_err(|e| DbusContactsError::EdsInterfaceSearchFailure(e))?;

    let iface = if let Some(x) = dbus_all_names
        .into_iter()
        .find(|s| s.contains("org.gnome.evolution.dataserver.AddressBook"))
    {
        x
    } else {
        return Err(DbusContactsError::EdsInterfaceMissingFailure);
    };

    let mut proxy = Proxy::new(
        &bus,
        iface,
        "/org/gnome/evolution/dataserver/AddressBookFactory",
        "org.gnome.evolution.dataserver.AddressBookFactory",
    )
    .map_err(|e| DbusContactsError::AddressBookOpenFailure(e))?;

    let (obj_path, eds_bus): (String, String) = proxy
        .call("OpenAddressBook", &("system-address-book",))
        .map_err(|e| DbusContactsError::AddressBookOpenFailure(e))?;

    proxy = Proxy::new(
        &bus,
        eds_bus,
        obj_path,
        "org.gnome.evolution.dataserver.AddressBook",
    )
    .map_err(|e| DbusContactsError::AddressBookOpenFailure(e))?;

    let contacts: Vec<String> = proxy
        .call("GetContactListUids", &(""))
        .map_err(|e| DbusContactsError::GetContactsFailure(e))?;

    let mut codi_contacts: Vec<CoDiContact> = Vec::new();

    for contact_id in contacts {
        let (contact,): (String,) = proxy
            .call("GetContact", &(contact_id,))
            .map_err(|e| DbusContactsError::GetContactFailure(e))?;

        let reader = ical::VcardParser::new(BufReader::new(contact.as_bytes()));

        let mut contact_name = String::new();
        let mut contact_numbers: Vec<CoDiContactNumber> = Vec::new();
        'vcardloop: for line in reader {
            let line = match line {
                Ok(x) => x,
                _ => continue 'vcardloop,
            };

            for prop in line.properties {
                if let Some(params) = &prop.params {
                    if prop.name == "FN" {
                        if let Some(x) = &prop.value {
                            contact_name = x.to_string();
                        } else {
                            continue 'vcardloop;
                        }
                    }

                    if prop.name == "TEL" {
                        for entry in params.iter().filter(|x| x.0 == "TYPE") {
                            contact_numbers.push(CoDiContactNumber {
                                phone_type: String::from(&entry.1[0]),
                                number: String::from(
                                    prop.value.clone().unwrap(),
                                ),
                            });
                        }
                    } else {
                        continue 'vcardloop;
                    }
                }
                break 'vcardloop;
            }
        }

        if contact_name.is_empty() && contact_numbers.is_empty()  {
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
